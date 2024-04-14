import numpy as np
from adapter import run_average, SimulationOptions
from plot_helper import plot

VARIABLE = "Dilly Dally Probability"
SIMULATIONS_EACH = 100

# x-axis
dilly_dally_probabilities = np.arange(0, 1, 0.05)

# y-axes
average_speeds = []
first_cell_flows = []
accelerations = []
deaccelerations = []

for dilly_dally_probability in dilly_dally_probabilities:
    metrics = run_average(SimulationOptions(traffic_density=0.3, dilly_dally_probability=dilly_dally_probability), SIMULATIONS_EACH)
    average_speeds.append(metrics.average_speed_kilometers_per_hour)
    first_cell_flows.append(metrics.monitor_cells_flow_cars_per_minute[0])
    accelerations.append(metrics.average_accelerations_n_per_car_per_round)
    deaccelerations.append(metrics.average_accelerations_n_per_car_per_round)

plot(VARIABLE, "Average Speed (km/h)", dilly_dally_probabilities, average_speeds)
plot(VARIABLE, "First Cell Flow (car/min)", dilly_dally_probabilities, first_cell_flows)
plot(VARIABLE, "Accelerations (n/car/round)", dilly_dally_probabilities, accelerations)
plot(VARIABLE, "Deaccelerations (n/car/round)", dilly_dally_probabilities, deaccelerations)

