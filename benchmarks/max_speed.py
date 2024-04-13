import numpy as np
from adapter import run_average, SimulationOptions
from plot_helper import plot

VARIABLE = "Max Speed"
SIMULATIONS_EACH = 100

# x-axis
max_speeds = np.arange(0, 9, 1)

# y-axes
average_speeds = []
exit_cell_flows = []
accelerations = []
deaccelerations = []

for max_speed in max_speeds:
    metrics = run_average(SimulationOptions(max_speed=max_speed, traffic_density=0.4, dilly_dally_probability=0.0), SIMULATIONS_EACH)
    average_speeds.append(metrics.average_speed__kilometers_per_hour)
    exit_cell_flows.append(metrics.exit_cell_flow__cars_per_minute)
    accelerations.append(metrics.accelerations)
    deaccelerations.append(metrics.deaccelerations)

plot(VARIABLE, "Average Speed (km/h)", dilly_dally_probabilities, average_speeds)
plot(VARIABLE, "Exit Cell Flow (car/min)", dilly_dally_probabilities, exit_cell_flows)
plot(VARIABLE, "Accelerations (n/car/round)", dilly_dally_probabilities, accelerations)
plot(VARIABLE, "Deaccelerations (n/car/round)", dilly_dally_probabilities, deaccelerations)

