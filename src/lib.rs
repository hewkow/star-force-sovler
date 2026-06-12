pub mod starforce;

use pyo3::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;
use rayon::prelude::*;

use crate::starforce::{kms_cost, run_single_sim, EnchanceConfig, EnchancementMode, StarProp};

fn parse_mode(mode: &str) -> PyResult<EnchancementMode> {
    match mode {
        "Standard" => Ok(EnchancementMode::Standard),
        "Level1" => Ok(EnchancementMode::Level1),
        "Level2" => Ok(EnchancementMode::Level2),
        "Level3" => Ok(EnchancementMode::Level3),
        "Level4" => Ok(EnchancementMode::Level4),
        _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Invalid EnchancementMode: {}",
            mode
        ))),
    }
}

#[pyfunction]
#[pyo3(signature = (trials, target_stars, equipment_level, mode_15_21, star_catch=true, ssf_event=false, safeguard=false))]
fn simulate(
    py: Python<'_>,
    trials: u32,
    target_stars: usize,
    equipment_level: u32,
    mode_15_21: Vec<String>,
    star_catch: bool,
    ssf_event: bool,
    safeguard: bool,
) -> PyResult<(f64, f64)> {
    if mode_15_21.len() != 7 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "mode_15_21 must contain exactly 7 modes",
        ));
    }

    let mut modes = [EnchancementMode::Standard; 7];
    for (i, mode_str) in mode_15_21.iter().enumerate() {
        modes[i] = parse_mode(mode_str)?;
    }

    let sim_config = EnchanceConfig {
        mode_15_21: modes,
        star_catch,
        ssf_event,
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

    // py.allow_threads releases the GIL so the Rayon worker pool can maximize CPU usage unhindered.
    let (total_boom, total_cost) = py.allow_threads(|| {
        (0..trials)
            .into_par_iter()
            .map_init(
                || SmallRng::from_os_rng(),
                |rng, _| {
                    run_single_sim(
                        target_stars,
                        rng,
                        &boom_thresholds,
                        &success_thresholds,
                        &cost_lookup,
                    )
                },
            )
            .reduce(
                || (0u32, 0u64),
                |acc, current| (acc.0 + current.0, acc.1 + current.1),
            )
    });

    Ok((
        total_boom as f64 / trials as f64,
        total_cost as f64 / trials as f64,
    ))
}

#[pymodule]
fn star_force_sim(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(simulate, m)?)?;
    Ok(())
}