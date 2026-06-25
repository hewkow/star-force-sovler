use num_format::{Locale, ToFormattedString};
use rand::prelude::*;
use rand::rngs::SmallRng;
use rayon::prelude::*;
use std::time::Instant;

use starforce_core::starforce::{EnhancementMode, EnhanceConfig, StarProp, kms_cost, run_single_sim, SimMetrics};

fn main() {
    let start = Instant::now();
    let trials: u32 = 10_000_000;
    let start_stars: usize = 0;
    let target_stars: usize = 22;
    let equiment_level: u32 = 200;

    let sim_config = EnhanceConfig {
        mode_15_21: [EnhancementMode::Level1; 7],
        star_catch: true,
        ssf_boom_reduce_event: true,
        ssf_cost_reduce_event: true,
        safeguard: false,
        ..Default::default()
    };

    let stars: [StarProp; 30] = core::array::from_fn(|i| StarProp::new(i as u8, &sim_config));
    
    let mut boom_thresholds = [0u32; 30];
    let mut success_thresholds = [0u32; 30];
    let mut cost_lookup = [0u64; 30];
    
    for i in 0..30 {
        boom_thresholds[i] = (stars[i].boom_rate * 4294967296.0).round() as u32;
        success_thresholds[i] = ((stars[i].boom_rate + stars[i].success_rate) * 4294967296.0).round() as u32;
        cost_lookup[i] = (kms_cost(i as u32, equiment_level) as f64 * stars[i].cost_multiply).round() as u64;
    }

    let final_metrics = (0..trials)
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
        );
    

    
    println!("boom avg count : {}", final_metrics.total_boom as f32 / trials as f32);

    let s = (final_metrics.total_cost / trials as u128).to_formatted_string(&Locale::en);
    println!("total avg cost for lv{} = {}", equiment_level , s);

    println!("Total runs: {}", final_metrics.total_runs.to_formatted_string(&Locale::en));
    println!("Time elapsed: {:?}", start.elapsed());
}


#[cfg(test)]
mod tests {
    use super::*;

