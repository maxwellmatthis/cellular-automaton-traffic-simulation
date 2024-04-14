use std::{cmp, fmt};
use rand::prelude::*;
use crate::cell::Cell;
use crate::car::Car;
use crate::flip_flop::FlipFlop;

pub struct Road {
    rng: ThreadRng,
    lanes: Vec<Vec<Cell>>,
    n_lanes: u32,
    length: u32,
    cells_to_next_cars: Vec<u8>,
    max_speed: u8,
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
        let cells_to_next_cars: Vec<u8> = vec![0; n_lanes as usize];
        let cars_per_lane = (traffic_density * length as f32).floor() as u32;

        for _ in 0..n_lanes {
            let mut lane = Vec::<Cell>::with_capacity(length as usize);
            for _ in 0..length {
                lane.push(Cell::new());
            }

            let mut spawned_cars: u32 = 0;
            let mut index: usize = 0;
            while spawned_cars < cars_per_lane {
                let cell = &mut lane[index];
                if Self::occurs(&mut rng, traffic_density) && cell.car().is_none() {
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
            max_speed,
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

    /// Returns the maximum speed allowed.
    pub fn max_speed(&self) -> u8 {
        self.max_speed
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

        for i in 0..self.cells_to_next_cars.len() {
            let mut cells_to_next_car = self.max_speed();
            let lane = &self.lanes[i];
            // Prepare wrap-around look-ahead for last vehicles.
            for (j, cell) in lane.iter().take(cmp::min(lane.len(), cells_to_next_car as usize)).enumerate() {
                if cell.car().is_some() {
                    cells_to_next_car = j as u8;
                    break;
                }
            }
            self.cells_to_next_cars[i] = cells_to_next_car;
        }

        // Iterate over cars in reverse to avoid having to look ahead each time.
        let length = self.length as usize;
        for rev_i in 1..=length {
            let cell_i = length - rev_i;
            for lane_i in 0..self.lanes.len() {
                let lane = &mut self.lanes[lane_i];
                let car = lane[cell_i].take_car();
                match car {
                    Some(mut car) => {
                        if !car.flip_flop_sync(&self.overflow_flip_flop) {
                            // Car has already been moved.
                            // This is likely due to a wrap-around.
                            lane[cell_i].put_car(car);
                            continue;
                        }

                        // TODO: add lane switching (car need to look ahead in own and adjacent
                        // lanes)
                        car.round(self.cells_to_next_cars[lane_i], Self::occurs(&mut self.rng, self.dilly_dally_probability));
                        self.cells_to_next_cars[lane_i] = 0;

                        let target_index = cell_i + car.speed() as usize;
                        for passed_cell_index in (cell_i + 1)..=target_index {
                            lane[passed_cell_index % length].pass();
                        }
                        lane[target_index % length].put_car(car);
                    }
                    None => {
                        // Prevents from adding with overflow in cases where the
                        // next gap is very far away
                        if self.cells_to_next_cars[lane_i] < self.max_speed() {
                            self.cells_to_next_cars[lane_i] += 1;
                        }
                    }
                }
            }
        }
        // Flip the flop to keep track of the current round.
        self.overflow_flip_flop.flip_flop();
    }
}

impl fmt::Display for Road {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut road = String::with_capacity(self.lanes.len() * self.length as usize);
        for lane in self.cells() {
            for cell in lane {
                if let Some(car) = cell.car() {
                    road += &car.speed().to_string();
                }
                else {
                    road += "_"
                }
            }
            road += "\n";
        }
        write!(f, "{}", road)
    }
}

