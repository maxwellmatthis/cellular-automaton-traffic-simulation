import numpy as np
from adapter import run_average, SimulationOptions
from plot_helper import plot

VARIABLE = "Max Speed"
SIMULATIONS_EACH = 100

# x-axis
max_speeds = np.arange(0, 10, 1)

# y-axes
average_speeds = []
first_cell_flows = []
accelerations = []
deaccelerations = []

for max_speed in max_speeds:
    metrics = run_average(SimulationOptions(max_speed=max_speed, traffic_density=0.4, dilly_dally_probability=0.0, monitor=[0, 10, 20]), SIMULATIONS_EACH)
    average_speeds.append(metrics.average_speed_kilometers_per_hour)
    first_cell_flows.append(metrics.monitor_cells_flow_cars_per_minute[0])
    accelerations.append(metrics.average_accelerations_n_per_car_per_round)
    deaccelerations.append(metrics.average_deaccelerations_n_per_car_per_round)

plot(VARIABLE, "Average Speed (km/h)", max_speeds, average_speeds)
plot(VARIABLE, "First Cell Flow (car/min)", max_speeds, first_cell_flows)
plot(VARIABLE, "Accelerations (n/car/round)", max_speeds, accelerations)
plot(VARIABLE, "Deaccelerations (n/car/round)", max_speeds, deaccelerations)

