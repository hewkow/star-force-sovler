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
        mode_15_21: [EnchancementMode::Level4; 7],
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
mod tests;