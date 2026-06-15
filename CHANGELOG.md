# Changelog

All notable changes to this project will be documented in this file.

### [0.2.0] 2026-06-15
- **Refactor:** Migrate repository into a Cargo workspace monorepo (`8f98558`)
- **Refactor:** Split SSF into 2 configs and correct boom reduction logic to only apply for 0-21 stars (`9da6dba`)

### 2026-06-12
- **Build:** Setup PyO3 and Maturin to wrap the Rust core into Python (`214bc9e`)
- **Documentation:** Update README to remove the "bare minimum" note (`2a074d2`)
- **Documentation:** Update current features in README.md (`a7d57cc`)
- **Miscellaneous:** Change little config in main.rs (`4afbcc5`)
- **Refactor:** Merge branch 'refactor/split-modules' (`d3aae3b`)
- **Feature:** Implement starcatching, shining star force (SSF) event, and safeguard mechanisms (`42b670a`)
- **Testing:** Deprecate some old tests (`55f0613`)
- **Bug Fix:** Fix issues caused by unverified AI code ("commit of shame") (`26e08ea`)

### 2026-06-11
- **Bug Fix:** Correct success rate at 26-29 stars (`192669e`)
- **Refactor:** Remove Temporal Coupling by moving logic from main to struct (`08a37a7`)

### 2026-06-10
- **Refactor:** Change u8 to enum, and f32 to f64 for future PyO3 compatibility (`3ed3fe0`)
- **Refactor:** Split monolith into library and binary crates (`99c4626`)

### [0.1.0] 2026-06-08
- **Documentation:** Update reference (`f8331e8`)
- **Testing:** Create unit tests based on https://brendonmay.github.io/starforceCalculator/ (`c0c6ed6`)
- **Documentation:** Update link for inspiration source in README (`1e75957`)
- **Miscellaneous:** Initial prototype: MapleStory Starforce bare minimum, no starcatch, no safeguard, and no SSF (`3ae4ae3`)
