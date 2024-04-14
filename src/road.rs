use std::{cmp, fmt};
use rand::prelude::*;
use crate::cell::Cell;
use crate::car::Car;
use crate::flip_flop::FlipFlop;

pub struct Road {
    rng: ThreadRng,
    cells: Vec<Cell>,
    max_speed: u8,
    rounds: u32,
    cars: u32,
    overflow_flip_flop: FlipFlop,
    dilly_dally_probability: f32,
}

impl Road {
    pub fn new(
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
        let mut cells = Vec::<Cell>::with_capacity(length as usize);
        let cars = (traffic_density * length as f32).floor() as u32;

        for _ in 0..length as usize {
            cells.push(Cell::new());
        }
        let mut spawned_cars: u32 = 0;
        let mut index: usize = 0;
        while spawned_cars < cars {
            let cell = &mut cells[index];
            if Self::occurs(&mut rng, traffic_density) && cell.car().is_none() {
                spawned_cars += 1;
                cell.put_car(Car::new(max_speed, 0));
            }
            index = (index + 1) % cells.len();
        }
        Self {
            rng,
            cells,
            max_speed,
            rounds: 0,
            cars,
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

    /// Returns the average number of cells driven per car per round.
    pub fn average_speed(&self) -> f64 {
        let mut sum = 0;
        for cell in &self.cells {
            if let Some(car) = cell.car() {
                sum += car.distance();
            }
        }
        sum as f64 / self.cars as f64 / self.rounds as f64
    }

    /// Returns the average amount of accelerations per car per round.
    pub fn average_accelerations(&self) -> f64 {
        let mut sum = 0;
        for cell in &self.cells {
            if let Some(car) = cell.car() {
                sum += car.accelerations();
            }
        }
        sum as f64 / self.cars as f64 / self.rounds as f64
    }

    /// Returns the average amount of deaccelerations per car per round.
    pub fn average_deaccelerations(&self) -> f64 {
        let mut sum = 0;
        for cell in &self.cells {
            if let Some(car) = cell.car() {
                sum += car.deaccelerations();
            }
        }
        sum as f64 / self.cars as f64 / self.rounds as f64
    }

    // Provides read access to road cells.
    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    /// Simulates one round of the cellular automaton.
    pub fn round(&mut self) {
        self.rounds += 1;

        let mut cells_to_next_car = self.max_speed();
        // Prepare wrap-around look-ahead for last vehicles.
        for i in 0..cmp::min(self.cells().len(), cells_to_next_car as usize) {
            if self.cells[i].car().is_some() {
                cells_to_next_car = i as u8;
                break;
            }
        }

        // Iterate over cars in reverse to avoid having to look ahead each time.
        let n_cells = self.cells.len();
        for rev_i in 1..=n_cells {
            let i = n_cells - rev_i;
            let car = self.cells[i].take_car();
            match car {
                Some(mut car) => {
                    if !car.flip_flop_sync(&self.overflow_flip_flop) {
                        // Car has already been moved.
                        // This is likely due to a wrap-around.
                        self.cells[i].put_car(car);
                        continue;
                    }

                    car.round(cells_to_next_car, Self::occurs(&mut self.rng, self.dilly_dally_probability));
                    cells_to_next_car = 0;

                    let target_index = i + car.speed() as usize;
                    for passed_cell_index in (i + 1)..=target_index {
                        self.cells[passed_cell_index % n_cells].pass();
                    }
                    self.cells[target_index % n_cells].put_car(car);
                }
                None => {
                    // Prevents from adding with overflow in cases where the
                    // next gap is very far away
                    if cells_to_next_car < self.max_speed() {
                        cells_to_next_car += 1;
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
        let mut road = String::with_capacity(self.cells.len());
        for cell in &self.cells {
            if let Some(car) = cell.car() {
                road += &car.speed().to_string();
            }
            else {
                road += "_"
            }
        }
        write!(f, "{}", road)
    }
}

