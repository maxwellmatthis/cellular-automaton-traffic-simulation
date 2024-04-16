use std::{cmp, fmt, isize};
use rand::prelude::*;
use crate::cell::Cell;
use crate::car::Car;
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
    rounds: u32,
    cars: u32,
    overflow_flip_flop: FlipFlop,
    dilly_dally_probability: f32,
}

impl Road {
    pub fn new(
        lanes: u32,
        length: u32,
        max_speed: u8,
        traffic_density: f32,
        dilly_dally_probability: f32, 
    ) -> Self {
        if !(0.0..=1.0).contains(&traffic_density) {
            panic!("Traffic density must be a number between 0 and 1.");
        }
        if !(0.0..=1.0).contains(&traffic_density) {
            panic!("Dilly-dally probability must be a number between 0 and 1.");
        }

        let mut rng = thread_rng();
        let n_lanes = lanes;
        let mut lanes = Vec::<Vec<Cell>>::with_capacity(n_lanes as usize);
        let mut cells_to_next_cars: Vec<u8> = vec![0; n_lanes as usize];
        let cars_per_lane = (traffic_density * length as f32).floor() as u32;

        #[allow(clippy::needless_range_loop)] // since the number of iterations is most important
        for lane_i in 0..n_lanes as usize {
            let mut lane = Vec::<Cell>::with_capacity(length as usize);
            for _ in 0..length {
                lane.push(Cell::new());
            }

            let mut spawned_cars: u32 = 0;
            let mut index: usize = 0;
            let mut set_cells_to_next_car = false;
            while spawned_cars < cars_per_lane {
                let cell = &mut lane[index];
                if Self::occurs(&mut rng, traffic_density) && cell.car().is_none() {
                    if !set_cells_to_next_car {
                        set_cells_to_next_car = false;
                        cells_to_next_cars[lane_i] = TryInto::<u8>::try_into(index).unwrap_or(255);
                    }
                    spawned_cars += 1;
                    cell.put_car(Car::new(max_speed, 0));
                }
                index = (index + 1) % lane.len();
            }

            lanes.push(lane);
        }

        Self {
            rng,
            lanes,
            n_lanes,
            length,
            cells_to_next_cars,
            rounds: 0,
            cars: cars_per_lane * n_lanes,
            overflow_flip_flop: FlipFlop::new(),
            dilly_dally_probability,
        }
    }

    /// Returns `true` `probability * 100`% of the time.
    fn occurs(rng: &mut ThreadRng, probability: f32) -> bool {
        rng.gen::<f32>() <= probability
    }

    /// Returns the number of cars on the road.
    pub fn cars(&self) -> u32 {
        self.cars
    }

    /// Returns the number of lanes.
    pub fn lanes(&self) -> u32 {
        self.n_lanes
    }

    /// Returns the length of the road. (Equal to the number of cells in each lane.)
    pub fn length(&self) -> u32 {
        self.length
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
        sum as f64 / self.cars as f64 / self.rounds as f64
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
        sum as f64 / self.cars as f64 / self.rounds as f64
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
        sum as f64 / self.cars as f64 / self.rounds as f64
    }

    /// Simulates one round of the cellular automaton.
    pub fn round(&mut self) {
        self.rounds += 1;

        // Iterate over cars in reverse to avoid having to look ahead each time.
        let length = self.length() as usize;

        for rev_i in 1..=length {
            let cell_i = length - rev_i;
            for lane_i in 0..self.lanes.len() {
                // Check sides for lane switching before borrowing &mut self.lanes to work on car.
                let not_in_leftmost_lane = lane_i > 0;
                let not_in_rightmost_lane = lane_i + 1 != self.lanes.len();
                let left_clear = not_in_leftmost_lane && self.lanes[lane_i - 1][cell_i].car().is_none();
                let right_clear = not_in_rightmost_lane && self.lanes[lane_i + 1][cell_i].car().is_none();

                let lane = &mut self.lanes[lane_i];
                let car = lane[cell_i].take_car();
                match car {
                    Some(mut car) => {
                        if !car.flip_flop_unsync(&self.overflow_flip_flop) {
                            // Car has already been moved.
                            // This is likely due to a wrap-around.
                            self.cells_to_next_cars[lane_i] = 0;
                            lane[cell_i].put_car(car);
                            continue;
                        }

                        car.increase_speed();
                        let best_switch: LaneSwitch = self.determine_best_lane(lane_i, car.speed(), left_clear, right_clear);
                        car.finish(best_switch.driveable(), !best_switch.is_switch() && Self::occurs(&mut self.rng, self.dilly_dally_probability));
                        self.cells_to_next_cars[lane_i] = 0;

                        if car.speed() > 1 {
                            self.lanes[lane_i][(cell_i + 1) % length].pass();
                        }
                        let target_i = cell_i + car.speed() as usize;
                        let target_lane_i = (lane_i as isize + best_switch.to_offset()) as usize;
                        let target_lane = &mut self.lanes[target_lane_i];
                        for passed_cell_i in (cell_i + 1)..=target_i {
                            target_lane[passed_cell_i % length].pass();
                        }
                        if target_i > cell_i {
                            // `car.speed()` cannot be `<1` because `target_i` would not be
                            // greater than `cell_i` if the car was stopped.
                            self.cells_to_next_cars[target_lane_i] = car.speed() - 1;
                        }
                        target_lane[target_i % length].put_car(car);
                    }
                    None => {
                        let cells_to_next_car = self.cells_to_next_cars[lane_i];
                        if cells_to_next_car < 255 && (cells_to_next_car as u32) < self.length() {
                            // Prevents from adding with overflow in cases where the
                            // next gap is very far away or there are no cars in the lane.
                            self.cells_to_next_cars[lane_i] += 1;
                        }
                    }
                }
            }
        }
        // Flip the flop to keep track of the current round.
        self.overflow_flip_flop.flip_flop();
    }

    /// Determines the best lane to switch to (or stay on) based on surrounding traffic and
    /// available_speed.
    fn determine_best_lane(&self, lane_i: usize, available_speed: u8, left_clear: bool, right_clear: bool) -> LaneSwitch {
        let driveable_without_passing_on_right = |target_lane_offset: isize| {
            let left_index = lane_i as isize + target_lane_offset - 1;
            let target_lane_index = (lane_i as isize + target_lane_offset) as usize;
            let mut distance = if left_index < 0 {
                // no lane to left to check
                self.cells_to_next_cars[target_lane_index]
            } else {
                // check lane to left of target
                cmp::min(
                    // distance to get alongside car in left lane from target
                    cmp::min(self.cells_to_next_cars[left_index as usize], 254) + 1,
                    self.cells_to_next_cars[target_lane_index]
                )
            };

            // Required because lanes that have already incremented their distance
            // counters one closer when switching lanes.
            if target_lane_offset < 0 && distance > 0 {
                distance -= 1 
            }
            distance
        };

        let front_space = cmp::min(driveable_without_passing_on_right(0), available_speed);
        let mut best_option = LaneSwitch::Stay(front_space);

        if front_space >= 1 || available_speed <= 1 {
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
        let mut road = String::with_capacity(self.lanes.len() * self.length as usize);
        for lane in self.cells() {
            for cell in lane {
                if let Some(car) = cell.car() {
                    let [r, g, b] = car.speed_rgb();
                    road += &format!("{}", car.speed().to_string().truecolor(r, g, b));
                } else {
                    road += "_"
                }
            }
            road += "\n";
        }
        write!(f, "{}", road)
    }
}

