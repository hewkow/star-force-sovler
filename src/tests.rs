use super::*;

fn run_test_sim(equipment_level: u32, target_stars: usize, trials: u32) -> (f32, u64) {
    let star_prob: [[f32; 3]; 30] = [
        [0.95, 0.05, 0.0], [0.9, 0.1, 0.0], [0.85, 0.15, 0.0], [0.85, 0.15, 0.0], [0.80, 0.2, 0.0],
        [0.75, 0.25, 0.0], [0.7, 0.3, 0.0], [0.65, 0.35, 0.0], [0.6, 0.4, 0.0], [0.55, 0.45, 0.0],
        [0.5, 0.5, 0.0], [0.45, 0.55, 0.0], [0.4, 0.6, 0.0], [0.35, 0.65, 0.0], [0.3, 0.7, 0.0],
        [0.3, 0.679, 0.021], [0.3, 0.679, 0.021], [0.15, 0.782, 0.068], [0.15, 0.782, 0.068],
        [0.15, 0.765, 0.085], [0.3, 0.595, 0.105], [0.15, 0.7225, 0.1275], [0.15, 0.68, 0.17],
        [0.10, 0.72, 0.18], [0.10, 0.72, 0.18], [0.10, 0.72, 0.18], [0.07, 0.744, 0.186],
        [0.05, 0.76, 0.19], [0.03, 0.776, 0.194], [0.01, 0.792, 0.198],
    ];

    let mut boom_thresholds = [0u32; 30];
    let mut success_thresholds = [0u32; 30];
    for i in 0..30 {
        boom_thresholds[i] = (star_prob[i][2] as f64 * 4294967296.0).round() as u32;
        success_thresholds[i] = ((star_prob[i][2] as f64 + star_prob[i][0] as f64) * 4294967296.0).round() as u32;
    }

    let mut cost_lookup = [0u64; 30];
    for i in 0..30 {
        cost_lookup[i] = kms_cost(i as u32, equipment_level);
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
    let diff = (actual - expected).abs();
    let allowed = expected * tolerance_pct;
    assert!(
        diff <= allowed,
        "{} value {} deviated from expected {} by more than {}%",
        metric, actual, expected, tolerance_pct * 100.0
    );
}

const TEST_TRIALS: u32 = 1_000_000;
const TOLERANCE: f32 = 0.05; // 5% allowed variance for statistical convergence

#[test]
fn test_case_1_lv140_star15() {
    let (avg_boom, avg_cost) = run_test_sim(140, 15, TEST_TRIALS);
    assert_within_tolerance(avg_boom, 0.0, TOLERANCE, "Boom");
    assert_within_tolerance(avg_cost as f32, 355e6, TOLERANCE, "Cost");
}

#[test]
fn test_case_1_lv140_star19() {
    let (avg_boom, avg_cost) = run_test_sim(140, 19, TEST_TRIALS);
    assert_within_tolerance(avg_boom, 1.43, TOLERANCE, "Boom");
    assert_within_tolerance(avg_cost as f32, 2.3e9, TOLERANCE, "Cost");
}

#[test]
fn test_case_2_lv140_star23() {
    let (avg_boom, avg_cost) = run_test_sim(140, 23, TEST_TRIALS);
    assert_within_tolerance(avg_boom, 18.87, TOLERANCE, "Boom");
    assert_within_tolerance(avg_cost as f32, 26.7e9, TOLERANCE, "Cost");
}

#[test]
fn test_case_3_lv160_star19() {
    let (avg_boom, avg_cost) = run_test_sim(160, 19, TEST_TRIALS);
    assert_within_tolerance(avg_boom, 1.45, TOLERANCE, "Boom");
    assert_within_tolerance(avg_cost as f32, 3.5e9, TOLERANCE, "Cost");
}

#[test]
fn test_case_4_lv160_star22() {
    let (avg_boom, avg_cost) = run_test_sim(160, 22, TEST_TRIALS);
    assert_within_tolerance(avg_boom, 8.4, TOLERANCE, "Boom");
    assert_within_tolerance(avg_cost as f32, 19.0e9, TOLERANCE, "Cost");
}

#[test]
fn test_case_5_lv200_star18() {
    let (avg_boom, avg_cost) = run_test_sim(200, 18, TEST_TRIALS);
    assert_within_tolerance(avg_boom, 0.66, TOLERANCE, "Boom");
    assert_within_tolerance(avg_cost as f32, 3.3e9, TOLERANCE, "Cost");
}

#[test]
fn test_case_6_lv200_star24() {
    let (avg_boom, avg_cost) = run_test_sim(200, 24, TEST_TRIALS);
    assert_within_tolerance(avg_boom, 50.74, TOLERANCE, "Boom");
    assert_within_tolerance(avg_cost as f32, 206.0e9, TOLERANCE, "Cost");
}
