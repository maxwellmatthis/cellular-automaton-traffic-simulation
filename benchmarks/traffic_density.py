import numpy as np
from adapter import run_average, SimulationOptions
from plot_helper import plot

VARIABLE = "Traffic Density"
SIMULATIONS_EACH = 100

# x-axis
densities = np.arange(0.05, 1, 0.05)

# y-axes
average_speeds = []
first_cell_flows = []
accelerations = []
deaccelerations = []

for density in densities:
    metrics = run_average(SimulationOptions(vehicles=[(5, 1, density), dilly_dally_probability=0.0), SIMULATIONS_EACH)
    average_speeds.append(metrics.average_speed_kilometers_per_hour)
    first_cell_flows.append(metrics.monitor_cells_flow_cars_per_minute[0])
    accelerations.append(metrics.average_accelerations_n_per_car_per_round)
    deaccelerations.append(metrics.average_deaccelerations_n_per_car_per_round)

plot(VARIABLE, "Average Speed (km/h)", densities, average_speeds)
plot(VARIABLE, "First Cell Flow (car/min)", densities, first_cell_flows)
plot(VARIABLE, "Accelerations (n/car/round)", densities, accelerations)
plot(VARIABLE, "Deaccelerations (n/car/round)", densities, deaccelerations)

