import numpy as np
import time
from adapter import run_average, SimulationOptions
from plot_helper import plot_3d, plot

# x-axis
VARIABLE_X = "Max Speed"
max_speeds = np.arange(0, 11)
max_speeds_expanded = []

# y-axis
VARIABLE_Y = "Traffic Density"
densities = np.arange(0.1, 1, 0.05)
densities_expanded = []

# z-axes
average_speeds = []
exit_cell_flows = []
accelerations = []
deaccelerations = []

SIMULATIONS_EACH = 10
ROUNDS = 5000

simulations = len(max_speeds) * len(densities) * SIMULATIONS_EACH
print(f"Running all {simulations} simulations ({simulations * ROUNDS} rounds total)...", end="", flush=True)
start = time.time()
for max_speed in max_speeds:
    print(f"running for max_speed={max_speed}")
    for density in densities:
        print(f"    running for density={density}")
        max_speeds_expanded.append(max_speed)
        densities_expanded.append(density)
        metrics = run_average(SimulationOptions(traffic_density=density, max_speed=max_speed, rounds=ROUNDS), SIMULATIONS_EACH)
        average_speeds.append(metrics.average_speed__kilometers_per_hour)
        exit_cell_flows.append(metrics.exit_cell_flow__cars_per_minute)
        accelerations.append(metrics.average_accelerations__n_per_car_per_round)
        deaccelerations.append(metrics.average_deaccelerations__n_per_car_per_round)
print(f"done in {time.time() - start}s")

plot_3d(VARIABLE_X, VARIABLE_Y, "Average Speed (km/h)", max_speeds_expanded, densities_expanded, average_speeds)
plot_3d(VARIABLE_X, VARIABLE_Y, "Exit Cell Flow (car/min)", max_speeds_expanded, densities_expanded, exit_cell_flows)
plot_3d(VARIABLE_X, VARIABLE_Y, "Accelerations (n/car/round)", max_speeds_expanded, densities_expanded, accelerations)
plot_3d(VARIABLE_X, VARIABLE_Y, "Deaccelerations (n/car/round)", max_speeds_expanded, densities_expanded, deaccelerations)

