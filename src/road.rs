use std::{cmp, fmt, isize};
use rand::prelude::*;
use crate::cell::{Cell, CellLocation, CellLocationRange, PutCarErrorInformation};
use crate::car::{Car, VehicleBlueprint};
use crate::flip_flop::FlipFlop;
use colored::Colorize;

#[derive(Debug)]
enum LaneSwitch {
    Left(u8),
    Right(u8),
    Stay(u8),
}

impl LaneSwitch {
    /// Returns the number of fields that are driveable (allowed as well as enough speed to be used)
    /// based on the lane switch.
    pub fn driveable(&self) -> u8 {
        match self {
            LaneSwitch::Left(cells) => *cells,
            LaneSwitch::Right(cells) => *cells,
            LaneSwitch::Stay(cells) => *cells,
        }
    }

    /// Converts the variant to one of `(-1, 0, 1)` to be used to index lists.
    pub fn to_offset(&self) -> isize {
        match self {
            LaneSwitch::Left(_) => -1,
            LaneSwitch::Right(_) => 1,
            LaneSwitch::Stay(_) => 0
        }
    }

    /// Returns `true` if variant is eihter `Left` or `Right`.
    pub fn is_switch(&self) -> bool {
        !matches!(self, LaneSwitch::Stay(_))
    }
}

/// Represents a road.
#[derive(Debug)]
pub struct Road {
    rng: ThreadRng,
    lanes: Vec<Vec<Cell>>,
    n_lanes: u32,
    length: u32,
    cells_to_next_cars: Vec<u8>,
    cells_to_next_obstacles: Vec<u8>,
    rounds: u32,
    n_cars: u32,
    overflow_flip_flop: FlipFlop,
    dilly_dally_probability: f32,
    stay_in_lane_probability: f32,
    traffic_lights_red: bool,
}

impl Road {
    pub fn new(
        lanes: u32,
        length: u32,
        vehicle_blueprints: &Vec<VehicleBlueprint>,
        dilly_dally_probability: f32, 
        stay_in_lane_probability: f32,
        block: &Vec<CellLocationRange>,
        traffic_lights: &Vec<CellLocation>,
    ) -> Self {

        if !(0.0..=1.0).contains(&dilly_dally_probability) {
            panic!("Dilly-dally probability must be a number between 0 and 1.");
        }

        let mut rng = thread_rng();
        let n_lanes = lanes;

        let mut lanes = Self::create_lanes_and_cells(n_lanes, length);
        let unblocked_cells_per_lane = Self::block_cells(&mut lanes, length, block);
        Self::add_traffic_lights(&mut lanes, traffic_lights);
        let n_cars = Self::add_cars(&mut lanes, unblocked_cells_per_lane, &mut rng, vehicle_blueprints);

        Self {
            rng,
            lanes,
            n_lanes,
            length,
            cells_to_next_cars: vec![255u8; n_lanes as usize],
            cells_to_next_obstacles: vec![255u8; n_lanes as usize],
            rounds: 0,
            n_cars,
            overflow_flip_flop: FlipFlop::new(),
            dilly_dally_probability,
            stay_in_lane_probability,
            traffic_lights_red: false,
        }
    }

    /// Creates a vector of lanes, where each lane is a vector of cells.
    fn create_lanes_and_cells(n_lanes: u32, lane_length: u32) -> Vec<Vec<Cell>> {
        let mut lanes = Vec::<Vec<Cell>>::with_capacity(n_lanes as usize);
        for _ in 0..n_lanes as usize {
            let mut lane = Vec::<Cell>::with_capacity(lane_length as usize);
            for _ in 0..lane_length {
                lane.push(Cell::new());
            }
            lanes.push(lane);
        }
        lanes
    }

    /// Blocks the certain cells for construction simulation. Returns the number of unblocked cells
    /// in each lane.
    fn block_cells(lanes: &mut [Vec<Cell>], length: u32, block: &Vec<CellLocationRange>) -> Vec<u32> {
        let mut unblocked_cells_per_lane = vec![length; lanes.len()];
        for blocked in block {
            let lane_i = blocked.lane();
            let lane = &mut lanes[blocked.lane()];
            let unblocked = &mut unblocked_cells_per_lane[lane_i];
            for cell_i in blocked.indexes() {
                lane[cell_i].block();
                *unblocked -= 1;
            }
        }
        unblocked_cells_per_lane
    }

