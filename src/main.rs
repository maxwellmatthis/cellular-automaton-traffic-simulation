use std::fmt::Debug;
use std::time::{Duration, Instant};
use std::str::FromStr;
use std::path::PathBuf;
use std::thread;
use road::Road;
use image_drawer::ImageDrawer;
use clap::Parser;
use serde::Serialize;
use std::io::{Write, stdout};
use crossterm::{QueueableCommand, cursor, terminal, ExecutableCommand};
use crate::cell::CellLocation;

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

    /// The number of lanes that make up the road.
    #[arg(long, default_value_t = 1)]
    lanes: u32,

    /// The number of cells in each lane that make up the road.
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

    /// The probability with which cars stay in their lane, even when it would be best to switch lanes.
    #[arg(short, long, default_value_t = 0.2)]
    stay_in_lane_probability: f32,

    /// The locations, specified as `(lane_index,cell_index); ...`, of the cells that are to be monitored.
    /// (Note: all cells are passively monitored but only those specified will be added to the simulation
    /// result.
    #[arg(long, value_delimiter = ';', default_value = "(0,0)")]
    monitor: Vec<String>,

    /// Whether to print the states of the road to stdout.
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Whether to print the states of the road to stdout using color and overwriting for greater
    /// viewing pleasure. This option trumps the `verbose` option.
    #[arg(short, long, default_value_t = false)]
    animate: bool,
 
    /// Whether to create a visualization image of the simulation.
    #[arg(short, long, default_value_t = false)]
    image: bool,

    /// Where to save the visualization image.
    #[arg(short, long, default_value = "traffic.png")]
    out_path: PathBuf
}

impl Args {
    /// Deserializes stringified_tuples that were provided as arguments.
    /// Note: This method assumes that the parenthesis are each one byte long. Beware of UTF-8
    /// characters in those positions.
    pub fn deserialize_tuple_type<D: FromStr>(stringified_tuples: Vec<String>) -> Vec<D> where <D as FromStr>::Err: Debug {
        let mut tuples = Vec::new();
        for string in stringified_tuples {
            tuples.push(string.parse::<D>().unwrap());
        }
        tuples
    }
}

fn main() {
    let args = Args::parse();
    println!("{}", run_sim(args).json());
}

#[derive(Serialize, Debug)]
pub struct SimulationResult {
    // Settings
    pub rounds: u32,
    pub lanes: u32,
    pub length: u32,
    pub max_speed: u8,
    pub traffic_density: f32,
    pub cars: u32,
    pub dilly_dally_probability: f32,
    pub stay_in_lane_probability: f32,
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
    // setup
    let start = Instant::now();
    let mut road = Road::new(
        args.lanes,
        args.length,
        args.max_speed,
        args.traffic_density,
        args.dilly_dally_probability,
        args.stay_in_lane_probability,
    );

    let monitors = Args::deserialize_tuple_type::<CellLocation>(args.monitor);

    // setup outputs
    if !args.animate && args.verbose { println!("{}", road); }
    let mut stdout = stdout();
    if args.animate { stdout.execute(cursor::Hide).unwrap(); }
    let mut image_drawer = if args.image {
        ImageDrawer::new(&road, args.rounds + 1)
    } else {
        ImageDrawer::placeholder()
    };
    if args.image { image_drawer.take_snapshot(&road); }

    // run simulator
    for _ in 0..args.rounds {
        road.round();
        if args.animate {
            stdout.queue(cursor::SavePosition).unwrap();
            stdout.write_all(format!("{}", road).as_bytes()).unwrap();
            stdout.queue(cursor::RestorePosition).unwrap();
            stdout.flush().unwrap();
            thread::sleep(Duration::from_millis(200));
            stdout.queue(cursor::RestorePosition).unwrap();
            stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
        } else if args.verbose {
            println!("\n{}", road);
        }
        if args.image { image_drawer.take_snapshot(&road); }
    }
    // clean-up
    if args.animate {
        stdout.execute(cursor::Show).unwrap();
        println!("{}", road);
    }
    if args.image { image_drawer.save(args.out_path).unwrap(); }

