# Cellular Automaton Traffic Simulation

A primarily one-dimensional cellular automaton for traffic simulation based on the Nagel-Schreckenberg model. (See: [Nagel-Schreckenberg model (Wikipedia/DE)](https://en.wikipedia.org/wiki/Nagel–Schreckenberg_model), [Nagel-Schreckenberg-Modell (Wikipedia/DE)](https://de.wikipedia.org/wiki/Nagel-Schreckenberg-Modell))

## Installation & Setup

### Simulator

The simulator is written in [Rust](https://rust-lang.org/learn/get-started) and can be compiled and run using Rust's package manager `cargo`.

```sh
# Run the simulator and show the usage
cargo run -- -h

# Build the simulator binary to use without cargo
cargo build --release
```

### Benchmark Tools

The benchmarking tools are written in Python, mainly to take advantage of matplotlib.
You'll need to have Python3 (`>=3.11`), as well as matplotlib and numpy installed on your system.

```sh
# Quickly install requirements
pip install -r benchmarks/requirements.txt
```

## Usage

### Simulator

Every constant used in the simulation has a reasonable default value but can also be manually changed
when running the simulator from the command line (see usage below).

The simulator can print the details of each round to stdout using the `-v` (verbose) switch and/or
generate an image using the `-i` (image) switch.
__Note:__ The number in parenthesis in front of every line in verbose mode is the number of vehicles on the
road. In image mode cars are represented as pixels ranging from red (stopped) to green (max_speed).
Each row represents a round. The image is read from the bottom up.

The simulator always ends the simulation by printing the its simulation relevant settings and useful
metrics about the simulation as JSON.

```sh
Usage: cellular-automaton-traffic-simulation [OPTIONS]

Options:
  -r, --rounds <ROUNDS>
          The number of rounds to run the simulation for [default: 4096]
  -l, --length <LENGTH>
          The number of cells that make up the road [default: 1000]
  -m, --max-speed <MAX_SPEED>
          The maximum number of cells that a car can drive in a round [default: 5]
      --place-car-probability <PLACE_CAR_PROBABILITY>
          The probability with which the initial cars are placed [default: 0.5]
      --dilly-dally-probability <DILLY_DALLY_PROBABILITY>
          The probability with which cars dilly-dally. (slow down randomly) [default: 0.2]
      --spawn-car-at-entrance-probability <SPAWN_CAR_AT_ENTRANCE_PROBABILITY>
          The probability with which a new car is spawned in the zeroth cell of the highway. Note: The cell must be clear [default: 0]
      --remove-car-on-exit-probability <REMOVE_CAR_ON_EXIT_PROBABILITY>
          The probability with which an existing car is removed when it passes the last cell of the highway. Colisions with cars in the first cells are ignored, unlike with the usual wrap-around [default: 0]
  -v, --verbose
          Whether to print the current road state to stdout
  -i, --image
          Whether to create a visualization image of the simulation
  -o, --out-path <OUT_PATH>
          Where to save the visualization image [default: traffic.png]
  -h, --help
          Print help
  -V, --version
          Print version
```

Here's an example of what a simulation image looks like:

![traffic](https://github.com/maxwellmatthis/cellular-automaton-traffic-simulation/assets/58150536/c449c61a-d267-4255-8412-61ecf133157d)

### Benchmarking

The [python adapter](./benchmarks/adapter.py) provides interfaces and convenience functions that
run the simulator and return the metrics in a pythonic way. The
[plot helper](./benchmarks/plot_helper.py) provides simple utility functions for plotting metrics
using Matplotlib.

Here's an example of a script using both to benchmark what happens to the metrics as the maximum speed increases:

```python
# from: benchmarks/max_speed.py

import numpy as np
from adapter import run_average, SimulationOptions
from plot_helper import plot

VARIABLE = "Max Speed"
ROUNDS = 100

# x-axis
max_speeds = np.arange(0, 9, 1)

# y-axes
average_speeds = []
exit_cell_flows = []
accelerations = []
deaccelerations = []

for max_speed in max_speeds:
    # Runs the simulator `ROUNDS` times for the current max_speed and averages the results.
    metrics = run_average(SimulationOptions(max_speed=max_speed, place_car_probability=0.4, dilly_dally_probability=0.0), ROUNDS)
    average_speeds.append(metrics.average_speed__kilometers_per_hour)
    exit_cell_flows.append(metrics.exit_cell_flow__cars_per_minute)
    accelerations.append(metrics.accelerations)
    deaccelerations.append(metrics.deaccelerations)

plot(VARIABLE, "Average Speed (km/h)", max_speeds, average_speeds, ROUNDS)
plot(VARIABLE, "Exit Cell Flow (car/min)", max_speeds, exit_cell_flows, ROUNDS)
plot(VARIABLE, "Accelerations", max_speeds, accelerations, ROUNDS)
plot(VARIABLE, "Deaccelerations", max_speeds, deaccelerations, ROUNDS)
```

Here's an example of what a 2D plot looks like:

![Max_Speed:Exit_Cell_Flow_(car:min)_(100_rounds_each)](https://github.com/maxwellmatthis/cellular-automaton-traffic-simulation/assets/58150536/19253a33-7866-42ef-a9a4-486a57d4866e)
