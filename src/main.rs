use std::time::Instant;
use road::Road;
use image_drawer::ImageDrawer;
use clap::Parser;
use json::*;

mod road;
mod cell;
mod car;
mod image_drawer;

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
    length: usize,

    /// The maximum number of cells that a car can drive in a round.
    #[arg(short, long, default_value_t = 5)]
    max_speed: u8,

    /// The probability with which the initial cars are placed.
    #[arg(long, default_value_t = 0.5)]
    place_car_probability: f32,

    /// The probability with which cars dilly-dally. (slow down randomly)
    #[arg(long, default_value_t = 0.5)]
    dilly_dally_probability: f32,

    /// The probability with which a new car is spawned in the zeroth cell of the highway. Note:
    /// The cell must be clear.
    #[arg(long, default_value_t = 0.5)]
    spawn_car_at_entrance_probability: f32,

    /// The probability with which an existing car is removed when it passes the last cell of the
    /// highway. Colisions with cars in the first cells are ignored, unlike with the usual
    /// wrap-around.
    #[arg(long, default_value_t = 0.5)]
    remove_car_on_exit_probability: f32,

    /// Whether to print the current road state to stdout.
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Where to save the visualization image.
    #[arg(short, long, default_value = "traffic.png")]
    out: String
}

fn main() {
    let args = Args::parse();

    let start = Instant::now();
    let mut road = Road::new(
        args.length,
        args.max_speed,
        args.place_car_probability,
        args.dilly_dally_probability,
        args.spawn_car_at_entrance_probability,
        args.remove_car_on_exit_probability
    );
    let mut image_drawer = ImageDrawer::new(&road, args.rounds + 1);
    image_drawer.add_snapshot(&road);
    if args.verbose { println!("{}", road); }
    for _ in 0..args.rounds {
        road.round();
        image_drawer.add_snapshot(&road);
        if args.verbose { println!("{}", road); }
    }
    image_drawer.save(args.out).unwrap();
    println!("{}", object!{
        // Settings
        rounds: args.rounds,
        max_speed: args.max_speed,
        dilly_dally_probability: args.dilly_dally_probability,
        place_car_probability: args.place_car_probability,
        spawn_car_at_entrance_probability: args.spawn_car_at_entrance_probability,
        remove_car_on_exit_probability: args.remove_car_on_exit_probability,
        // Metrics
        runtime: format!("{:?}", start.elapsed()),
        average_speed__kilometers_per_hour: road.average_speed() * CELL_M / ROUND_S * 3.6,
        exit_cell_flow__cars_per_minute: road.cells()[args.length - 1].flow(args.rounds) / ROUND_S * 60.0,
        accelerations: road.accelerations(),
        deaccelerations: road.deaccelerations()
    }.dump());
}

