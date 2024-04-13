use std::time::Instant;
use std::path::PathBuf;
use road::Road;
use image_drawer::ImageDrawer;
use clap::Parser;
use json::*;

mod road;
mod cell;
mod car;
mod image_drawer;
mod flip_flop;

const CELL_M: f64 = 7.5;
const ROUND_S: f64 = 1.0;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
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
    #[arg(long, default_value_t = 0.5)]
    traffic_density: f32,

    /// The probability with which cars dilly-dally. (slow down randomly)
    #[arg(long, default_value_t = 0.2)]
    dilly_dally_probability: f32,

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

    let start = Instant::now();
    let mut road = Road::new(
        args.length,
        args.max_speed,
        args.traffic_density,
        args.dilly_dally_probability,
    );
    let mut image_drawer = if args.image {
        ImageDrawer::new(&road, args.rounds + 1)
    } else {
        ImageDrawer::placeholder()
    };
    if args.image { image_drawer.add_snapshot(&road); }
    if args.verbose { println!("{}", road); }
    for _ in 0..args.rounds {
        road.round();
        if args.image { image_drawer.add_snapshot(&road); }
        if args.verbose { println!("{}", road); }
    }
    if args.image { image_drawer.save(args.out_path).unwrap(); }
    println!("{}", object!{
        // Settings
        rounds: args.rounds,
        max_speed: args.max_speed,
        traffic_density: args.traffic_density,
        cars: road.cars(),
        dilly_dally_probability: args.dilly_dally_probability,
        // Metrics
        runtime__s: start.elapsed().as_secs_f64(),
        average_speed__kilometers_per_hour: road.average_speed() * (CELL_M / ROUND_S) * 3.6,
        exit_cell_flow__cars_per_minute: road.cells()[road.cells().len() - 1].flow(args.rounds) / ROUND_S * 60.0,
        average_accelerations__n_per_car_per_round: road.average_accelerations(),
        average_deaccelerations__n_per_car_per_round: road.average_deaccelerations()
    }.dump());
}

