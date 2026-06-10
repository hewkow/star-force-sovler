use rand::prelude::*;
use rand::rngs::SmallRng;

pub struct StarProp {
    pub stars: u8,
    pub cost_multiply : f32,
    pub success_rate : f32,
    pub boom_rate : f32,

    pub enchance_level : Option<u8>
}

impl StarProp {
    pub fn enchance_mode_apply(&mut self) {
        let rates = match (self.stars, self.enchance_level) {
            // Stars 15 and 16 share the exact same rules
            (15..=16, Some(1)) => (1.0, 0.30, 0.0210),
            (15..=16, Some(2)) => (1.5, 0.30, 0.0140),
            (15..=16, Some(3)) => (2.5, 0.30, 0.0070),
            (15..=16, Some(4)) => (3.0, 0.30, 0.0000),

            // Star 17
            (17, Some(1))      => (1.0, 0.15, 0.0680),
            (17, Some(2))      => (1.5, 0.15, 0.0425),
            (17, Some(3))      => (2.5, 0.15, 0.0170),
            (17, Some(4))      => (3.0, 0.15, 0.0000),

            // Star 18
            (18, Some(1))      => (1.0, 0.15, 0.0680),
            (18, Some(2))      => (2.0, 0.12, 0.0440),
            (18, Some(3))      => (3.5, 0.10, 0.0180),
            (18, Some(4))      => (6.5, 0.08, 0.0000),

            // Star 19
            (19, Some(1))      => (1.0, 0.15, 0.0850),
            (19, Some(2))      => (2.0, 0.12, 0.0616),
            (19, Some(3))      => (3.5, 0.10, 0.0360),
            (19, Some(4))      => (6.5, 0.08, 0.0000),

            // Star 20
            (20, Some(1))      => (1.0, 0.30, 0.1050),
            (20, Some(2))      => (2.0, 0.25, 0.0750),
            (20, Some(3))      => (3.5, 0.20, 0.0400),
            (20, Some(4))      => (6.5, 0.15, 0.0000),

            // Star 21
            (21, Some(1))      => (1.0, 0.15, 0.1275),
            (21, Some(2))      => (2.0, 0.12, 0.0880),
            (21, Some(3))      => (3.5, 0.10, 0.0450),
            (21, Some(4))      => (6.5, 0.08, 0.0000),

            // Fallback case if inputs don't match any system constraints
            _ => return, 
        };

        // 2. Destructure the assigned tuple directly into the struct fields.
        // This execution line only runs if a valid match arm above was triggered.
        (self.cost_multiply, self.success_rate, self.boom_rate) = rates;
    }
}

pub struct MesoConfig {
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




pub fn run_single_sim(
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
