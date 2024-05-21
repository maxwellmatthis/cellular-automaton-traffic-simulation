use std::fmt::Debug;
use std::time::{Duration, Instant};
use std::str::FromStr;
use std::path::PathBuf;
use std::thread;
use car::VehicleBlueprint;
use cell::CellLocationRange;
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

    /// Allows specifying different vehicle types and with which density they occur.
    /// Format: `(max_speed, acceleration_time, traffic_density); ...`
    /// Corresponding model with units: `(x * 7.5m/s, (1 / x) * 7.5m/s^2, x * 100% of road on lane-by-lane
    /// basis)`
    #[arg(long, value_delimiter = ';', default_value = "(5, 1, 0.2)")]
    vehicles: Vec<String>,

    /// The probability with which cars dilly-dally. (slow down randomly)
    #[arg(short, long, default_value_t = 0.2)]
    dilly_dally_probability: f32,

    /// The probability with which cars stay in their lane, even when it would be best to switch lanes.
    #[arg(short, long, default_value_t = 0.2)]
    stay_in_lane_probability: f32,

    /// The locations, specified as `(lane_index, cell_index); ...`, of the cells that are to be monitored.
    /// (Note: all cells are passively monitored but only those specified will be added to the simulation
    /// result.
    #[arg(long, value_delimiter = ';', default_value = "(0,0)")]
    monitor: Vec<String>,

    /// The locations, specified as `(lane_index, cell_index); ...`, of the cells that represent
    /// traffic lights. Traffic lights will be green for 100 rounds and then be red for 100 rounds.
    #[arg(long, value_delimiter = ';', default_value = "")]
    traffic_lights: Vec<String>,

    /// The locations, specified as `(lane_index, cell_index_start - cell_index_end_exclusive); ...`
    /// or `(lane_index, cell_index); ...`, of the cells that may not be driven over. This simulates
    /// blockages as they occur when construction work is being done.
    #[arg(long, value_delimiter = ';', default_value = "")]
    block: Vec<String>,

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
    pub fn deserialize_tuple_type<D: FromStr>(stringified_tuples: &Vec<String>) -> Vec<D> where <D as FromStr>::Err: Debug {
        let mut tuples = Vec::new();
        for string in stringified_tuples {
            if string.is_empty() { continue; }
            tuples.push(string.parse::<D>().unwrap());
        }
        tuples
    }

    pub fn vehicles(&self) -> Vec<VehicleBlueprint> {
        Self::deserialize_tuple_type(&self.vehicles)
    }

    pub fn monitor(&self) -> Vec<CellLocation> {
        Self::deserialize_tuple_type(&self.monitor)
    }

    pub fn block(&self) -> Vec<CellLocationRange> {
        Self::deserialize_tuple_type(&self.block)
    }

    pub fn traffic_lights(&self) -> Vec<CellLocation> {
        Self::deserialize_tuple_type(&self.traffic_lights)
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
    // Parse data here so that the program fails immediately if anything is wrong.
    let args_vehicles = args.vehicles();
    let args_monitors = args.monitor();
    let args_block = args.block();
    let args_traffic_lights = args.traffic_lights();

    // setup
    let start = Instant::now();
    let mut road = Road::new(
        args.lanes,
        args.length,
        &args_vehicles,
        args.dilly_dally_probability,
        args.stay_in_lane_probability,
        &args_block,
        &args_traffic_lights,
    );

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
            thread::sleep(Duration::from_millis(20));
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

    let flows_cars_per_minute = args_monitors
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
    use std::{path::PathBuf, str::FromStr};

    use crate::{run_sim, Args, CELL_M, ROUND_S};

    // -- simple simulation --

    #[test]
    fn no_road() {
        let result = run_sim(Args {
            rounds: 100,
            lanes: 0,
            length: 0,
            vehicles: vec!["(5, 1, 0.5)".to_string()],
            dilly_dally_probability: 0.2,
            stay_in_lane_probability: 0.0,
            monitor: vec!["(24,1000)".to_string()], // invalid monitors result in f64::NAN
            block: vec![],
            traffic_lights: vec![],
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
            vehicles: vec!["(5, 1, 0.5)".to_string()],
            dilly_dally_probability: 0.2,
            stay_in_lane_probability: 0.0,
            monitor: vec!["(0,0)".to_string(), "(0,500)".to_string(), "(0,999)".to_string()],
            block: vec![],
            traffic_lights: vec![],
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
            vehicles: vec!["(5, 1, 0.1)".to_string()],
            dilly_dally_probability: 0.0,
            stay_in_lane_probability: 0.0,
            monitor: vec!["(0,0)".to_string()],
            block: vec![],
            traffic_lights: vec![],
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

    // -- multilane extension --

    #[test]
    fn three_cars_three_lanes_no_switches() {
        let result = run_sim(Args {
            rounds: 10,
            lanes: 3,
            length: 10,
            vehicles: vec!["(5, 1, 0.1)".to_string()],
            dilly_dally_probability: 0.0,
            stay_in_lane_probability: 1.0,
            monitor: vec!["(0,0)".to_string(), "(1,0)".to_string(), "(2,0)".to_string()],
            block: vec![],
            traffic_lights: vec![],
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
            vehicles: vec!["(2, 1, 0.1)".to_string()],
            dilly_dally_probability: 0.1,
            stay_in_lane_probability: 0.0,
            monitor: {
                let mut mon = Vec::new();
                for lane in 4..=8 {
                    mon.push(format!("({},0)", lane));
                }
                mon
            },
            block: vec![],
            traffic_lights: vec![],
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

    // -- multilane extension with blockages --

    #[test]
    fn single_lane_full_blockage() {
        let result = run_sim(Args {
            rounds: 10,
            lanes: 1,
            length: 10,
            vehicles: vec!["(5, 1, 0.1)".to_string()],
            dilly_dally_probability: 0.0,
            stay_in_lane_probability: 0.0,
            monitor: vec!["(0,0)".to_string()],
            block: vec!["(0,0)".to_string()],
            traffic_lights: vec![],
            verbose: true,
            image: false,
            animate: false,
            out_path: PathBuf::new()
        });

        println!("{:?}", result);

        assert_eq!(result.cars, 1);
        assert_eq!(result.monitor_cells_flow_cars_per_minute[0], 0.0);
    }

    #[test]
    fn left_lane_full_blockage() {
        // This test is the same as `one_car`, except that there is a second lane
        // that is totally blocked.
        let result = run_sim(Args {
            rounds: 10,
            lanes: 2,
            length: 10,
            vehicles: vec!["(5, 1, 0.1)".to_string()],
            dilly_dally_probability: 0.0,
            stay_in_lane_probability: 0.0,
            monitor: vec!["(0,0)".to_string()],
            block: vec!["(0,0-10)".to_string()],
            traffic_lights: vec![],
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
    fn ultra_bottleneck() {
        let _result = run_sim(Args {
            rounds: 100,
            lanes: 10,
            length: 100,
            vehicles: vec!["(5, 1, 0.3)".to_string()],
            dilly_dally_probability: 0.0,
            stay_in_lane_probability: 0.0,
            monitor: vec![],
            block: {
                let mut blk = Vec::new();
                for lane in 0..=7 {
                    blk.push(format!("({},{}-100)", lane, 90 + lane));
                }
                for lane in 2..=9 {
                    blk.push(format!("({},{}-30)", lane, 20 + (9 - lane)));
                }
                for lane in 4..=6 {
                    blk.push(format!("({},50-60)", lane));
                }
                blk
            },
            traffic_lights: vec![],
            verbose: true,
            animate: false,
            image: false,
            out_path: PathBuf::from_str("traffic-ultra_bottleneck.png").unwrap()
        });

        // This test is too confusing to write comprehensive tests for. It's enough for me if
        // nothing in the simulator itself panics.
        // Just uncomment the following explicit `panic!` and set `Args.image` to `true` and witness
        // the chaos unfold:
        // panic!();
    }

    // -- different vehicle types extension --

    #[test]
    fn slow_truck_causing_traffic_jam() {
        let result = run_sim(Args {
            rounds: 100,
            lanes: 1,
            length: 100,
            vehicles: vec!["(4, 6, 0.01)".to_string(), "(5, 1, 0.2)".to_string()],
            dilly_dally_probability: 0.0,
            stay_in_lane_probability: 0.0,
            monitor: vec!["(0,0)".to_string()],
            block: vec![],
            traffic_lights: vec![],
            verbose: true,
            image: false,
            animate: false,
            out_path: PathBuf::from_str("traffic-slow_truck.png").unwrap()
        });

        println!("{:?}", result);

        assert!(result.monitor_cells_flow_cars_per_minute[0] > 0.0);
    }
    
    #[test]
    fn bunch_of_trucks() {
        let result = run_sim(Args {
            rounds: 100,
            lanes: 1,
            length: 100,
            vehicles: vec!["(4, 6, 0.3)".to_string()],
            dilly_dally_probability: 0.0,
            stay_in_lane_probability: 0.0,
            monitor: vec!["(0,0)".to_string()],
            block: vec![],
            traffic_lights: vec![],
            verbose: true,
            image: false,
            animate: false,
            out_path: PathBuf::from_str("traffic-bunch_of_truck.png").unwrap()
        });

        println!("{:?}", result);

        assert!(result.monitor_cells_flow_cars_per_minute[0] > 0.0);
    }

    #[test]
    #[should_panic]
    fn sum_of_densities_cannot_be_greater_than_1() {
        let result = run_sim(Args {
            rounds: 100,
            lanes: 1,
            length: 100,
            vehicles: vec!["(4, 6, 0.3)".to_string(), "(5, 1, 0.8)".to_string()],
            dilly_dally_probability: 0.0,
            stay_in_lane_probability: 0.0,
            monitor: vec![],
            block: vec![],
            traffic_lights: vec![],
            verbose: true,
            image: false,
            animate: false,
            out_path: PathBuf::new()
        });

        println!("{:?}", result);
    }

    // -- traffic lights --

    #[test]
    fn single_lane_traffic_light() {
        let result = run_sim(Args {
            rounds: 200,
            lanes: 1,
            length: 10,
            vehicles: vec!["(5, 1, 0.1)".to_string()],
            dilly_dally_probability: 0.0,
            stay_in_lane_probability: 0.0,
            monitor: vec!["(0,0)".to_string()],
            block: vec![],
            traffic_lights: vec!["(0, 9)".to_string()],
            verbose: true,
            image: false,
            animate: false,
            out_path: PathBuf::new()
        });

        println!("{:?}", result);

        assert_eq!(result.cars, 1);
        assert!(
            result.average_speed_kilometers_per_hour -
            (1+2+3+4+5+(100-5)*5) as f64 / 200.0 * (CELL_M / ROUND_S) * 3.6
            < 2.0
        );
    }
}

