from multiprocessing import Pool
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


def run_rounds(simulations_each: int):
    l_simulations_eachs_expanded = []
    l_roundss_expanded = []

    # z-axes
    l_average_speeds = []
    l_exit_cell_flows = []
    l_accelerations = []
    l_batch_times = []

    for rounds in roundss:
        print(f"    running for rounds={rounds}")
        l_simulations_eachs_expanded.append(simulations_each)
        l_roundss_expanded.append(rounds)
        start = time.time()
        metrics = run_average(SimulationOptions(rounds=rounds), simulations_each)
        l_batch_times.append(time.time() - start)
        l_average_speeds.append(metrics.average_speed__kilometers_per_hour)
        l_exit_cell_flows.append(metrics.exit_cell_flow__cars_per_minute)
        l_accelerations.append(metrics.average_accelerations__n_per_car_per_round)

    return (l_simulations_eachs_expanded, l_roundss_expanded, l_average_speeds, l_exit_cell_flows, l_accelerations, l_batch_times)

if __name__ == "__main__":
    with Pool(6) as p:
        for rounds_result in p.map(run_rounds, simulations_eachs):
            simulations_eachs_expanded.extend(rounds_result[0])
            roundss_expanded.extend(rounds_result[1])

            average_speeds.extend(rounds_result[2])
            exit_cell_flows.extend(rounds_result[3])
            accelerations.extend(rounds_result[4])
            batch_times.extend(rounds_result[5])

    plot_3d(VARIABLE_X, VARIABLE_Y, "Average Speed (km/h)", simulations_eachs_expanded, roundss_expanded, average_speeds)
    plot_3d(VARIABLE_X, VARIABLE_Y, "Exit Cell Flow (car/min)", simulations_eachs_expanded, roundss_expanded, exit_cell_flows)
    plot_3d(VARIABLE_X, VARIABLE_Y, "Accelerations (n/car/round)", simulations_eachs_expanded, roundss_expanded, accelerations)
    plot_3d(VARIABLE_X, VARIABLE_Y, "Batch Time", simulations_eachs_expanded, roundss_expanded, batch_times)

