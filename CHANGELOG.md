# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Python class `PySimResult` exposing high-performance parallel simulation outcomes and detailed distributions to Python.
- Dataframe-ready getters (`cost_histogram_df`, `session_booms_df`, `per_star_friction_df`) returning column-oriented dictionaries suitable for direct Polars DataFrame ingestion.
- Comprehensive simulation metrics and distributions tracking:
  - `RunResult` struct in `starforce.rs` containing cost spent, boom count, and a `per_star_friction` matrix (cost, booms, attempts per star level).
  - `SimMetrics` struct to aggregate simulation runs via Rayon parallel `.fold()` and `.reduce()`.
  - `cost_histogram` (quantized to 100M meso buckets) to support Cumulative Distribution Function (CDF) plotting.
  - `session_booms_histogram` (up to 99 booms) to support Boom Probability Mass Function (PMF) plotting.
  - Cumulative `per_star_friction` array to enable Sojourn Time / bottleneck analysis per star level.

### Changed
- Refactored Python `simulate()` function to return a structured `PySimResult` object instead of a raw `(avg_boom, avg_cost)` tuple.
- Refactored `run_single_sim` and the parallel simulation engine in `main.rs` to return `RunResult` and aggregate into `SimMetrics` instead of returning a simple `(u32, u64)` tuple.
- Moved `SimMetrics` and `BIN_SIZE` to the shared `starforce_core` library (at `starforce.rs`) to de-duplicate definitions.

### Fixed
- Fixed critical integer overflow panic/silent wraparound bug in simulation metric counters by promoting `total_boom` from `u32` to `u64` and `total_cost` from `u64` to `u128` (supporting extremely large trial counts like 100M+).
- Suppressed Rust 2024 edition compilation warnings triggered by PyO3 macros.

## [0.3.0] - 2026-06-16

### Added
- Parameter `start_stars` to Rust core simulation engine `run_single_sim`, integration tests, and PyO3 Python binding `simulate` function (`9be1e13`).

## [0.2.0] - 2026-06-15

### Added
- PyO3 and Maturin support to compile Rust simulation core as a Python native module (`214bc9e`).
- Simulation mechanics for star catching, Safeguard protection, and Shining Star Force (SSF) event modifiers (`42b670a`).

### Changed
- Migrated single monolith repository to a Cargo workspace monorepo layout ([starforce_core](file:///D:/Projects/star-force-sovler/starforce_core) and [starforce_py](file:///D:/Projects/star-force-sovler/starforce_py)) (`8f98558`).
- Split Shining Star Force (SSF) event configuration into distinct boom reduction and cost reduction settings (`9da6dba`).
- Refactored simulator code to eliminate temporal coupling by moving logic from binary entry to configuration structs (`08a37a7`).
- Upgraded scalar primitive parameter types from `u8` to enum classes and `f32` to `f64` for PyO3 alignment (`3ed3fe0`).
- Re-architected code structure separating library logic from binary entry points (`99c4626`).

### Fixed
- Corrected Shining Star Force (SSF) event boom reduction logic to only trigger on stars 0–21 (`9da6dba`).
- Adjusted success rate percentage computation for stars 26-29 (`192669e`).
- Patched critical bugs introduced from unchecked generative AI scripts (`26e08ea`).

### Deprecated
- Redundant and outdated unit test cases (`55f0613`).

## [0.1.0] - 2026-06-08

### Added
- Initial project prototype executing baseline MapleStory GMS Star Force calculations (`3ae4ae3`).
- Validation unit test suite mapped against BrendonMay's Starforce Calculator (`c0c6ed6`).
