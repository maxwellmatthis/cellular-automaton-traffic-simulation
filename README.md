# Cellular Automaton Traffic Simulation

A cellular automaton for traffic simulation based on the Nagel-Schreckenberg model. (See: [Nagel-Schreckenberg model (Wikipedia/DE)](https://en.wikipedia.org/wiki/Nagel–Schreckenberg_model), [Nagel-Schreckenberg-Modell (Wikipedia/DE)](https://de.wikipedia.org/wiki/Nagel-Schreckenberg-Modell)) Extended to support multiple lanes, construction sites, traffic lights, different vehicle types and more.

## Table of Contents

- [Installation & Setup](#installation--setup)
  - [Simulator](#simulator)
  - [Benchmark Tools](#benchmark-tools)
- [Usage](#usage)
  - [Simulator](#simulator-1)
  - [Benchmarking](#benchmarking)
- [Model](#model)
  - [Basics](#basics)
  - [Update Rules](#update-rules)
  - [Multi Lane Extension](#multi-lane-extension)
  - [Cell Blocking Extension](#cell-blocking-extension)
  - [Traffic Light Extension](#traffic-light-extension)

## Installation & Setup

### Simulator

The simulator is written in [Rust](https://rust-lang.org/learn/get-started) and can be compiled and run using Rust's package manager `cargo`.

```sh
# Run the simulator and show the usage
cargo run -- -h

# Build the simulator binary to use without cargo
cargo build --release
# Run the release binary and show the usage
target/release/cellular-automaton-traffic-simulation -h
```

### Benchmark Tools

The benchmarking tools are written in Python, mainly to take advantage of matplotlib.
You'll need to have Python3 (`>=3.11`), as well as matplotlib and numpy installed on your system.

```sh
# Quickly install requirements
pip install -r benchmarks/requirements.txt
```

__Note:__ The benchmarking tools can run the simulator through cargo or directly.
The default is running the release binary directly, howevery, you'll need to compile it first using
`cargo build --release`. Directly running the release binary is recommended as its faster and
doesn't print every time like cargo does. If you'd like to run the simulator through cargo, go to
benchmarks/adapter.py:6 and change the comment.

## Usage

### Simulator

Every constant used in the simulation has a reasonable default value but can also be set
when running the simulator from the command line (see usage below) or from a yaml definition using
the `-y` argument. __Note:__ when using a yaml definition, all arguments must be manually set. (See
[example.yaml](example.yaml))

The simulator can print the details of each round to stdout using the `-v` (verbose) switch or in an
animated way using the `-a` (animate) switch and generate an image using the `-i` (image) switch.
__Tip:__ In image mode cars are represented as pixels ranging from red (stopped) to green (max_speed).
Each row represents a round. The image is read from the bottom up.

The simulator always ends the simulation by printing relevant settings and useful
metrics about the simulation as JSON.

```sh
Usage: cellular-automaton-traffic-simulation [OPTIONS]

Options:
  -r, --rounds <ROUNDS>
          The number of rounds to run the simulation for [default: 4096]
      --lanes <LANES>
          The number of lanes that make up the road [default: 1]
  -l, --length <LENGTH>
          The number of cells in each lane that make up the road [default: 1000]
      --vehicles <VEHICLES>
          Allows specifying different vehicle types and with which density they occur. Format: `(max_speed, acceleration_time, traffic_density); ...` Corresponding model with units: `(x * 7.5m/s, (1 / x) * 7.5m/s^2, x * 100% of road on lane-by-lane basis)` [default: "(5, 1, 0.2)"]
  -d, --dilly-dally-probability <DILLY_DALLY_PROBABILITY>
          The probability with which cars dilly-dally. (slow down randomly) [default: 0.2]
  -s, --stay-in-lane-probability <STAY_IN_LANE_PROBABILITY>
          The probability with which cars stay in their lane, even when it would be best to switch lanes [default: 0.2]
      --monitor <MONITOR>
          The locations, specified as `(lane_index, cell_index); ...`, of the cells that are to be monitored. (Note: all cells are passively monitored but only those specified will be added to the simulation result [default: (0,0)]
      --traffic-lights <TRAFFIC_LIGHTS>
          The locations, specified as `(lane_index, cell_index); ...`, of the cells that represent traffic lights. Traffic lights will be green for 100 rounds and then be red for 100 rounds [default: ]
      --block <BLOCK>
          The locations, specified as `(lane_index, cell_index_start - cell_index_end_exclusive); ...` or `(lane_index, cell_index); ...`, of the cells that may not be driven over. This simulates blockages as they occur when construction work is being done [default: ]
  -v, --verbose
          Whether to print the states of the road to stdout
  -a, --animate
          Whether to print the states of the road to stdout using color and overwriting for greater viewing pleasure. This option trumps the `verbose` option
  -i, --image
          Whether to create a visualization image of the simulation
  -o, --out-path <OUT_PATH>
          Where to save the visualization image [default: traffic.png]
  -y, --yaml <YAML>
          Optionally provide simulator settings as a yaml file to avoid using the command line for detailed simulations. Note: All Options except `yaml` must be used!
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
    metrics = run_average(SimulationOptions(vehicles=[(5, 1, 0.3)], dilly_dally_probability=dilly_dally_probability), SIMULATIONS_EACH)
    average_speeds.append(metrics.average_speed_kilometers_per_hour)
    first_cell_flows.append(metrics.monitor_cells_flow_cars_per_minute[0])
    accelerations.append(metrics.average_accelerations_n_per_car_per_round)
    deaccelerations.append(metrics.average_accelerations_n_per_car_per_round)

plot(VARIABLE, "Average Speed (km/h)", dilly_dally_probabilities, average_speeds)
plot(VARIABLE, "First Cell Flow (car/min)", dilly_dally_probabilities, first_cell_flows)
plot(VARIABLE, "Accelerations (n/car/round)", dilly_dally_probabilities, accelerations)
plot(VARIABLE, "Deaccelerations (n/car/round)", dilly_dally_probabilities, deaccelerations)
```

The results from all benchmarks are stored in [benchmarks/results/](./benchmarks/results/).

## Model

### Basics

- The road is a closed loop, which means that the number of cars is constant and driving forever is possible.
- The road is a made up of cells, where each cell may contain exactly one or no car.
- Cars are `7.5m` long. => Each cell is `7.5m` long.
- Each round is 1s long.
- Cars can move a natural number of cells (equal to their speed) each round. => Cars move at `n * 7.5m/s` (`n * 27km/h`).
- The default maximum speed is set to `5cells/round`, although it can be set to any number. (Note: It does not make sense to set the maximum speed any higher than `10cells/round` (`10 * 27km/h => 270km/h`) since there are almost no cars that can reach and almost no drivers willing to pay for the gasoline needed to sustain such speeds.)

### Update Rules

The following steps are executed in order for each car each round.

1. Increase speed by `7.5m/s`.
2. Decrease speed to `cells_to_next_car * 7.5m/s`.
3. Decrease speed by `7.5m/s` with a chance of `dilly_dally_probability`.

### Multi-Lane Extension

The multilane extension adds support for multiple lanes and lane switching to the model.

- Cars can only switch to adjacent lanes.
- Switching is only allowed if there is no one (1) directly in front of or (2) next to the car. Exception: Switching is with a car directly in front is allowed for cars moving at `1cell/round`.
- Passing directly on the right is not allowed.
- Cars that switch lanes do not dilly-dally. (This avoids cars going sideways by switching lanes at `v=0cells/round` and break checking people behind them.)
- Cars may stay in their lane `stay_in_lane_probability * 100`% of the time. This models how drivers forget or choose not to switch lanes when they have the chance and should.

- Cars always switch to the right lane if there is enough space (speed + 1 cells) for them to drive without slowing down.
- Cars always switch to the lane with the most space if none of the lanes have enough space to drive without slowing down.

Since all cars theoretically move at the same time but it is very hard to make the computer
simulate all cars at the same time, the cars are simulated lane-by-lane, starting on the left.
This functions without hard-to-resolve conflicts, because passing on the right is not allowed.

__Examples:__

The following examples show the options and behaviour of the red car (`v=5cells/round`) for one round. The columns represent lanes 1-4.

(Legend: 🚙 : other car, ❌ : not allowed, ✅ : allowed, 🎯 : where the car will move to)

| 1 | 2 | 3 | 4 |
| --- | --- | --- | --- |
|❌ |🚙 |❌ |🚙 |
|❌ |🚙 |❌ |🚙 |
|❌ |🚙 |❌ |❌ |
|❌ |🚙 |❌ |❌ |
|❌ |🚙 |❌ |❌ |
|❌ |🚗 |❌ |❌ |

| 1 | 2 | 3 | 4 |
| --- | --- | --- | --- |
|🎯 |🚙 |🚙 |🚙 |
|✅ |🚙 |🚙 |🚙 |
|✅ |❌ |❌ |❌ |
|✅ |🚙 |✅ |❌ |
|✅ |✅ |✅ |❌ |
|❌ |🚗 |❌ |❌ |

| 1 | 2 | 3 | 4 |
| --- | --- | --- | --- |
|❌ |🚙 |🚙 |🚙 |
|❌ |🚙 |🎯 |❌ |
|❌ |✅ |✅ |❌ |
|🚙 |🚗 |✅ |❌ |
|❌ |❌ |🚙 |❌ |

| 1 | 2 | 3 | 4 |
| --- | --- | --- | --- |
|❌ |❌ |🚙 |❌ |
|❌ |❌ |❌ |❌ |
|✅ |✅ |🎯 |❌ |
|✅ |✅ |✅ |❌ |
|✅ |✅ |✅ |❌ |
|✅ |✅ |✅ |❌ |
|✅ |✅ |✅ |❌ |
|❌ |🚗 |❌ |❌ |

| 1 | 2 | 3 | 4 |
| --- | --- | --- | --- |
|❌ |❌ |🚙 |❌ |
|✅ |🎯 |✅ |❌ |
|✅ |✅ |✅ |❌ |
|✅ |✅ |✅ |❌ |
|✅ |✅ |✅ |❌ |
|✅ |✅ |✅ |❌ |
|❌ |🚗 |❌ |❌ |

### Cell Blocking Extension

The lane blocking extension adds the option to block individual cells or ranges of cells. The feature can be used to simulate a construction site or accident.

### Traffic Light Extension

The traffic light extension add traffic lights to the model. All traffic lights turn red and green at the same time. Switching occurs every 100 model seconds (100 simulation rounds).
