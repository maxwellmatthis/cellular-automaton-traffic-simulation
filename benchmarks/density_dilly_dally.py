import numpy as np
import time
from adapter import run_average, SimulationOptions
from plot_helper import plot_3d, plot

# x-axis
VARIABLE_X = "Dilly Dally Probability (dilly_dallies/round/car)"
dilly_dally_probabilities = np.arange(0.0, 1.0, 0.05)
dilly_dally_probabilities_expanded = []

# y-axis
VARIABLE_Y = "Traffic Density (car/cell)"
densities = np.arange(0.1, 0.65, 0.05)
densities_expanded = []

# z-axes
average_speeds = []
first_cell_flows = []
accelerations = []
deaccelerations = []

SIMULATIONS_EACH = 10
ROUNDS = 600

MAX_SPEED = 5

simulations = len(dilly_dally_probabilities) * len(densities) * SIMULATIONS_EACH
print(f"Running all {simulations} simulations ({simulations * ROUNDS} rounds total)...")
start = time.time()
for dilly_dally_probability in dilly_dally_probabilities:
    print(f"running for dilly_dally_probability={dilly_dally_probability}")
    for density in densities:
        print(f"    running for density={density}")
        metrics = run_average(SimulationOptions(vehicles=[(MAX_SPEED, 1, density)], rounds=ROUNDS, dilly_dally_probability=dilly_dally_probability), SIMULATIONS_EACH)
        flow = metrics.monitor_cells_flow_cars_per_minute[0]
        if 31-1 < flow < 31+1:
            print(f"dilly-dally-probability: {dilly_dally_probability}, density: {density}, flow: {flow}")
            dilly_dally_probabilities_expanded.append(dilly_dally_probability)
            densities_expanded.append(density)
            average_speeds.append(metrics.average_speed_kilometers_per_hour)
            first_cell_flows.append(flow)
            accelerations.append(metrics.average_accelerations_n_per_car_per_round)
            deaccelerations.append(metrics.average_deaccelerations_n_per_car_per_round)
print(f"done in {time.time() - start}s")

# plot_3d(VARIABLE_X, VARIABLE_Y, "Average Speed (km/h)", dilly_dally_probabilities_expanded, densities_expanded, average_speeds)
plot_3d(VARIABLE_X, VARIABLE_Y, "First Cell Flow (car/min)", dilly_dally_probabilities_expanded, densities_expanded, first_cell_flows)
# plot_3d(VARIABLE_X, VARIABLE_Y, "Accelerations (n/car/round)", dilly_dally_probabilities_expanded, densities_expanded, accelerations)
