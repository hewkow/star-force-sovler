use rand::prelude::*;
use rand::rngs::SmallRng;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnhancementMode {
    Standard,
    Level1,
    Level2,
    Level3,
    Level4,
}

pub struct EnhanceConfig {
    pub mode_15_21 : [EnhancementMode; 7],
    pub star_catch: bool,
    pub ssf_cost_reduce_event: bool,
    pub ssf_boom_reduce_event: bool,
    pub safeguard: bool,
}

pub struct StarProp {
    pub stars: u8,
    pub cost_multiply : f64,
    pub success_rate : f64,
    pub boom_rate : f64,

    pub enhance_level : EnhancementMode,
}

impl StarProp {
    pub fn new(stars : u8, config: &EnhanceConfig) -> Self{
        let mut mode = match stars {
            15..=21 => config.mode_15_21[(stars - 15) as usize],
            _ => EnhancementMode::Standard
        };

        let (mut cost_mult, mut success, mut boom) = Self::get_base_rates(stars, mode);
        if config.star_catch {
            let base_success = success;
            success = (base_success * 1.05).min(1.0);
            
            let denom = 1.0 - base_success;
            if denom > 0.0 {
                let left = 1.0 - success; 
                boom = (boom * left) / denom;
            } else {
                boom = 0.0;
            }
        }
        
        if config.safeguard && (15..=17).contains(&stars) && boom > 0.0 {
            // change level to 1 when safeguard is true at 15..=17
            match mode {
                EnhancementMode::Standard | EnhancementMode::Level1 => {}
                EnhancementMode::Level2 | EnhancementMode::Level3 | EnhancementMode::Level4 => {
                    mode = EnhancementMode::Level1;
                }
            }
            boom = 0.0;
            cost_mult = 3.0;
        }
                
        if config.ssf_boom_reduce_event && (1..=21).contains(&stars) {
            // now it's confirmed that ssf applied to new enhancement mode 
            boom *= 0.7;
        }

        if config.ssf_cost_reduce_event {
            match  mode {
                EnhancementMode::Standard | EnhancementMode::Level1 => {
                    cost_mult -= 0.30;
                }
                EnhancementMode::Level2 | EnhancementMode::Level3 | EnhancementMode::Level4 => {
                    cost_mult *= 0.7;
                }
            }
        }
        
        
        Self {
            stars,
            cost_multiply: cost_mult,
            success_rate: success,
            boom_rate: boom,
            enhance_level: mode,
        }
    }

