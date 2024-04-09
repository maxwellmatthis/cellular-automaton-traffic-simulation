import numpy as np
from adapter import run_average, SimulationOptions
from plot_helper import plot

VARIABLE = "Traffic Density"
ROUNDS = 100

# x-axis
densities = np.arange(0, 1, 0.05)

# y-axes
average_speeds = []
exit_cell_flows = []
accelerations = []
deaccelerations = []

for density in densities:
    metrics = run_average(SimulationOptions(place_car_probability=density, dilly_dally_probability=0.0), ROUNDS)
    average_speeds.append(metrics.average_speed__kilometers_per_hour)
    exit_cell_flows.append(metrics.exit_cell_flow__cars_per_minute)
    accelerations.append(metrics.accelerations)
    deaccelerations.append(metrics.deaccelerations)

plot(VARIABLE, "Average Speed (km/h)", densities, average_speeds, ROUNDS)
plot(VARIABLE, "Exit Cell Flow (car/min)", densities, exit_cell_flows, ROUNDS)
plot(VARIABLE, "Accelerations", densities, accelerations, ROUNDS)
plot(VARIABLE, "Deaccelerations", densities, deaccelerations, ROUNDS)

