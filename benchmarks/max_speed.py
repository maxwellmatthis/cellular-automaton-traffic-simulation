import numpy as np
from adapter import run_average, SimulationOptions
from plot_helper import plot

VARIABLE = "Max Speed"
SIMULATIONS_EACH = 30

# x-axis
max_speeds = np.arange(0, 6, 1)

# y-axes
average_speeds = []
first_cell_flows = []
accelerations = []
deaccelerations = []

for max_speed in max_speeds:
    metrics = run_average(SimulationOptions(vehicles=[(max_speed, 1, 0.3)], dilly_dally_probability=0.15, monitor=[(0, 0)], rounds=1000), SIMULATIONS_EACH)
    average_speeds.append(metrics.average_speed_kilometers_per_hour)
    first_cell_flows.append(metrics.monitor_cells_flow_cars_per_minute[0])
    accelerations.append(metrics.average_accelerations_n_per_car_per_round)
    deaccelerations.append(metrics.average_deaccelerations_n_per_car_per_round)

plot(VARIABLE, "Average Speed (km/h)", max_speeds, average_speeds)
plot(VARIABLE, "First Cell Flow (car/min)", max_speeds, first_cell_flows)
plot(VARIABLE, "Accelerations (n/car/round)", max_speeds, accelerations)

