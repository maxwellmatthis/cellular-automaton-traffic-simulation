import numpy as np
from adapter import run_average, SimulationOptions, SimulationMetrics
from plot_helper import plot

VARIABLE = "Max Speed"
ROUNDS = 100

# x-axis
max_speeds = np.arange(0, 9, 1)

# y-axes
average_speeds = []
exit_cell_flows = []
accelerations = []
deaccelerations = []

for max_speed in max_speeds:
    metrics = run_average(SimulationOptions(max_speed=max_speed, place_car_probability=0.4, dilly_dally_probability=0.0), ROUNDS)
    average_speeds.append(metrics.average_speed__kilometers_per_hour)
    exit_cell_flows.append(metrics.exit_cell_flow__cars_per_minute)
    accelerations.append(metrics.accelerations)
    deaccelerations.append(metrics.deaccelerations)

plot(VARIABLE, "Average Speed (km/h)", max_speeds, average_speeds, ROUNDS)
plot(VARIABLE, "Exit Cell Flow (car/min)", max_speeds, exit_cell_flows, ROUNDS)
plot(VARIABLE, "Accelerations", max_speeds, accelerations, ROUNDS)
plot(VARIABLE, "Deaccelerations", max_speeds, deaccelerations, ROUNDS)
