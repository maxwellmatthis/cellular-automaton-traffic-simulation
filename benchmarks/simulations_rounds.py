import numpy as np
import time
from adapter import run_average, SimulationOptions
from plot_helper import plot_3d

# x-axis
VARIABLE_X = "Simulations"
simulations_eachs = np.arange(1, 61, 10)
simulations_eachs_expanded = []

# y-axis
VARIABLE_Y = "Rounds"
roundss = [10, 50, 100, 300, 500, 700, 1000, 1500, 2000, 3000, 4000, 6000, 8000]
roundss_expanded = []

# z-axes
average_speeds = []
exit_cell_flows = []
accelerations = []
batch_times = []

for simulations_each in simulations_eachs:
    print(f"running for simulations_each={simulations_each}")
    for rounds in roundss:
        print(f"    running for rounds={rounds}")
        simulations_eachs_expanded.append(simulations_each)
        roundss_expanded.append(rounds)
        start = time.time()
        metrics = run_average(SimulationOptions(rounds=rounds), simulations_each)
        batch_times.append(time.time() - start)
        average_speeds.append(metrics.average_speed__kilometers_per_hour)
        exit_cell_flows.append(metrics.exit_cell_flow__cars_per_minute)
        accelerations.append(metrics.accelerations)

plot_3d(VARIABLE_X, VARIABLE_Y, "Average Speed (km/h)", simulations_eachs_expanded, roundss_expanded, average_speeds)
plot_3d(VARIABLE_X, VARIABLE_Y, "Exit Cell Flow (car/min)", simulations_eachs_expanded, roundss_expanded, exit_cell_flows)
plot_3d(VARIABLE_X, VARIABLE_Y, "Accelerations", simulations_eachs_expanded, roundss_expanded, accelerations)
plot_3d(VARIABLE_X, VARIABLE_Y, "Batch Time", simulations_eachs_expanded, roundss_expanded, batch_times)

