import time
import star_force_sim_py

def main():
    trials = 10_000_000
    start_stars = 0
    target_stars = 23
    equip_level = 200

    # Define the 7 modes mapping to levels 15 through 21
    modes = ["Level1"] * 7

    print("Starting simulation...")
    start = time.time()

    result = star_force_sim_py.simulate(
        trials=trials,
        start_stars=start_stars,
        target_stars=target_stars,
        equipment_level=equip_level,
        mode_15_21=modes,
        ssf_boom_reduce_event=True,
        ssf_cost_reduce_event=True,
        safeguard=True,
    )

    elapsed = time.time() - start

    print(result.per_star_friction_df)
    print(f"Time elapsed: {elapsed:.3f} seconds")

if __name__ == "__main__":
    main()