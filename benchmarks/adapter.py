import json
import subprocess
from typing import Optional, Self
from dataclasses import dataclass, fields

# BINARY = "cargo run --"
BINARY = "target/release/cellular-automaton-traffic-simulation"

@dataclass
class SimulationOptions:
    rounds: Optional[int] = None
    length: Optional[int] = None
    max_speed: Optional[int] = None
    traffic_density: Optional[float] = None
    dilly_dally_probability: Optional[float] = None
    img_out: Optional[str] = None

    def to_flags_and_vals(self):
        flags_and_vals = []
        for field in fields(self):
            val = getattr(self, field.name)
            if val == None:
                continue
            flags_and_vals.extend([f"--{field.name.replace('_', '-')}", str(val)])
        return flags_and_vals

@dataclass
class SimulationMetrics:
    rounds: int
    max_speed: int
    traffic_density: float
    cars: int
    dilly_dally_probability: float

    runtime__s: float
    average_speed__kilometers_per_hour: float
    exit_cell_flow__cars_per_minute: float
    average_accelerations__n_per_car_per_round: int
    average_deaccelerations__n_per_car_per_round: int

    def __init__(self, json: str):
        for field in fields(self):
            setattr(self, field.name, json[field.name])

    def add(self, other: Self):
        for field in fields(self):
            setattr(
                self,
                field.name,
                getattr(self, field.name) +
                    getattr(other, field.name)
            )

    def divide_all(self, by: int):
        for field in fields(self):
            setattr(
                self,
                field.name,
                getattr(self, field.name) / by
            )

def run(simulation_options: SimulationOptions):
    output = subprocess.run([BINARY, *simulation_options.to_flags_and_vals()], stdout=subprocess.PIPE).stdout.decode("utf-8")
    metrics = json.loads(output)
    return SimulationMetrics(metrics)

def run_average(simulation_options: SimulationOptions, simulations: int):
    average_metrics: Optional[SimulationMetrics] = None
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
