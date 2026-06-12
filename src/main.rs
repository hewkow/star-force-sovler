use num_format::{Locale, ToFormattedString};
use rand::prelude::*;
use rand::rngs::SmallRng;
use rayon::prelude::*;
use std::time::Instant; 

use star_force_sim::starforce::{EnchancementMode, EnchanceConfig, StarProp, kms_cost, run_single_sim};

fn main() {
    let start = Instant::now();
    let trials: u32 = 10_000_000;
    let target_stars: usize = 22;
    let equiment_level: u32 = 200;

    let sim_config = EnchanceConfig {
        mode_15_21: [EnchancementMode::Level1; 7],
        star_catch: true,
        ssf_event: true,
        safeguard: true,
    };

    let stars: [StarProp; 30] = core::array::from_fn(|i| {
        StarProp::new(i as u8, &sim_config)
    });

    
    let mut boom_thresholds = [0u32; 30];
    let mut success_thresholds = [0u32; 30];
    for i in 0..30 {
        boom_thresholds[i] = (stars[i].boom_rate  * 4294967296.0).round() as u32;
        success_thresholds[i] = ((stars[i].boom_rate + stars[i].success_rate) * 4294967296.0).round() as u32;
    }

    let mut cost_lookup = [0u64; 30];
    for i in 0..30 {
        cost_lookup[i] = (kms_cost(i as u32, equiment_level) as f64 * stars[i].cost_multiply).round() as u64 ;
    }

    let (total_boom, total_cost) = (0..trials)
        .into_par_iter()
        .map_init(
            || SmallRng::from_os_rng(),
            |rng, _| {
                run_single_sim(target_stars, rng, &boom_thresholds, &success_thresholds, &cost_lookup)
            },
        )
        .reduce(
            || ( 0u32, 0u64), 
            |acc, current| {
                (
                    acc.0 + current.0,
                    acc.1 + current.1,
                )
            },
        );


    println!("boom avg count : {}", total_boom as f32 / trials as f32);

    let s = (total_cost / trials as u64).to_formatted_string(&Locale::en);
    println!("total avg cost for lv{} = {}", equiment_level , s);
    println!("\nTime elapsed: {:?}", start.elapsed());
}


#[cfg(test)]
mod tests {
    use super::*;


    fn run_test_sim(sim_config: &EnchanceConfig, equipment_level: u32, target_stars: usize, trials: u32) -> (f32, u64) {
        let stars: [StarProp; 30] = core::array::from_fn(|i| {
            StarProp::new(i as u8, sim_config)
        });

        let mut boom_thresholds = [0u32; 30];
        let mut success_thresholds = [0u32; 30];
        for i in 0..30 {
            boom_thresholds[i] = (stars[i].boom_rate * 4294967296.0).round() as u32;
            success_thresholds[i] = ((stars[i].boom_rate + stars[i].success_rate) * 4294967296.0).round() as u32;
        }

        let mut cost_lookup = [0u64; 30];
        for i in 0..30 {
            cost_lookup[i] = (kms_cost(i as u32, equipment_level) as f64 * stars[i].cost_multiply).round() as u64;
        }

        let (total_boom, total_cost) = (0..trials)
            .into_par_iter()
            .map_init(
                || SmallRng::from_os_rng(),
                |rng, _| {
                    run_single_sim(target_stars, rng, &boom_thresholds, &success_thresholds, &cost_lookup)
                },
            )
            .reduce(
                || (0u32, 0u64),
                |acc, current| (acc.0 + current.0, acc.1 + current.1),
            );

        (total_boom as f32 / trials as f32, total_cost / trials as u64)
    }

    fn assert_within_tolerance(actual: f32, expected: f32, tolerance_pct: f32, metric: &str) {
        if expected == 0.0 {
            assert_eq!(actual, 0.0, "{} value {} must be exactly 0, but a boom occurred.", metric, actual);
            return;
        }
        
        let diff = (actual - expected).abs();
        let allowed = expected * tolerance_pct;
        assert!(
            diff <= allowed,
            "{} value {} deviated from expected {} by more than {}%",
            metric, actual, expected, tolerance_pct * 100.0
        );
    }

    // Note: Adjusted down from 1_000_000 to prevent CI timeouts (see critique below).
    // Adjust this back up if running locally and you need strict precision.
    const TEST_TRIALS: u32 = 1_000_000; 
    const TOLERANCE: f32 = 0.05;

