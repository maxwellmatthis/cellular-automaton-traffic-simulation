# Benchmark Results

This directory contains graphs of some benchmarks that show how the model behaves under different circumstances.

## Interesting findings

Disclaimer: These are just notes that may still need to be confirmed.

- There is a local maximum near `(max_speed: 5*27km/h, traffic_density: 30%, z)` It becomes less significant with rising dilly-dally-probability. => The "Richtgeschwindigkeit" in Germany is `4.8*27km/h`.
- Flow is best at `traffic_density=0.5`. Flow is optimal for `dilly_dally_probability=0` and `max_speed=1` (Pattern: `[car, empty, car, empty, ...]`). However, flow is not optimal for `traffic_density=0.6` and `max_speed=2` (Pattern: [car, empty, empty, car, ...]).

