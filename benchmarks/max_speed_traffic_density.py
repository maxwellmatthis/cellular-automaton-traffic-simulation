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
first_cell_flows = []
accelerations = []
deaccelerations = []

SIMULATIONS_EACH = 10
ROUNDS = 600

simulations = len(max_speeds) * len(densities) * SIMULATIONS_EACH
print(f"Running all {simulations} simulations ({simulations * ROUNDS} rounds total)...")
start = time.time()
for max_speed in max_speeds:
    print(f"running for max_speed={max_speed}")
    for density in densities:
        print(f"    running for density={density}")
        max_speeds_expanded.append(max_speed)
        densities_expanded.append(density)
        metrics = run_average(SimulationOptions(vehicles=[(max_speed, 1, density)], rounds=ROUNDS, dilly_dally_probability=0.15), SIMULATIONS_EACH)
        average_speeds.append(metrics.average_speed_kilometers_per_hour)
        first_cell_flows.append(metrics.monitor_cells_flow_cars_per_minute[0])
        accelerations.append(metrics.average_accelerations_n_per_car_per_round)
        deaccelerations.append(metrics.average_deaccelerations_n_per_car_per_round)
print(f"done in {time.time() - start}s")

plot_3d(VARIABLE_X, VARIABLE_Y, "Average Speed (km/h)", max_speeds_expanded, densities_expanded, average_speeds)
plot_3d(VARIABLE_X, VARIABLE_Y, "First Cell Flow (car/min)", max_speeds_expanded, densities_expanded, first_cell_flows)
plot_3d(VARIABLE_X, VARIABLE_Y, "Accelerations (n/car/round)", max_speeds_expanded, densities_expanded, accelerations)