    struct MatrixEntry {
        level: u32,
        target: usize,
        mode: EnchancementMode,
        star_catch: bool,
        ssf_event: bool,
        safeguard: bool,
        expected_boom: f32,
        expected_cost: f32,
    }

    #[test]
    fn test_comprehensive_config_matrix() {
        use EnchancementMode::*;
        let matrix = [
            // Standard
            MatrixEntry { level: 160, target: 18, mode: Standard, star_catch: false, ssf_event: false, safeguard: false, expected_boom: 0.66, expected_cost: 1.70e9 },
            MatrixEntry { level: 200, target: 18, mode: Standard, star_catch: false, ssf_event: false, safeguard: false, expected_boom: 0.66, expected_cost: 3.30e9 },
            MatrixEntry { level: 200, target: 22, mode: Standard, star_catch: false, ssf_event: false, safeguard: false, expected_boom: 8.32, expected_cost: 3.70e10 },
            MatrixEntry { level: 200, target: 22, mode: Standard, star_catch: true,  ssf_event: false, safeguard: false, expected_boom: 7.40, expected_cost: 3.29e10 },
            MatrixEntry { level: 200, target: 22, mode: Standard, star_catch: false, ssf_event: true,  safeguard: false, expected_boom: 4.22, expected_cost: 1.79e10 },
            MatrixEntry { level: 200, target: 22, mode: Standard, star_catch: false, ssf_event: false, safeguard: true,  expected_boom: 4.68, expected_cost: 4.70e10 },
            MatrixEntry { level: 200, target: 22, mode: Standard, star_catch: true,  ssf_event: true,  safeguard: false, expected_boom: 3.80, expected_cost: 1.62e10 },
            MatrixEntry { level: 200, target: 22, mode: Standard, star_catch: true,  ssf_event: false, safeguard: true,  expected_boom: 4.22, expected_cost: 4.19e10 },
            MatrixEntry { level: 200, target: 22, mode: Standard, star_catch: false, ssf_event: true,  safeguard: true,  expected_boom: 2.66, expected_cost: 2.60e10 },
            MatrixEntry { level: 200, target: 22, mode: Standard, star_catch: true,  ssf_event: true,  safeguard: true,  expected_boom: 2.41, expected_cost: 2.35e10 },

            // Level 1
            MatrixEntry { level: 160, target: 18, mode: Level1, star_catch: false, ssf_event: false, safeguard: false, expected_boom: 0.66, expected_cost: 1.70e9 },
            MatrixEntry { level: 200, target: 18, mode: Level1, star_catch: false, ssf_event: false, safeguard: false, expected_boom: 0.66, expected_cost: 3.30e9 },
            MatrixEntry { level: 200, target: 22, mode: Level1, star_catch: false, ssf_event: false, safeguard: false, expected_boom: 8.32, expected_cost: 3.70e10 },
            MatrixEntry { level: 200, target: 22, mode: Level1, star_catch: true,  ssf_event: false, safeguard: false, expected_boom: 7.40, expected_cost: 3.29e10 },
            MatrixEntry { level: 200, target: 22, mode: Level1, star_catch: false, ssf_event: true,  safeguard: false, expected_boom: 4.22, expected_cost: 1.79e10 },
            MatrixEntry { level: 200, target: 22, mode: Level1, star_catch: false, ssf_event: false, safeguard: true,  expected_boom: 4.68, expected_cost: 4.70e10 },
            MatrixEntry { level: 200, target: 22, mode: Level1, star_catch: true,  ssf_event: true,  safeguard: false, expected_boom: 3.80, expected_cost: 1.62e10 },
            MatrixEntry { level: 200, target: 22, mode: Level1, star_catch: true,  ssf_event: false, safeguard: true,  expected_boom: 4.22, expected_cost: 4.19e10 },
            MatrixEntry { level: 200, target: 22, mode: Level1, star_catch: false, ssf_event: true,  safeguard: true,  expected_boom: 2.66, expected_cost: 2.60e10 },
            MatrixEntry { level: 200, target: 22, mode: Level1, star_catch: true,  ssf_event: true,  safeguard: true,  expected_boom: 2.41, expected_cost: 2.35e10 },

            // Level 2
            MatrixEntry { level: 160, target: 18, mode: Level2, star_catch: false, ssf_event: false, safeguard: false, expected_boom: 0.40, expected_cost: 1.92e9 },
            MatrixEntry { level: 200, target: 18, mode: Level2, star_catch: false, ssf_event: false, safeguard: false, expected_boom: 0.40, expected_cost: 3.76e9 },
            MatrixEntry { level: 200, target: 22, mode: Level2, star_catch: false, ssf_event: false, safeguard: false, expected_boom: 5.49, expected_cost: 6.19e10 },
            MatrixEntry { level: 200, target: 22, mode: Level2, star_catch: true,  ssf_event: false, safeguard: false, expected_boom: 4.92, expected_cost: 5.60e10 },
            MatrixEntry { level: 200, target: 22, mode: Level2, star_catch: false, ssf_event: false, safeguard: true,  expected_boom: 3.66, expected_cost: 6.83e10 },
            MatrixEntry { level: 200, target: 22, mode: Level2, star_catch: true,  ssf_event: false, safeguard: true,  expected_boom: 3.35, expected_cost: 6.20e10 },

            // Level 3
            MatrixEntry { level: 160, target: 18, mode: Level3, star_catch: false, ssf_event: false, safeguard: false, expected_boom: 0.16, expected_cost: 2.48e9 },
            MatrixEntry { level: 200, target: 18, mode: Level3, star_catch: false, ssf_event: false, safeguard: false, expected_boom: 0.16, expected_cost: 4.83e9 },
            MatrixEntry { level: 200, target: 22, mode: Level3, star_catch: false, ssf_event: false, safeguard: false, expected_boom: 2.23, expected_cost: 8.72e10 },
            MatrixEntry { level: 200, target: 22, mode: Level3, star_catch: true,  ssf_event: false, safeguard: false, expected_boom: 2.06, expected_cost: 8.08e10 },
            MatrixEntry { level: 200, target: 22, mode: Level3, star_catch: false, ssf_event: false, safeguard: true,  expected_boom: 1.79, expected_cost: 8.83e10 },
            MatrixEntry { level: 200, target: 22, mode: Level3, star_catch: true,  ssf_event: false, safeguard: true,  expected_boom: 1.66, expected_cost: 8.14e10 },

            // Level 4
            MatrixEntry { level: 160, target: 18, mode: Level4, star_catch: false, ssf_event: false, safeguard: false, expected_boom: 0.0, expected_cost: 2.67e9 },
            MatrixEntry { level: 200, target: 18, mode: Level4, star_catch: false, ssf_event: false, safeguard: false, expected_boom: 0.0, expected_cost: 5.22e9 },
            MatrixEntry { level: 200, target: 22, mode: Level4, star_catch: false, ssf_event: false, safeguard: false, expected_boom: 0.0, expected_cost: 1.07e11 },
            MatrixEntry { level: 200, target: 22, mode: Level4, star_catch: true,  ssf_event: false, safeguard: false, expected_boom: 0.0, expected_cost: 1.02e11 },
            MatrixEntry { level: 200, target: 22, mode: Level4, star_catch: false, ssf_event: false, safeguard: true,  expected_boom: 0.0, expected_cost: 1.07e11 },
            MatrixEntry { level: 200, target: 22, mode: Level4, star_catch: true,  ssf_event: false, safeguard: true,  expected_boom: 0.0, expected_cost: 1.02e11 },
        ];

        for (i, case) in matrix.iter().enumerate() {
            let config = EnchanceConfig {
                mode_15_21: [case.mode; 7],
                star_catch: case.star_catch,
                ssf_event: case.ssf_event,
                safeguard: case.safeguard,
            };

            let label = format!(
                "Row {}: Lv{}->{} | Mode:{:?} | Catch:{} SSF:{} Safe:{}",
                i + 1, case.level, case.target, case.mode, case.star_catch, case.ssf_event, case.safeguard
            );

            let (avg_boom, avg_cost) = run_test_sim(&config, case.level, case.target, TEST_TRIALS);

            assert_within_tolerance(avg_boom, case.expected_boom, TOLERANCE, &format!("[{}] Boom", label));
            assert_within_tolerance(avg_cost as f32, case.expected_cost, TOLERANCE, &format!("[{}] Cost", label));
        }
    }
}