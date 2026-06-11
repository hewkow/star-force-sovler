use rand::prelude::*;
use rand::rngs::SmallRng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnchancementMode {
    Standard,
    Level1,
    Level2,
    Level3,
    Level4,
}

pub struct EnchanceConfig {
    pub mode_15_21 : [EnchancementMode; 7],
}

pub struct StarProp {
    pub stars: u8,
    pub cost_multiply : f64,
    pub success_rate : f64,
    pub boom_rate : f64,

    pub enchance_level : EnchancementMode,
}

impl StarProp {
    pub fn new(stars : u8, config: &EnchanceConfig) -> Self{
        let mode = match stars {
            15..22 => config.mode_15_21[(stars - 15) as usize],
            _ => EnchancementMode::Standard
        };

        let (mut cost_mult, mut success, mut boom) = Self::get_base_rates(stars, mode);

        Self {
            stars,
            cost_multiply: cost_mult,
            success_rate: success,
            boom_rate: boom,
            enchance_level: mode,
        }
    }

    pub fn get_base_rates( stars : u8, mode: EnchancementMode) -> (f64, f64, f64) {
        match (stars, mode) {
    // Your custom enhancement rules for 15-21
                (15..=16, EnchancementMode::Level1) => (1.0, 0.30, 0.0210),
                (15..=16, EnchancementMode::Level2) => (1.5, 0.30, 0.0140),
                (15..=16, EnchancementMode::Level3) => (2.5, 0.30, 0.0070),
                (15..=16, EnchancementMode::Level4) => (3.0, 0.30, 0.0000),
    
                (17, EnchancementMode::Level1)      => (1.0, 0.15, 0.0680),
                (17, EnchancementMode::Level2)      => (1.5, 0.15, 0.0425),
                (17, EnchancementMode::Level3)      => (2.5, 0.15, 0.0170),
                (17, EnchancementMode::Level4)      => (3.0, 0.15, 0.0000),
    
                (18, EnchancementMode::Level1)      => (1.0, 0.15, 0.0680),
                (18, EnchancementMode::Level2)      => (2.0, 0.12, 0.0440),
                (18, EnchancementMode::Level3)      => (3.5, 0.10, 0.0180),
                (18, EnchancementMode::Level4)      => (6.5, 0.08, 0.0000),
    
                (19, EnchancementMode::Level1)      => (1.0, 0.15, 0.0850),
                (19, EnchancementMode::Level2)      => (2.0, 0.12, 0.0616),
                (19, EnchancementMode::Level3)      => (3.5, 0.10, 0.0360),
                (19, EnchancementMode::Level4)      => (6.5, 0.08, 0.0000),
    
                (20, EnchancementMode::Level1)      => (1.0, 0.30, 0.1050),
                (20, EnchancementMode::Level2)      => (2.0, 0.25, 0.0750),
                (20, EnchancementMode::Level3)      => (3.5, 0.20, 0.0400),
                (20, EnchancementMode::Level4)      => (6.5, 0.15, 0.0000),
    
                (21, EnchancementMode::Level1)      => (1.0, 0.15, 0.1275),
                (21, EnchancementMode::Level2)      => (2.0, 0.12, 0.0880),
                (21, EnchancementMode::Level3)      => (3.5, 0.10, 0.0450),
                (21, EnchancementMode::Level4)      => (6.5, 0.08, 0.0000),
    
                // Standard fallback rates mapped from your original array
                (0, _)  => (1.0, 0.95, 0.0),
                (1, _)  => (1.0, 0.90, 0.0),
                (2, _)  => (1.0, 0.85, 0.0),
                (3, _)  => (1.0, 0.85, 0.0),
                (4, _)  => (1.0, 0.80, 0.0),
                (5, _)  => (1.0, 0.75, 0.0),
                (6, _)  => (1.0, 0.70, 0.0),
                (7, _)  => (1.0, 0.65, 0.0),
                (8, _)  => (1.0, 0.60, 0.0),
                (9, _)  => (1.0, 0.55, 0.0),
                (10, _) => (1.0, 0.50, 0.0),
                (11, _) => (1.0, 0.45, 0.0),
                (12, _) => (1.0, 0.40, 0.0),
                (13, _) => (1.0, 0.35, 0.0),
                (14, _) => (1.0, 0.30, 0.0),
                (15, _) => (1.0, 0.30, 0.021),
                (16, _) => (1.0, 0.30, 0.021),
                (17, _) => (1.0, 0.15, 0.068),
                (18, _) => (1.0, 0.15, 0.068),
                (19, _) => (1.0, 0.15, 0.085),
                (20, _) => (1.0, 0.30, 0.105),
                (21, _) => (1.0, 0.15, 0.1275),
                (22, _) => (1.0, 0.15, 0.17),
                (23, _) => (1.0, 0.10, 0.18),
                (24, _) => (1.0, 0.10, 0.18),
                (25, _) => (1.0, 0.10, 0.18),
                (26, _) => (1.0, 0.07, 0.186),
                (27, _) => (1.0, 0.05, 0.19),
                (28, _) => (1.0, 0.03, 0.194),
                (29, _) => (1.0, 0.01, 0.198),
                _ => (1.0, 0.0, 0.0), // Failsafe for out of bounds
        }
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