    pub fn get_base_rates( stars : u8, mode: EnhancementMode) -> (f64, f64, f64) {
        match (stars, mode) {
    // Your custom enhancement rules for 15-21
                (15..=16, EnhancementMode::Level1) => (1.0, 0.30, 0.0210),
                (15..=16, EnhancementMode::Level2) => (1.5, 0.30, 0.0140),
                (15..=16, EnhancementMode::Level3) => (2.5, 0.30, 0.0070),
                (15..=16, EnhancementMode::Level4) => (3.0, 0.30, 0.0000),
    
                (17, EnhancementMode::Level1)      => (1.0, 0.15, 0.0680),
                (17, EnhancementMode::Level2)      => (1.5, 0.15, 0.0425),
                (17, EnhancementMode::Level3)      => (2.5, 0.15, 0.0170),
                (17, EnhancementMode::Level4)      => (3.0, 0.15, 0.0000),
    
                (18, EnhancementMode::Level1)      => (1.0, 0.15, 0.0680),
                (18, EnhancementMode::Level2)      => (2.0, 0.12, 0.0440),
                (18, EnhancementMode::Level3)      => (3.5, 0.10, 0.0180),
                (18, EnhancementMode::Level4)      => (6.5, 0.08, 0.0000),
    
                (19, EnhancementMode::Level1)      => (1.0, 0.15, 0.0850),
                (19, EnhancementMode::Level2)      => (2.0, 0.12, 0.0616),
                (19, EnhancementMode::Level3)      => (3.5, 0.10, 0.0360),
                (19, EnhancementMode::Level4)      => (6.5, 0.08, 0.0000),
    
                (20, EnhancementMode::Level1)      => (1.0, 0.30, 0.1050),
                (20, EnhancementMode::Level2)      => (2.0, 0.25, 0.0750),
                (20, EnhancementMode::Level3)      => (3.5, 0.20, 0.0400),
                (20, EnhancementMode::Level4)      => (6.5, 0.15, 0.0000),
    
                (21, EnhancementMode::Level1)      => (1.0, 0.15, 0.1275),
                (21, EnhancementMode::Level2)      => (2.0, 0.12, 0.0880),
                (21, EnhancementMode::Level3)      => (3.5, 0.10, 0.0450),
                (21, EnhancementMode::Level4)      => (6.5, 0.08, 0.0000),
    
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

pub struct RunResult {
    pub total_cost: u64,
    pub total_booms: u32,
    // [Cost Spent, Booms Triggered, Attempts Made]
    pub per_star_friction: [[u64; 3]; 30],
}


pub fn run_single_sim(
    start_stars: usize,
    target_star: usize,
    rng: &mut SmallRng,
    boom_thresholds: &[u32; 30],
    success_thresholds: &[u32; 30],
    cost_lookup: &[u64; 30],
) -> RunResult {
    let mut current_star: usize = start_stars;

    let mut result = RunResult {
        total_cost: 0,
        total_booms: 0,
        per_star_friction: [[0; 3]; 30],
    };
    
    while current_star < target_star && current_star < 30 {
        let attempt_cost = cost_lookup[current_star];
        
        result.total_cost += attempt_cost;
        result.per_star_friction[current_star][0] += attempt_cost;
        result.per_star_friction[current_star][2] += 1;

        let val: u32 = rng.random();
        
        if val < boom_thresholds[current_star] {
            result.total_booms += 1;
            result.per_star_friction[current_star][1] += 1;
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
    result
}

pub const BIN_SIZE: u64 = 100_000_000;

#[derive(Clone, Debug)]
pub struct SimMetrics {
    pub cost_histogram: BTreeMap<u64, u32>,
    pub session_booms_histogram: [u32; 100],
    pub per_star_friction: [[u64; 3]; 30],
    pub total_runs: u32,
    pub total_cost: u128,
    pub total_boom: u64,
}

impl Default for SimMetrics {
    fn default() -> Self {
        Self {
            cost_histogram: BTreeMap::new(),
            session_booms_histogram: [0; 100],
            per_star_friction: [[0; 3]; 30],
            total_runs: 0,
            total_boom: 0,
            total_cost: 0,
        }
    }
}

impl SimMetrics {
    pub fn add_run(&mut self, run: RunResult) {
        let bin = run.total_cost / BIN_SIZE;
        *self.cost_histogram.entry(bin).or_insert(0) += 1;

        let boom_idx = (run.total_booms as usize).min(99);
        self.session_booms_histogram[boom_idx] += 1;

        for i in 0..30 {
            self.per_star_friction[i][0] += run.per_star_friction[i][0];
            self.per_star_friction[i][1] += run.per_star_friction[i][1];
            self.per_star_friction[i][2] += run.per_star_friction[i][2];
        }

        self.total_cost += run.total_cost as u128;
        self.total_boom += run.total_booms as u64;
        self.total_runs += 1;
    }

    pub fn merge(mut self, other: Self) -> Self {
        for (bin, count) in other.cost_histogram {
            *self.cost_histogram.entry(bin).or_insert(0) += count;
        }
        for i in 0..100 {
            self.session_booms_histogram[i] += other.session_booms_histogram[i];
        }
        for i in 0..30 {
            self.per_star_friction[i][0] += other.per_star_friction[i][0];
            self.per_star_friction[i][1] += other.per_star_friction[i][1];
            self.per_star_friction[i][2] += other.per_star_friction[i][2];
        }
        self.total_cost += other.total_cost;
        self.total_boom += other.total_boom;
        self.total_runs += other.total_runs;
        self
    }
}