    let flows_cars_per_minute = monitors
        .iter()
        .map(|cl| {
            if cl.lane() >= road.lanes() as usize || cl.index() >= road.length() as usize {
                f64::NAN
            } else {
                road.cells()[cl.lane()][cl.index()].flow(args.rounds) / ROUND_S * 60.0
            }
        })
        .collect();

    SimulationResult {
        // Settings
        rounds: road.rounds(),
        lanes: road.lanes(),
        length: road.length(),
        max_speed: args.max_speed,
        traffic_density: args.traffic_density,
        cars: road.cars(),
        dilly_dally_probability: road.dilly_dally_probability(),
        stay_in_lane_probability: road.stay_in_lane_probability(),
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
            lanes: 0,
            length: 0,
            max_speed: 5,
            traffic_density: 0.5,
            dilly_dally_probability: 0.2,
            stay_in_lane_probability: 0.0,
            monitor: vec!["(24,1000)".to_string()], // invalid monitors result in f64::NAN
            verbose: true,
            image: false,
            animate: false,
            out_path: PathBuf::new()
        });

        println!("{:?}", result);

        assert!(result.average_speed_kilometers_per_hour.is_nan());
        assert!(result.average_accelerations_n_per_car_per_round.is_nan());
        assert!(result.average_deaccelerations_n_per_car_per_round.is_nan());
        assert!(result.monitor_cells_flow_cars_per_minute[0].is_nan());
    }

    #[test]
    fn default_simulation() {
        let result = run_sim(Args {
            rounds: 4096,
            lanes: 1,
            length: 1000,
            max_speed: 5,
            traffic_density: 0.5,
            dilly_dally_probability: 0.2,
            stay_in_lane_probability: 0.0,
            monitor: vec!["(0,0)".to_string(), "(0,500)".to_string(), "(0,999)".to_string()],
            verbose: false,
            image: false,
            animate: false,
            out_path: PathBuf::new()
        });

        println!("{:?}", result);

        assert_eq!(result.cars, 500);
        assert!(result.monitor_cells_flow_cars_per_minute[0].is_finite());
    }

    #[test]
    fn one_car() {
        let result = run_sim(Args {
            rounds: 10,
            lanes: 1,
            length: 10,
            max_speed: 5,
            traffic_density: 0.1,
            dilly_dally_probability: 0.0,
            stay_in_lane_probability: 0.0,
            monitor: vec!["(0,0)".to_string()],
            verbose: true,
            image: false,
            animate: false,
            out_path: PathBuf::new()
        });

        println!("{:?}", result);

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

    #[test]
    fn three_cars_three_lanes_no_switches() {
        let result = run_sim(Args {
            rounds: 10,
            lanes: 3,
            length: 10,
            max_speed: 5,
            traffic_density: 0.1,
            dilly_dally_probability: 0.0,
            stay_in_lane_probability: 1.0,
            monitor: vec!["(0,0)".to_string(), "(1,0)".to_string(), "(2,0)".to_string()],
            verbose: true,
            image: false,
            animate: false,
            out_path: PathBuf::new()
        });

        println!("{:?}", result);

        assert_eq!(result.cars, 3);
        assert!(result.average_speed_kilometers_per_hour <= 108.0);
        assert!(result.average_speed_kilometers_per_hour >= 50.0);
        assert!(result.average_accelerations_n_per_car_per_round >= 0.5);
        assert_eq!(result.stay_in_lane_probability, 1.0);
    }

    #[test]
    fn slow_all_moving_over() {
        let result = run_sim(Args {
            rounds: 100,
            lanes: 10,
            length: 20,
            max_speed: 2,
            traffic_density: 0.1,
            dilly_dally_probability: 0.1,
            stay_in_lane_probability: 0.0,
            monitor: {
                let mut mon = Vec::new();
                for lane in 4..9 {
                    mon.push(format!("({},0)", lane));
                }
                mon
            },
            verbose: true,
            image: false,
            animate: false,
            out_path: PathBuf::new()
        });

        println!("{:?}", result);

        let mut last = 0.0;
        for val in result.monitor_cells_flow_cars_per_minute {
            assert!(last <= val);
            last = val;
        }
    }
}

