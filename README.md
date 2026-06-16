# MapleStory GMS Star Force Monte Carlo Simulator

A high-performance Star Force cost simulator for MapleStory GMS, written in Rust with a Python binding layer. Uses parallel Monte Carlo simulation to estimate average meso cost and equipment boom counts under various enhancement configurations.

---

## Features

- **Star Catching** — 5% success rate boost with correctly adjusted boom probability
- **Safeguard** — zero-boom protection on stars 15–17, with the 3× cost multiplier
- **Shining Star Force (SSF) Events**
  - 30% boom rate reduction event (`ssf_boom_reduce_event`)
  - 30% cost reduction event (`ssf_cost_reduce_event`)
- **New 4-Level Enhancement System** (stars 15–21)
  - `Standard` — baseline GMS rates
  - `Level1` — same rates as Standard (1× cost)
  - `Level2` — reduced boom, increased cost multiplier (~1.5–2×)
  - `Level3` — further reduced boom, higher cost multiplier (~2.5–3.5×)
  - `Level4` — **zero boom**, highest cost multiplier (~3–6.5×)
  - Per-star configuration: each star from 15 to 21 can be set independently
- **Flexible Starting Star** — configure arbitrary starting star levels 
- **Validated test matrix** — comprehensive integration tests covering 40+ config combinations, cross-checked against [MathBro's calculator](https://brendonmay.github.io/starforceCalculator/) and [v269 GMS Star Force Calculator](https://starforce.tadeucci.dev/) within 5% tolerance

---

## Architecture

```
star-force-sovler/
├── starforce_core/          # Pure Rust simulation library + binaries
│   └── src/
│       ├── starforce.rs     # Core types: EnchanceConfig, StarProp, kms_cost, run_single_sim
│       ├── lib.rs           # Crate exports
│       ├── main.rs          # Standalone Rust binary (with integrated test matrix)
│       └── bin/
│           ├── feature.rs   # Scratch binary for prototyping new mechanics
│           └── arai.rs      # Scratch binary for raw rate inspection
│
└── starforce_py/            # PyO3 Python extension module
    ├── src/lib.rs           # `simulate()` PyO3 binding, GIL-releasing parallel run
    └── main.py              # Example Python driver
```

**Runtime flow:** `EnchanceConfig` → pre-compute `StarProp[30]` + threshold/cost lookup tables → Rayon parallel `run_single_sim` over N trials → aggregate (avg_boom, avg_cost).

The integer threshold trick (`rate * 2^32` → `u32`) eliminates floating-point comparisons in the hot loop.

---

## Usage

### Rust (standalone binary)

```bash
cargo run --release -p starforce_core
```

Configurable constants in `main.rs`: `trials`, `target_stars`, `equipment_level`, `EnchanceConfig`.

### Python (PyO3 extension)

Build the extension wheel (requires [maturin](https://github.com/PyO3/maturin)):

```bash
cd starforce_py
maturin develop --release
python main.py
```

Python API:

```python
import star_force_sim_py

avg_boom, avg_cost = star_force_sim_py.simulate(
    trials=10_000_000,
    start_stars=15,
    target_stars=22,
    equipment_level=200,
    mode_15_21=["Level1"] * 7,   # one mode per star 15-21
    star_catch=True,
    ssf_boom_reduce_event=True,
    ssf_cost_reduce_event=True,
    safeguard=False,
)
print(f"Avg booms: {avg_boom:.4f}")
print(f"Avg cost:  {int(avg_cost):,} mesos")
```

`mode_15_21` must be a list of exactly 7 strings, one per star (15 → 21), each one of:
`"Standard"`, `"Level1"`, `"Level2"`, `"Level3"`, `"Level4"`.

### Run tests

```bash
cargo test --release -p starforce_core
```

---

## Roadmap

- [ ] Solver: optimal strategy for cost-efficient or boom-minimizing paths
- [ ] WebGUI with per-slot configuration
- [ ] Drag-and-drop formula/strategy builder

---

## Credits & Acknowledgments


- **Rates & Formulas:** Success/boom rates and the KMS cost formula derived from [MathBro's Star Force Calculator](https://brendonmay.github.io/starforceCalculator/) and [AngeloTadeucci/starforcing-test](https://starforce.tadeucci.dev/)
