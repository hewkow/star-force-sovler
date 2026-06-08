use num_format::{Locale, ToFormattedString};
use rand::prelude::*;
use rand::rngs::SmallRng;
use rayon::prelude::*;
use std::time::Instant; 

fn main() {
    let start = Instant::now();
    let trials: u32 = 10_000_000;
    let target_stars: usize = 23;
    let equiment_level: u32 = 200;

    let mut star_prob: [[f32; 3]; 30] = [
        [0.95, 0.05, 0.0],      // 0
        [0.9, 0.1, 0.0],        // 1
        [0.85, 0.15, 0.0],      // 2
        [0.85, 0.15, 0.0],      // 3
        [0.80, 0.2, 0.0],       // 4
        [0.75, 0.25, 0.0],      // 5
        [0.7, 0.3, 0.0],        // 6
        [0.65, 0.35, 0.0],      // 7
        [0.6, 0.4, 0.0],        // 8
        [0.55, 0.45, 0.0],      // 9
        [0.5, 0.5, 0.0],        // 10
        [0.45, 0.55, 0.0],      // 11
        [0.4, 0.6, 0.0],        // 12
        [0.35, 0.65, 0.0],      // 13
        [0.3, 0.7, 0.0],        // 14
        [0.3, 0.679, 0.021],    // 15
        [0.3, 0.679, 0.021],    // 16
        [0.15, 0.782, 0.068],   // 17
        [0.15, 0.782, 0.068],   // 18
        [0.15, 0.765, 0.085],   // 19
        [0.3, 0.595, 0.105],    // 20
        [0.15, 0.7225, 0.1275], // 21
        [0.15, 0.68, 0.17],     // 22
        [0.10, 0.72, 0.18],     // 23
        [0.10, 0.72, 0.18],     // 24
        [0.10, 0.72, 0.18],     // 25
        [0.07, 0.744, 0.186],   // 26
        [0.05, 0.76, 0.19],     // 27
        [0.03, 0.776, 0.194],   // 28
        [0.01, 0.792, 0.198],   // 29
    ];

    let mut boom_thresholds = [0u32; 30];
    let mut success_thresholds = [0u32; 30];
    for i in 0..30 {
        boom_thresholds[i] = (star_prob[i][2] as f64 * 4294967296.0).round() as u32;
        success_thresholds[i] = ((star_prob[i][2] as f64 + star_prob[i][0] as f64) * 4294967296.0).round() as u32;
    }

    let mut cost_lookup = [0u64; 30];
    for i in 0..30 {
        cost_lookup[i] = kms_cost(i as u32, equiment_level);
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

struct MesoConfig {
    divisor: f64,
    current_star_exp: f64,
    extra_mult: f64,
}

pub fn kms_cost(current_star: u32, item_level: u32) -> u64 {
    let config = match current_star {
        11 => MesoConfig {
            divisor: 22000.0,
            current_star_exp: 2.7,
            extra_mult: 1.0,
        },
        12 => MesoConfig {
            divisor: 15000.0,
            current_star_exp: 2.7,
            extra_mult: 1.0,
        },
        13 => MesoConfig {
            divisor: 11000.0,
            current_star_exp: 2.7,
            extra_mult: 1.0,
        },
        14 => MesoConfig {
            divisor: 7500.0,
            current_star_exp: 2.7,
            extra_mult: 1.0,
        },
        17 => MesoConfig {
            divisor: 20000.0,
            current_star_exp: 2.7,
            extra_mult: 4.0 / 3.0,
        },
        18 => MesoConfig {
            divisor: 20000.0,
            current_star_exp: 2.7,
            extra_mult: 20.0 / 7.0,
        },
        19 => MesoConfig {
            divisor: 20000.0,
            current_star_exp: 2.7,
            extra_mult: 40.0 / 9.0,
        },
        21 => MesoConfig {
            divisor: 20000.0,
            current_star_exp: 2.7,
            extra_mult: 8.0 / 5.0,
        },
        15.. => MesoConfig {
            divisor: 20000.0,
            current_star_exp: 2.7,
            extra_mult: 1.0,
        },
        10.. => MesoConfig {
            divisor: 40000.0,
            current_star_exp: 2.7,
            extra_mult: 1.0,
        },
        _ => MesoConfig {
            divisor: 2500.0,
            current_star_exp: 1.0,
            extra_mult: 1.0,
        },
    };

    let level_factor = ((item_level / 10) * 10) as f64;
    let star_factor = (current_star + 1) as f64;

    let base_calc =
        (config.extra_mult * level_factor.powi(3) * star_factor.powf(config.current_star_exp))
            / config.divisor;

    100 * ((base_calc + 10.0).round() as u64)
}

fn run_single_sim(
    target_star: usize,
    rng: &mut SmallRng,
    boom_thresholds: &[u32; 30],
    success_thresholds: &[u32; 30],
    cost_lookup: &[u64; 30],
) -> (u32, u64) {
    let mut current_star: usize = 0;
    let mut boom_count: u32 = 0;
    let mut sim_cost: u64 = 0;

    while current_star < target_star && current_star < 30 {
        sim_cost += cost_lookup[current_star];

        let val: u32 = rng.random();
        
        if val < boom_thresholds[current_star] {
                    boom_count += 1;
                    current_star = match current_star {
                        26.. => 20,
                        23..=25 => 19,
                        21..=22 => 17,
                        20 => 15,
                        _ => 12,
                    };
                } else if val < success_thresholds[current_star] {
                    current_star += 1;
                }
                // Fall into no change
            }
    (boom_count, sim_cost)
}
