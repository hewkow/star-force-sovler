use num_format::{Locale, ToFormattedString};
use rand::prelude::*;
use rand::rngs::SmallRng;
use rayon::prelude::*;
use std::time::Instant; 

use star_force_sim::starforce::{StarProp, kms_cost, run_single_sim};

fn main() {
    let start = Instant::now();
    let trials: u32 = 10_000_000;
    let target_stars: usize = 22;
    let equiment_level: u32 = 200;
    let enchance_mode: [u8; 7] = [1; 7];

    let mut stars: [StarProp; 30]  = [
        StarProp {stars: 0, cost_multiply: 1.0, success_rate: 0.95, boom_rate: 0.0, enchance_level: None},  // 0
        StarProp {stars: 1, cost_multiply: 1.0, success_rate: 0.9, boom_rate: 0.0, enchance_level: None},   // 1
        StarProp {stars: 2, cost_multiply: 1.0, success_rate: 0.85, boom_rate: 0.0, enchance_level: None},  // 2
        StarProp {stars: 3, cost_multiply: 1.0, success_rate: 0.85, boom_rate: 0.0, enchance_level: None},  // 3
        StarProp {stars: 4, cost_multiply: 1.0, success_rate: 0.80, boom_rate: 0.0, enchance_level: None},  // 4
        StarProp {stars: 5, cost_multiply: 1.0, success_rate: 0.75, boom_rate: 0.0, enchance_level: None},  // 5
        StarProp {stars: 6, cost_multiply: 1.0, success_rate: 0.7, boom_rate: 0.0, enchance_level: None},  // 6
        StarProp {stars: 7, cost_multiply: 1.0, success_rate: 0.65, boom_rate: 0.0, enchance_level: None},  // 7
        StarProp {stars: 8, cost_multiply: 1.0, success_rate: 0.6, boom_rate: 0.0, enchance_level: None},  // 8
        StarProp {stars: 9, cost_multiply: 1.0, success_rate: 0.55, boom_rate: 0.0, enchance_level: None},  // 9
        StarProp {stars: 10, cost_multiply: 1.0, success_rate: 0.5, boom_rate: 0.0, enchance_level: None},  // 10
        StarProp {stars: 11, cost_multiply: 1.0, success_rate: 0.45, boom_rate: 0.0, enchance_level: None},  // 11
        StarProp {stars: 12, cost_multiply: 1.0, success_rate: 0.4, boom_rate: 0.0, enchance_level: None},  // 12
        StarProp {stars: 13, cost_multiply: 1.0, success_rate: 0.35, boom_rate: 0.0, enchance_level: None},  // 13
        StarProp {stars: 14, cost_multiply: 1.0, success_rate: 0.3, boom_rate: 0.0, enchance_level: None},  // 14
        StarProp {stars: 15, cost_multiply: 1.0, success_rate: 0.3, boom_rate: 0.021, enchance_level: Some(enchance_mode[0])},  // 15
        StarProp {stars: 16, cost_multiply: 1.0, success_rate: 0.3, boom_rate: 0.021, enchance_level: Some(enchance_mode[1])},  // 16
        StarProp {stars: 17, cost_multiply: 1.0, success_rate: 0.15, boom_rate: 0.068, enchance_level: Some(enchance_mode[2])},  // 17
        StarProp {stars: 18, cost_multiply: 1.0, success_rate: 0.15, boom_rate: 0.068, enchance_level: Some(enchance_mode[3])},  // 18
        StarProp {stars: 19, cost_multiply: 1.0, success_rate: 0.15, boom_rate: 0.085, enchance_level: Some(enchance_mode[4])},  // 19
        StarProp {stars: 20, cost_multiply: 1.0, success_rate: 0.3, boom_rate: 0.105, enchance_level: Some(enchance_mode[5])},  // 20
        StarProp {stars: 21, cost_multiply: 1.0, success_rate: 0.15, boom_rate: 0.1275, enchance_level: Some(enchance_mode[6])},  // 21
        StarProp {stars: 22, cost_multiply: 1.0, success_rate: 0.15, boom_rate: 0.17, enchance_level: None},  // 22
        StarProp {stars: 23, cost_multiply: 1.0, success_rate: 0.10, boom_rate: 0.18, enchance_level: None},  // 23
        StarProp {stars: 24, cost_multiply: 1.0, success_rate: 0.10, boom_rate: 0.18, enchance_level: None},  // 24
        StarProp {stars: 25, cost_multiply: 1.0, success_rate: 0.10, boom_rate: 0.18, enchance_level: None},  // 25
        StarProp {stars: 26, cost_multiply: 1.0, success_rate: 0.07, boom_rate: 0.18, enchance_level: None},  // 26
        StarProp {stars: 27, cost_multiply: 1.0, success_rate: 0.05, boom_rate: 0.186, enchance_level: None},  // 27
        StarProp {stars: 28, cost_multiply: 1.0, success_rate: 0.03, boom_rate: 0.19, enchance_level: None},  // 28
        StarProp {stars: 29, cost_multiply: 1.0, success_rate: 0.01, boom_rate: 0.194, enchance_level: None},  // 29

    ];

    stars[15].enchance_mode_apply();
    stars[16].enchance_mode_apply();
    stars[17].enchance_mode_apply();
    stars[18].enchance_mode_apply();
    stars[19].enchance_mode_apply();
    stars[20].enchance_mode_apply();
    stars[21].enchance_mode_apply();

    
    let mut boom_thresholds = [0u32; 30];
    let mut success_thresholds = [0u32; 30];
    for i in 0..30 {
        boom_thresholds[i] = (stars[i].boom_rate as f64 * 4294967296.0).round() as u32;
        success_thresholds[i] = ((stars[i].boom_rate as f64 + stars[i].success_rate as f64) * 4294967296.0).round() as u32;
    }

    let mut cost_lookup = [0u64; 30];
    for i in 0..30 {
        cost_lookup[i] = (kms_cost(i as u32, equiment_level) as f32 * stars[i].cost_multiply).round() as u64 ;
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
mod tests;