    fn add_traffic_lights(lanes: &mut [Vec<Cell>], traffic_lights: &Vec<CellLocation>) {
        for traffic_light in traffic_lights {
            lanes[traffic_light.lane()][traffic_light.index()].make_traffic_light();
        }
    }

    /// Adds cars to the road. Formula for number of cars in each lane: `(traffic_density * unblocked_cells_in_lane).round()`.
    fn add_cars(lanes: &mut [Vec<Cell>], unblocked_cells_per_lane: Vec<u32>, rng: &mut ThreadRng, vehicle_blueprints: &Vec<VehicleBlueprint>) -> u32 {
        if !(0.0..=1.0).contains(&vehicle_blueprints.iter().map(|vb| vb.traffic_density()).reduce(|acc, td| td + acc).unwrap_or(0.0)) {
            panic!("The sum of all traffic densities must be a number between 0 and 1.");
        }
        let mut n_cars: u32 = 0;
        for vehicle_blueprint in vehicle_blueprints {
            for (lane, unblocked)in lanes.iter_mut().zip(unblocked_cells_per_lane.iter()) {
                let n_cars_in_lane = (vehicle_blueprint.traffic_density() * *unblocked as f32).round() as u32;
                let mut spawned_cars: u32 = 0;
                let mut index: usize = 0;
                while spawned_cars < n_cars_in_lane {
                    let cell = &mut lane[index];
                    if Self::occurs(rng, vehicle_blueprint.traffic_density()) && cell.free(false) {
                        spawned_cars += 1;
                        cell.put_car(Car::new(vehicle_blueprint)).unwrap();
                    }
                    index = (index + 1) % lane.len();
                }
                n_cars += n_cars_in_lane;
            }
        }
        n_cars
    }

    /// Returns `true` `probability * 100`% of the time.
    fn occurs(rng: &mut ThreadRng, probability: f32) -> bool {
        rng.gen::<f32>() <= probability
    }

    /// Returns the number of cars on the road.
    pub fn cars(&self) -> u32 {
        self.n_cars
    }

    /// Returns the number of lanes.
    pub fn lanes(&self) -> u32 {
        self.n_lanes
    }

    /// Returns the length of the road. (Equal to the number of cells in each lane.)
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Returns the number of rounds that the simulator has run so far.
    pub fn rounds(&self) -> u32 {
        self.rounds
    }

    /// Returns the `dilly_dally_probability`.
    pub fn dilly_dally_probability(&self) -> f32 {
        self.dilly_dally_probability
    }

    /// Returns the `stay_in_lane_probability`.
    pub fn stay_in_lane_probability(&self) -> f32 {
        self.stay_in_lane_probability
    }

    /// Provides read access to all cells. Outer vector holds lanes, inner vector holds cells.
    pub fn cells(&self) -> &Vec<Vec<Cell>> {
        &self.lanes
    }

    /// Returns the average number of cells driven per car per round.
    pub fn average_speed(&self) -> f64 {
        let mut sum = 0;
        for lane in &self.lanes {
            for cell in lane {
                if let Some(car) = cell.car() {
                    sum += car.distance();
                }
            }
        }
        sum as f64 / self.cars() as f64 / self.rounds() as f64
    }

    /// Returns the average amount of accelerations per car per round.
    pub fn average_accelerations(&self) -> f64 {
        let mut sum = 0;
        for lane in &self.lanes {
            for cell in lane {
                if let Some(car) = cell.car() {
                    sum += car.accelerations();
                }
            }
        }
        sum as f64 / self.cars() as f64 / self.rounds() as f64
    }

    /// Returns the average amount of deaccelerations per car per round.
    pub fn average_deaccelerations(&self) -> f64 {
        let mut sum = 0;
        for lane in &self.lanes {
            for cell in lane {
                if let Some(car) = cell.car() {
                    sum += car.deaccelerations();
                }
            }
        }
        sum as f64 / self.cars() as f64 / self.rounds() as f64
    }

    fn update_traffic_lights(&mut self) {
        self.traffic_lights_red = self.rounds % 100 != self.rounds % 200;
    }

    pub fn traffic_lights_red(&self) -> bool {
        self.traffic_lights_red
    }

