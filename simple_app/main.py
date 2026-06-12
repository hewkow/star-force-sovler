import time
import star_force_sim

def main():
    trials = 100_000_000
    target_stars = 22
    equip_level = 200

    # Define the 7 modes mapping to levels 15 through 21
    modes = ["Level4"] * 7

    print("Starting simulation...")
    start = time.time()

    avg_boom, avg_cost = star_force_sim.simulate(
        trials=trials,
        target_stars=target_stars,
        equipment_level=equip_level,
        mode_15_21=modes,
        star_catch=True,
        ssf_event=False,
        safeguard=True,
    )

    elapsed = time.time() - start

    print(f"Boom avg count: {avg_boom:.6f}")
    print(f"Total avg cost for lv{equip_level}: {int(avg_cost):,}")
    print(f"Time elapsed: {elapsed:.3f} seconds")

if __name__ == "__main__":
    main()