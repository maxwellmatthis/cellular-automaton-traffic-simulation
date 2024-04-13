import numpy as np
from adapter import run_average, SimulationOptions
from plot_helper import plot

VARIABLE = "Traffic Density"
SIMULATIONS_EACH = 100

# x-axis
densities = np.arange(0.05, 1, 0.05)

# y-axes
average_speeds = []
exit_cell_flows = []
accelerations = []
deaccelerations = []

for density in densities:
    metrics = run_average(SimulationOptions(place_car_probability=density, dilly_dally_probability=0.0), SIMULATIONS_EACH)
    average_speeds.append(metrics.average_speed__kilometers_per_hour)
    exit_cell_flows.append(metrics.exit_cell_flow__cars_per_minute)
    accelerations.append(metrics.average_accelerations__n_per_car_per_round)
    deaccelerations.append(metrics.average_deaccelerations__n_per_car_per_round)

plot(VARIABLE, "Average Speed (km/h)", densities, average_speeds)
plot(VARIABLE, "Exit Cell Flow (car/min)", densities, exit_cell_flows)
plot(VARIABLE, "Accelerations (n/car/round)", densities, accelerations)
plot(VARIABLE, "Deaccelerations (n/car/round)", densities, deaccelerations)

