import star_force_sim_py

# Returns a PySimResult object
result = star_force_sim_py.simulate(
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

print(f"Total runs:  {result.total_runs:,}")
print(f"Total cost:  {result.total_cost:,} mesos")
print(f"Total booms: {result.total_boom:,}")

# Export Polars-compatible dictionaries for DataFrame analysis
cost_histogram_dict = result.cost_histogram_df        # {"cost_bin_start": [...], "count": [...]}
session_booms_dict = result.session_booms_df          # {"booms": [...], "count": [...]}
per_star_friction_dict = result.per_star_friction_df  # {"star": [...], "cost_spent": [...], "booms_triggered": [...], "attempts_made": [...]}