    fn prepare_cells_to_next_obstacles_for_wrap_around(&mut self) {
        for (lane_i, lane) in self.lanes.iter().enumerate() {
            let mut looking_for_first_obstacle = true;
            'cells: for cell_i in 0u8..cmp::min(self.length(), 255) as u8 {
                if looking_for_first_obstacle && !lane[cell_i as usize].free(self.traffic_lights_red) {
                    self.cells_to_next_obstacles[lane_i] = cell_i;
                    looking_for_first_obstacle = false;
                }
                if lane[cell_i as usize].car().is_some() {
                    self.cells_to_next_cars[lane_i] = cell_i;
                    break 'cells;
                }
            }
        }
    }

    fn check_sides_clear(&self, lane_index: usize, cell_index: usize) -> (bool, bool) {
        let not_in_leftmost_lane = lane_index > 0;
        let not_in_rightmost_lane = lane_index + 1 != self.lanes.len();
        let left_clear = not_in_leftmost_lane && self.lanes[lane_index - 1][cell_index].free(self.traffic_lights_red);
        let right_clear = not_in_rightmost_lane && self.lanes[lane_index + 1][cell_index].free(self.traffic_lights_red);
        (left_clear, right_clear)
    }

    /// Notes that there is a car in a certain lane a certain amount of cells away.
    fn note_car_obstacle(&mut self, lane_index: usize, distance_away: u8) {
        self.cells_to_next_cars[lane_index] = distance_away;
        // a car is always an obstacles too
        self.cells_to_next_obstacles[lane_index] = distance_away;
    }

    fn note_car_free(&mut self, lane_index: usize, other_obstacle: bool) {
        let road_length = self.length();
        let cells_to_next_car = &mut self.cells_to_next_cars[lane_index];
        if *cells_to_next_car < 255 && (*cells_to_next_car as u32) < road_length {
            // Prevents from adding with overflow in cases where the
            // next gap is very far away or there are no cars in the lane.
            *cells_to_next_car += 1;
        }
        if other_obstacle {
            self.cells_to_next_obstacles[lane_index] = 0;
        } else {
            let cells_to_next_obstacle = &mut self.cells_to_next_obstacles[lane_index];
            if *cells_to_next_obstacle < 255 && (*cells_to_next_obstacle as u32) < road_length {
                *cells_to_next_obstacle += 1;
            }
        }
    }

    /// Simulates one round of the cellular automaton.
    pub fn round(&mut self) {
        self.rounds += 1;
        self.update_traffic_lights();

        let length = self.length() as usize;
        let n_lanes = self.lanes.len();

        self.prepare_cells_to_next_obstacles_for_wrap_around();

        // Iterate over cars in reverse to avoid having to look ahead each time.
        for cell_i in (0..length).rev() {
            for lane_i in 0..n_lanes {
                if self.lanes[lane_i][cell_i].blocked() || self.lanes[lane_i][cell_i].is_red_light(self.traffic_lights_red) {
                    // skip blocked cells
                    self.note_car_free(lane_i, true);
                    continue;
                }

                let (left_clear, right_clear) = self.check_sides_clear(lane_i, cell_i);
                // let lane = &mut self.lanes[lane_i];
                let car = self.lanes[lane_i][cell_i].take_car();
                match car {
                    Some(mut car) => {
                        if !car.flip_flop_unsync(&self.overflow_flip_flop) {
                            // Car has already been moved. This is due to a wrap-around.
                            self.note_car_obstacle(lane_i, 0);
                            self.lanes[lane_i][cell_i].put_car(car).expect("Cannot put car into a cell that already contains a car. If you see this error message something has gone very wrong. The flip-flop must be broken.");
                            continue;
                        }

                        // -- calculate movement and update car --
                        car.increase_speed();
                        let stay = Self::occurs(&mut self.rng, self.stay_in_lane_probability);
                        let best_switch: LaneSwitch = self.determine_best_lane(lane_i, car.speed(), left_clear, right_clear, stay);
                        let is_switch = best_switch.is_switch();
                        car.finish(best_switch.driveable(), !is_switch && Self::occurs(&mut self.rng, self.dilly_dally_probability));
                        self.note_car_obstacle(lane_i, 0);

                        // -- place car into new cell and record cell passage --
                        if is_switch && car.speed() > 1 {
                            self.lanes[lane_i][(cell_i + 1) % length].pass();
                        }
                        let target_i = cell_i + car.speed() as usize;
                        let target_lane_i = (lane_i as isize + best_switch.to_offset()) as usize;
                        if is_switch && car.speed() > 0 {
                            self.note_car_obstacle(target_lane_i, car.speed() - 1);
                        }
                        let target_lane = &mut self.lanes[target_lane_i];
                        for passed_cell_i in (cell_i + 1)..=target_i {
                            target_lane[passed_cell_i % length].pass();
                        }
                        if let Err(PutCarErrorInformation { cell_blocked, new_car }) = target_lane[target_i % length].put_car(car) {
                            panic!(
                                "FATAL: Cannot put car into a cell that {}!\nDEBUG INFO:\n  Round: {}\n  Car: {}:{} (lane_index:cell_index)\n  Speed: {}\n  Cells to next cars by lane: {:?}\n  Cells to next obstacles by lane: {:?}\n  LaneSwitch: {:?}\n    Target: {}:{} (lane_index:cell_index)",
                                if cell_blocked { "is blocked" } else { "already contains a car" },
                                self.rounds,
                                lane_i, cell_i,
                                new_car.speed(),
                                self.cells_to_next_cars,
                                self.cells_to_next_obstacles,
                                best_switch,
                                target_lane_i, target_i % length
                            );
                        }
                    },
                    None => {
                        self.note_car_free(lane_i, false);
                    }
                }
            }
        }
        // Flip the flop to keep track of which cars have already been moved in a round.
        self.overflow_flip_flop.flip_flop();
    }

    /// Determines the best lane to switch to (or stay on) based on surrounding traffic, 
    /// available_speed and the stay in late probability.
    fn determine_best_lane(&self, lane_i: usize, available_speed: u8, left_clear: bool, right_clear: bool, stay: bool) -> LaneSwitch {
        let driveable_without_passing_on_right = |target_lane_offset: isize| {
            let left_index = lane_i as isize + target_lane_offset - 1;
            let target_lane_index = (lane_i as isize + target_lane_offset) as usize;
            let mut distance = if left_index < 0 {
                // no lane to left to check
                self.cells_to_next_obstacles[target_lane_index]
            } else {
                // check lane to left of target
                cmp::min(
                    // distance to get alongside car in left lane from target
                    cmp::min(self.cells_to_next_cars[left_index as usize], 254) + 1,
                    self.cells_to_next_obstacles[target_lane_index]
                )
            };

            // Required because lanes that have already incremented their distance
            // counters are one cell closer when switching lanes.
            if target_lane_offset < 0 && distance > 0 {
                distance -= 1 
            }
            distance
        };

        let front_space = cmp::min(driveable_without_passing_on_right(0), available_speed);
        let mut best_option = LaneSwitch::Stay(front_space);

        if !stay && (front_space >= 1 || available_speed <= 1) {
            if left_clear {
                let left_space = cmp::min(driveable_without_passing_on_right(-1), available_speed);
                if left_space > 0 && left_space > best_option.driveable() {
                    best_option = LaneSwitch::Left(left_space);
                }
            }
            if right_clear {
                let right_space = driveable_without_passing_on_right(1);
                if right_space > 0 && right_space >= best_option.driveable() {
                    best_option = LaneSwitch::Right(cmp::min(right_space, available_speed));
                }
            }
        }

        best_option
    }
}

impl fmt::Display for Road {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut road = String::with_capacity((self.lanes.len() + 1) * (self.length + 3) as usize);
        let colored_digits: Vec<String> = (0..10).map(|n| format!("{}",
            n.to_string().truecolor(
                ((1.0 - n as f32 / 10.0) * 255.0).floor() as u8,
                ((1.0 - n as f32 / 10.0 / 2.5) * 255.0).floor() as u8,
                255
            )
        )).collect();
        road += "  ";
        for row in 0..self.length {
            road += &colored_digits[(row % 10) as usize];
        }
        road += "\n";
        for (index, lane) in self.cells().iter().enumerate() {
            road += &(colored_digits[index % 10].clone() + " ");
            for cell in lane {
                if let Some(car) = cell.car() {
                    let [r, g, b] = car.speed_rgb();
                    road += &format!("{}", car.speed().to_string().truecolor(r, g, b));
                } else if cell.blocked() {
                    road += "x";
                } else if cell.is_red_light(self.traffic_lights_red()) {
                    road += "#";
                } else {
                    road += "_";
                }
            }
            if index + 1 < self.n_lanes as usize {
                road += "\n";
            }
        }
        write!(f, "{}", road)
    }
}

