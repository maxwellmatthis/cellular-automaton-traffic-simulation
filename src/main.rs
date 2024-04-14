use std::time::Instant;
use std::path::PathBuf;
use road::Road;
use image_drawer::ImageDrawer;
use clap::Parser;
use serde::Serialize;

mod road;
mod cell;
mod car;
mod image_drawer;
mod flip_flop;

const CELL_M: f64 = 7.5;
const ROUND_S: f64 = 1.0;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The number of rounds to run the simulation for.
    #[arg(short, long, default_value_t = 4096)]
    rounds: u32,

    /// The number of cells that make up the road.
    #[arg(short, long, default_value_t = 1000)]
    length: u32,

    /// The maximum number of cells that a car can drive in a round.
    #[arg(short, long, default_value_t = 5)]
    max_speed: u8,

    /// The density of traffic. Number of cars on the road are computed as `floor(traffic_density * road_length)`.
    #[arg(short, long, default_value_t = 0.5)]
    traffic_density: f32,

    /// The probability with which cars dilly-dally. (slow down randomly)
    #[arg(short, long, default_value_t = 0.2)]
    dilly_dally_probability: f32,

    /// The indexes of the cells that are to be monitored. (Note: Although all cells are always
    /// monitored, only the cells you specify here will be included in the simulation metrics at
    /// the end on the simulation.)
    #[arg(long, value_delimiter = ',', default_value = "0")]
    monitor: Vec<u32>,

    /// Whether to print the current road state to stdout.
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Whether to create a visualization image of the simulation.
    #[arg(short, long, default_value_t = false)]
    image: bool,

    /// Where to save the visualization image.
    #[arg(short, long, default_value = "traffic.png")]
    out_path: PathBuf
}

fn main() {
    let args = Args::parse();
    println!("{}", run_sim(args).json());
}

#[derive(Serialize)]
pub struct SimulationResult {
    // Settings
    pub rounds: u32,
    pub length: u32,
    pub max_speed: u8,
    pub traffic_density: f32,
    pub cars: u32,
    pub dilly_dally_probability: f32,
    // Metrics
    pub runtime_s: f64,
    pub average_speed_kilometers_per_hour: f64,
    pub monitor_cells_flow_cars_per_minute: Vec<f64>,
    pub average_accelerations_n_per_car_per_round: f64,
    pub average_deaccelerations_n_per_car_per_round: f64
}

impl SimulationResult {
    pub fn json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub fn run_sim(args: Args) -> SimulationResult {
    let start = Instant::now();
    let mut road = Road::new(
        args.length,
        args.max_speed,
        args.traffic_density,
        args.dilly_dally_probability,
    );
    if args.verbose { println!("{}", road); }

    let mut image_drawer = if args.image {
        ImageDrawer::new(&road, args.rounds + 1)
    } else {
        ImageDrawer::placeholder()
    };
    if args.image { image_drawer.add_snapshot(&road); }

    // run simulator
    for _ in 0..args.rounds {
        road.round();
        if args.verbose { println!("{}", road); }
        if args.image { image_drawer.add_snapshot(&road); }
    }

    if args.image { image_drawer.save(args.out_path).unwrap(); }

    let flows_cars_per_minute: Vec<f64> = args.monitor
        .iter()
        .map(|i| *i as usize)
        .filter(|i| i < &road.cells().len())
        .map(|i| road.cells()[i].flow(args.rounds) / ROUND_S * 60.0)
        .collect();
    SimulationResult {
        // Settings
        rounds: args.rounds,
        length: args.length,
        max_speed: args.max_speed,
        traffic_density: args.traffic_density,
        cars: road.cars(),
        dilly_dally_probability: args.dilly_dally_probability,
        // Metrics
        runtime_s: start.elapsed().as_secs_f64(),
        average_speed_kilometers_per_hour: road.average_speed() * (CELL_M / ROUND_S) * 3.6,
        monitor_cells_flow_cars_per_minute: flows_cars_per_minute,
        average_accelerations_n_per_car_per_round: road.average_accelerations(),
        average_deaccelerations_n_per_car_per_round: road.average_deaccelerations()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{run_sim, Args, CELL_M, ROUND_S};

    #[test]
    fn no_road() {
        let result = run_sim(Args {
            rounds: 100,
            length: 0,
            max_speed: 5,
            traffic_density: 0.5,
            dilly_dally_probability: 0.2,
            monitor: vec![0, 100],
            verbose: false,
            image: false,
            out_path: PathBuf::new()
        });

        assert!(result.average_speed_kilometers_per_hour.is_nan());
        assert!(result.average_accelerations_n_per_car_per_round.is_nan());
        assert!(result.average_deaccelerations_n_per_car_per_round.is_nan());
    }

    #[test]
    fn default_simulation() {
        let result = run_sim(Args {
            rounds: 4096,
            length: 1000,
            max_speed: 5,
            traffic_density: 0.5,
            dilly_dally_probability: 0.2,
            monitor: vec![0, 500, 999],
            verbose: false,
            image: false,
            out_path: PathBuf::new()
        });

        assert_eq!(result.cars, 500);
    }

    #[test]
    fn one_car() {
        let result = run_sim(Args {
            rounds: 10,
            length: 10,
            max_speed: 5,
            traffic_density: 0.1,
            dilly_dally_probability: 0.0,
            monitor: vec![0, 500, 999],
            verbose: false,
            image: false,
            out_path: PathBuf::new()
        });

        assert_eq!(result.cars, 1);
        assert_eq!(
            result.average_speed_kilometers_per_hour,
            (1+2+3+4+5+(10-5)*5) as f64 / 10.0 * (CELL_M / ROUND_S) * 3.6
        );
        assert_eq!(
            result.average_accelerations_n_per_car_per_round,
            5.0 / 10.0
        );
        assert_eq!(
            result.average_deaccelerations_n_per_car_per_round,
            0.0
        );
    }
}

