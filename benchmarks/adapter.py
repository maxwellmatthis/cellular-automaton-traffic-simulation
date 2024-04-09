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
    place_car_probability: Optional[float] = None
    dilly_dally_probability: Optional[float] = None
    spawn_car_at_entrance_probability: Optional[float] = None
    remove_car_on_exit_probability: Optional[float] = None
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
    dilly_dally_probability: float
    place_car_probability: float
    spawn_car_at_entrance_probability: float
    remove_car_on_exit_probability: float

    runtime: float
    average_speed__kilometers_per_hour: float
    exit_cell_flow__cars_per_minute: float
    accelerations: int
    deaccelerations: int

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

def run_average(simulation_options: SimulationOptions, rounds: int = 10):
    average_metrics: Optional[SimulationMetrics] = None
    for r in range(rounds):
        metrics = run(simulation_options)
        if average_metrics == None:
            average_metrics = metrics
            continue
        average_metrics.add(metrics)
    average_metrics.divide_all(rounds)
    return average_metrics


if __name__ == "__main__":
    print(run(SimulationOptions()))
