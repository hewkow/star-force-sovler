#![allow(unsafe_op_in_unsafe_fn)]

use pyo3::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;
use rayon::prelude::*;
use std::collections::{BTreeMap, HashMap};

use starforce_core::starforce::{kms_cost, run_single_sim, EnhanceConfig, EnhancementMode, StarProp, SimMetrics, BIN_SIZE};

// Wrapper class exposed to Python
#[pyclass]
pub struct PySimResult {
    #[pyo3(get)]
    pub total_runs: u32,
    #[pyo3(get)]
    pub total_cost: u128,
    #[pyo3(get)]
    pub total_boom: u64,
    joint_histogram: BTreeMap<u64, [u32;100]>,
    cost_histogram: BTreeMap<u64, u32>,
    session_booms_histogram: [u32; 100],
    per_star_friction: [[u64; 3]; 30],

}

#[pymethods]
impl PySimResult {
    // Converts internal BTreeMap to a column-oriented dict for Polars
    #[getter]
    fn joint_histogram_df(&self) -> HashMap<&'static str, Vec<u64>> {
        let mut cost_bin = Vec::new();
        let mut booms = Vec::new();
        let mut count = Vec::new();

        // Iterate through the BTreeMap
        for (&bin, boom_array) in &self.joint_histogram {
            let actual_cost = bin * BIN_SIZE;
            
            // Iterate through the boom array
            for (b_idx, &cnt) in boom_array.iter().enumerate() {
                if cnt > 0 { // Only export data where runs actually happened
                    cost_bin.push(actual_cost);
                    booms.push(b_idx as u64);
                    count.push(cnt as u64);
                }
            }
        }

        let mut map = HashMap::new();
        map.insert("cost", cost_bin);
        map.insert("booms", booms);
        map.insert("count", count);
        map
    }

    #[getter]
    fn cost_histogram_df(&self) -> HashMap<&'static str, Vec<u64>> {
        let mut cost_bin = Vec::with_capacity(self.cost_histogram.len());
        let mut count = Vec::with_capacity(self.cost_histogram.len());
        for (&bin, &cnt) in &self.cost_histogram {
            cost_bin.push(bin * BIN_SIZE);
            count.push(cnt as u64);
        }
        let mut map = HashMap::new();
        map.insert("cost_bin_start", cost_bin);
        map.insert("count", count);
        map
    }

    // Converts array data to a column-oriented dict for Polars
    #[getter]
    fn session_booms_df(&self) -> HashMap<&'static str, Vec<u32>> {
        let mut booms = Vec::with_capacity(100);
        let mut count = Vec::with_capacity(100);
        for (idx, &cnt) in self.session_booms_histogram.iter().enumerate() {
            if cnt > 0 { // Omit trailing zeros to save memory
                booms.push(idx as u32);
                count.push(cnt);
            }
        }
        let mut map = HashMap::new();
        map.insert("booms", booms);
        map.insert("count", count);
        map
    }

    // Converts nested matrix data into separate series for Polars
    #[getter]
    fn per_star_friction_df(&self) -> HashMap<&'static str, Vec<u64>> {
        let mut star = Vec::with_capacity(30);
        let mut cost_spent = Vec::with_capacity(30);
        let mut booms_triggered = Vec::with_capacity(30);
        let mut attempts_made = Vec::with_capacity(30);

        for i in 0..30 {
            if self.per_star_friction[i][2] > 0 { // Only export active star levels
                star.push(i as u64);
                cost_spent.push(self.per_star_friction[i][0]);
                booms_triggered.push(self.per_star_friction[i][1]);
                attempts_made.push(self.per_star_friction[i][2]);
            }
        }

        let mut map = HashMap::new();
        map.insert("star", star);
        map.insert("cost_spent", cost_spent);
        map.insert("booms_triggered", booms_triggered);
        map.insert("attempts_made", attempts_made);
        map
    }
}

fn parse_mode(mode: &str) -> PyResult<EnhancementMode> {
    match mode {
        "Standard" => Ok(EnhancementMode::Standard),
        "Level1" => Ok(EnhancementMode::Level1),
        "Level2" => Ok(EnhancementMode::Level2),
        "Level3" => Ok(EnhancementMode::Level3),
        "Level4" => Ok(EnhancementMode::Level4),
        _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Invalid EnhancementMode: {}",
            mode
        ))),
    }
}

#[pyfunction]
#[pyo3(signature = (trials, start_stars, target_stars, equipment_level, mode_15_21, star_catch=true, ssf_boom_reduce_event=false, ssf_cost_reduce_event=false, safeguard=false))]
fn simulate(
    py: Python<'_>,
    trials: u32,
    start_stars: usize,
    target_stars: usize,
    equipment_level: u32,
    mode_15_21: Vec<String>,
    star_catch: bool,
    ssf_boom_reduce_event: bool,
    ssf_cost_reduce_event: bool,
    safeguard: bool,
) -> PyResult<PySimResult> {
    if mode_15_21.len() != 7 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "mode_15_21 must contain exactly 7 modes",
        ));
    }

    let mut modes = [EnhancementMode::Standard; 7];
    for (i, mode_str) in mode_15_21.iter().enumerate() {
        modes[i] = parse_mode(mode_str)?;
    }

    let sim_config = EnhanceConfig {
        mode_15_21: modes,
        star_catch,
        ssf_boom_reduce_event,
        ssf_cost_reduce_event,
        safeguard,
    };

    let stars: [StarProp; 30] = core::array::from_fn(|i| StarProp::new(i as u8, &sim_config));

    let mut boom_thresholds = [0u32; 30];
    let mut success_thresholds = [0u32; 30];
    for i in 0..30 {
        boom_thresholds[i] = (stars[i].boom_rate * 4294967296.0).round() as u32;
        success_thresholds[i] =
            ((stars[i].boom_rate + stars[i].success_rate) * 4294967296.0).round() as u32;
    }

    let mut cost_lookup = [0u64; 30];
    for i in 0..30 {
        cost_lookup[i] =
            (kms_cost(i as u32, equipment_level) as f64 * stars[i].cost_multiply).round() as u64;
    }

    let final_metrics = py.allow_threads(|| {
        (0..trials) 
        .into_par_iter()
        .map_init(
            || SmallRng::from_os_rng(),
            |rng, _| run_single_sim(start_stars, target_stars, rng, &boom_thresholds, &success_thresholds, &cost_lookup)
        )
        .fold(
            || SimMetrics::default(),
            |mut acc, run_result| {
                acc.add_run(run_result);
                acc
            }
        )
        .reduce(
            || SimMetrics::default(),
            |acc1, acc2| acc1.merge(acc2)
        )
    });

    Ok(PySimResult {
        total_runs: final_metrics.total_runs,
        total_cost: final_metrics.total_cost,
        total_boom: final_metrics.total_boom,
        cost_histogram: final_metrics.cost_histogram,
        session_booms_histogram: *final_metrics.session_booms_histogram,
        per_star_friction: *final_metrics.per_star_friction,
        joint_histogram: final_metrics.joint_histogram,
    })
}

#[pymodule]
fn star_force_sim_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(simulate, m)?)?;
    m.add_class::<PySimResult>()?;
    Ok(())
}