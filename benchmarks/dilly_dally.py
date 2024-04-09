import numpy as np
from adapter import run_average, SimulationOptions, SimulationMetrics
from plot_helper import plot

VARIABLE = "Dilly Dally Probability"
ROUNDS = 100

# x-axis
dilly_dally_probabilities = np.arange(0, 1, 0.05)

# y-axes
average_speeds = []
exit_cell_flows = []
accelerations = []
deaccelerations = []

for dilly_dally_probability in dilly_dally_probabilities:
    metrics = run_average(SimulationOptions(place_car_probability=0.3, dilly_dally_probability=dilly_dally_probability), ROUNDS)
    average_speeds.append(metrics.average_speed__kilometers_per_hour)
    exit_cell_flows.append(metrics.exit_cell_flow__cars_per_minute)
    accelerations.append(metrics.accelerations)
    deaccelerations.append(metrics.deaccelerations)

plot(VARIABLE, "Average Speed (km/h)", dilly_dally_probabilities, average_speeds, ROUNDS)
plot(VARIABLE, "Exit Cell Flow (car/min)", dilly_dally_probabilities, exit_cell_flows, ROUNDS)
plot(VARIABLE, "Accelerations", dilly_dally_probabilities, accelerations, ROUNDS)
plot(VARIABLE, "Deaccelerations", dilly_dally_probabilities, deaccelerations, ROUNDS)

