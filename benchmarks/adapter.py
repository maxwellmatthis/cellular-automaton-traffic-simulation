import json
import subprocess
from typing import Optional, Self, List, Tuple
from dataclasses import dataclass, fields

# BINARY = "cargo run --"
BINARY = "target/release/cellular-automaton-traffic-simulation"

@dataclass
class SimulationOptions:
    lanes: Optional[int] = None
    rounds: Optional[int] = None
    length: Optional[int] = None
    vehicles: Optional[List[Tuple[float, float, float]]] = None
    dilly_dally_probability: Optional[float] = None
    stay_in_lane_probability: Optional[float] = None
    monitor: Optional[List[Tuple[int, int]]] = None

    verbose: bool = None
    image: bool = None
    out_path: Optional[str] = None

    def to_flags_and_vals(self):
        flags_and_vals = []
        for field in fields(self):
            val = getattr(self, field.name)
            if val == None:
                continue
            if type(val) is list:
                val = ";".join(map(lambda v : str(v).replace(" ", ""), val))
            flags_and_vals.extend([f"--{field.name.replace('_', '-')}", str(val)])
        return flags_and_vals

@dataclass
class SimulationResult:
    rounds: int
    lanes: int
    length: int
    # TODO: add complex simulation parameters and exclude them from averageing
    cars: int
    dilly_dally_probability: float
    stay_in_lane_probability: float

    runtime_s: float
    average_speed_kilometers_per_hour: float
    monitor_cells_flow_cars_per_minute: List[float]
    average_accelerations_n_per_car_per_round: int
    average_deaccelerations_n_per_car_per_round: int

    def __init__(self, json: str):
        for field in fields(self):
            setattr(self, field.name, json[field.name])

    def add(self, other: Self):
        for field in fields(self):
            own_val = getattr(self, field.name)
            other_val = getattr(other, field.name)

            if type(own_val) is list:
                for i in range(len(own_val)):
                    own_val[i] += other_val[i]
            else:
                setattr(self, field.name, own_val + other_val)

    def divide_all(self, by: int):
        for field in fields(self):
            val = getattr(self, field.name)
            if type(val) is list:
                for i in range(len(val)):
                    val[i] /= by
            else:
                setattr(self, field.name, val / by)

def run(simulation_options: SimulationOptions):
    output = subprocess.run([BINARY, *simulation_options.to_flags_and_vals()], stdout=subprocess.PIPE).stdout.decode("utf-8").strip().split("\n")[-1]
    metrics = json.loads(output)
    return SimulationResult(metrics)

def run_average(simulation_options: SimulationOptions, simulations: int):
    average_metrics: Optional[SimulationResult] = None
    for r in range(simulations):
        metrics = run(simulation_options)
        if average_metrics == None:
            average_metrics = metrics
            continue
        average_metrics.add(metrics)
    average_metrics.divide_all(simulations)
    return average_metrics


if __name__ == "__main__":
    print(run(SimulationOptions()))