    fn run_test_sim(sim_config: &EnhanceConfig, equipment_level: u32, start_stars: usize, target_stars: usize, trials: u32) -> (f32, u64) {
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

        let final_metrics = (0..trials)
            .into_par_iter()
            .map_init(
                || SmallRng::from_os_rng(),
                |rng, _| run_single_sim(start_stars ,target_stars, rng, &boom_thresholds, &success_thresholds, &cost_lookup)
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
            );

        (final_metrics.total_boom as f32 / trials as f32, (final_metrics.total_cost / trials as u128) as u64)
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

    const TEST_TRIALS: u32 = 1_000_000; 
    const TOLERANCE: f32 = 0.05;

    struct MatrixEntry {
        level: u32,
        start_stars: usize,
        target: usize,
        mode: EnhancementMode,
        star_catch: bool,
        ssf_boom_reduce_event: bool,
        ssf_cost_reduce_event: bool,
        safeguard: bool,
        expected_boom: f32,
        expected_cost: f32,
    }

    #[test]
    fn test_comprehensive_config_matrix() {
        use EnhancementMode::*;
        let matrix = [
            // Standard
            MatrixEntry { level: 160, start_stars: 0, target: 18, mode: Standard, star_catch: false, ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: false, expected_boom: 0.66, expected_cost: 1.70e9 },
            MatrixEntry { level: 200, start_stars: 0, target: 18, mode: Standard, star_catch: false, ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: false, expected_boom: 0.66, expected_cost: 3.30e9 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Standard, star_catch: false, ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: false, expected_boom: 8.32, expected_cost: 3.70e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Standard, star_catch: true,  ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: false, expected_boom: 7.40, expected_cost: 3.29e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Standard, star_catch: false, ssf_boom_reduce_event: true,  ssf_cost_reduce_event: true,  safeguard: false, expected_boom: 4.22, expected_cost: 1.79e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Standard, star_catch: false, ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: true,  expected_boom: 4.68, expected_cost: 4.70e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Standard, star_catch: true,  ssf_boom_reduce_event: true,  ssf_cost_reduce_event: true,  safeguard: false, expected_boom: 3.80, expected_cost: 1.62e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Standard, star_catch: true,  ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: true,  expected_boom: 4.22, expected_cost: 4.19e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Standard, star_catch: false, ssf_boom_reduce_event: true,  ssf_cost_reduce_event: true,  safeguard: true,  expected_boom: 2.66, expected_cost: 2.60e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Standard, star_catch: true,  ssf_boom_reduce_event: true,  ssf_cost_reduce_event: true,  safeguard: true,  expected_boom: 2.41, expected_cost: 2.35e10 },

            // Level 1
            MatrixEntry { level: 160, start_stars: 0, target: 18, mode: Level1, star_catch: false, ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: false, expected_boom: 0.66, expected_cost: 1.70e9 },
            MatrixEntry { level: 200, start_stars: 0, target: 18, mode: Level1, star_catch: false, ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: false, expected_boom: 0.66, expected_cost: 3.30e9 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Level1, star_catch: false, ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: false, expected_boom: 8.32, expected_cost: 3.70e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Level1, star_catch: true,  ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: false, expected_boom: 7.40, expected_cost: 3.29e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Level1, star_catch: false, ssf_boom_reduce_event: true,  ssf_cost_reduce_event: true,  safeguard: false, expected_boom: 4.22, expected_cost: 1.79e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Level1, star_catch: false, ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: true,  expected_boom: 4.68, expected_cost: 4.70e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Level1, star_catch: true,  ssf_boom_reduce_event: true,  ssf_cost_reduce_event: true,  safeguard: false, expected_boom: 3.80, expected_cost: 1.62e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Level1, star_catch: true,  ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: true,  expected_boom: 4.22, expected_cost: 4.19e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Level1, star_catch: false, ssf_boom_reduce_event: true,  ssf_cost_reduce_event: true,  safeguard: true,  expected_boom: 2.66, expected_cost: 2.60e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Level1, star_catch: true,  ssf_boom_reduce_event: true,  ssf_cost_reduce_event: true,  safeguard: true,  expected_boom: 2.41, expected_cost: 2.35e10 },

            // Level 2
            MatrixEntry { level: 160, start_stars: 0, target: 18, mode: Level2, star_catch: false, ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: false, expected_boom: 0.40, expected_cost: 1.92e9 },
            MatrixEntry { level: 200, start_stars: 0, target: 18, mode: Level2, star_catch: false, ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: false, expected_boom: 0.40, expected_cost: 3.76e9 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Level2, star_catch: false, ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: false, expected_boom: 5.49, expected_cost: 6.19e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Level2, star_catch: true,  ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: false, expected_boom: 4.92, expected_cost: 5.60e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Level2, star_catch: false, ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: true,  expected_boom: 3.66, expected_cost: 6.83e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Level2, star_catch: true,  ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: true,  expected_boom: 3.35, expected_cost: 6.20e10 },

            // Level 3
            MatrixEntry { level: 160, start_stars: 0, target: 18, mode: Level3, star_catch: false, ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: false, expected_boom: 0.16, expected_cost: 2.48e9 },
            MatrixEntry { level: 200, start_stars: 0, target: 18, mode: Level3, star_catch: false, ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: false, expected_boom: 0.16, expected_cost: 4.83e9 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Level3, star_catch: false, ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: false, expected_boom: 2.23, expected_cost: 8.72e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Level3, star_catch: true,  ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: false, expected_boom: 2.06, expected_cost: 8.08e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Level3, star_catch: false, ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: true,  expected_boom: 1.79, expected_cost: 8.83e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Level3, star_catch: true,  ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: true,  expected_boom: 1.66, expected_cost: 8.14e10 },

            // Level 4
            MatrixEntry { level: 160, start_stars: 0, target: 18, mode: Level4, star_catch: false, ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: false, expected_boom: 0.0, expected_cost: 2.67e9 },
            MatrixEntry { level: 200, start_stars: 0, target: 18, mode: Level4, star_catch: false, ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: false, expected_boom: 0.0, expected_cost: 5.22e9 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Level4, star_catch: false, ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: false, expected_boom: 0.0, expected_cost: 1.07e11 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Level4, star_catch: true,  ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: false, expected_boom: 0.0, expected_cost: 1.02e11 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Level4, star_catch: false, ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: true,  expected_boom: 0.0, expected_cost: 1.07e11 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Level4, star_catch: true,  ssf_boom_reduce_event: false, ssf_cost_reduce_event: false, safeguard: true,  expected_boom: 0.0, expected_cost: 1.02e11 },

            // Past 22 stars
            MatrixEntry { level: 200, start_stars: 0, target: 23, mode: Standard, star_catch: true, ssf_boom_reduce_event: true, ssf_cost_reduce_event: true, safeguard: false, expected_boom: 8.85, expected_cost: 3.336e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 23, mode: Standard, star_catch: true, ssf_boom_reduce_event: true, ssf_cost_reduce_event: true, safeguard: true,  expected_boom: 6.12, expected_cost: 4.769e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 25, mode: Standard, star_catch: true, ssf_boom_reduce_event: true, ssf_cost_reduce_event: true, safeguard: false, expected_boom: 66.0, expected_cost: 2.25e11 },
            MatrixEntry { level: 200, start_stars: 0, target: 25, mode: Standard, star_catch: true, ssf_boom_reduce_event: true, ssf_cost_reduce_event: true, safeguard: true,  expected_boom: 49.22, expected_cost: 3.09e11 },

            // SSF with enhancement mode
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Level2, star_catch: true, ssf_boom_reduce_event: true, ssf_cost_reduce_event: true, safeguard: true, expected_boom: 1.96, expected_cost: 3.566e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Level3, star_catch: true, ssf_boom_reduce_event: true, ssf_cost_reduce_event: true, safeguard: true, expected_boom: 1.04, expected_cost: 5.011e10 },
            MatrixEntry { level: 200, start_stars: 0, target: 22, mode: Level4, star_catch: true, ssf_boom_reduce_event: true, ssf_cost_reduce_event: true, safeguard: true, expected_boom: 0.00, expected_cost: 7.199e10 },

        ];

        for (i, case) in matrix.iter().enumerate() {
            let config = EnhanceConfig {
                mode_15_21: [case.mode; 7],
                star_catch: case.star_catch,
                ssf_boom_reduce_event: case.ssf_boom_reduce_event,
                ssf_cost_reduce_event: case.ssf_cost_reduce_event,
                safeguard: case.safeguard,
            };

            let label = format!(
                "Row {}: Lv{} | {}->{} | Mode:{:?} | Catch:{} SSF_Boom:{} SSF_Cost:{} Safe:{}",
                i + 1, case.level, case.start_stars, case.target, case.mode, case.star_catch, case.ssf_boom_reduce_event, case.ssf_cost_reduce_event, case.safeguard
            );

            let (avg_boom, avg_cost) = run_test_sim(&config, case.level, case.start_stars, case.target, TEST_TRIALS);

            assert_within_tolerance(avg_boom, case.expected_boom, TOLERANCE, &format!("[{}] Boom", label));
            assert_within_tolerance(avg_cost as f32, case.expected_cost, TOLERANCE, &format!("[{}] Cost", label));
        }
    }
}