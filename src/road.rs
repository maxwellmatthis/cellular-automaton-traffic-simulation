use std::fmt;
use rand::prelude::*;
use crate::cell::Cell;
use crate::car::Car;

pub struct Road {
    rng: ThreadRng,
    cells: Vec<Cell>,
    max_speed: u8,
    rounds: u32,
    dilly_dally_probability: f32,
    spawn_car_at_entrance_probability: f32,
    n_spawned_cars: f64,
    remove_car_on_exit_probability: f32,
    removed_cars_average_speed_sum: f64,
    n_removed_cars: f64
}

impl Road {
    pub fn new(
        length: usize,
        max_speed: u8,
        place_car_probability: f32,
        dilly_dally_probability: f32, 
        spawn_car_at_entrance_probability: f32,
        remove_car_on_exit_probability: f32
    ) -> Self {
        let mut rng = thread_rng();
        let mut cells = Vec::<Cell>::with_capacity(length);
        let mut n_spawned_cars = 0.0;
        for _ in 0..length {
            let mut cell = Cell::new();
            if Self::occurs(&mut rng, place_car_probability) {
                n_spawned_cars += 1.0;
                cell.put_car(Car::new(max_speed, 0));
            }
            cells.push(cell);
        }
        return Self {
            rng,
            cells,
            max_speed,
            rounds: 0,
            dilly_dally_probability,
            spawn_car_at_entrance_probability,
            n_spawned_cars,
            remove_car_on_exit_probability,
            removed_cars_average_speed_sum: 0.0,
            n_removed_cars: 0.0
        }
    }

    /// Returns `true` `probability * 100`% of the time.
    fn occurs(rng: &mut ThreadRng, probability: f32) -> bool {
        return rng.gen::<f32>() <= probability;
    }

    /// Returns the average number of cells driven per round.
    pub fn average_speed(&self) -> f64 {
        let mut n_cars: f64 = 0.0;
        let mut sum: f64 = 0.0;
        for cell in &self.cells {
            if let Some(car) = cell.car() {
                n_cars += 1.0;
                sum += car.average_speed();
            }
        }
        return sum / n_cars;
    }

    /// Returns the number of cars currently on the road.
    pub fn n_cars(&self) -> f64 {
        return self.n_spawned_cars - self.n_removed_cars;
    }

    // Provides read access to a road cell at a specific index.
    pub fn cell(&self, index: usize) -> &Cell {
        return &self.cells[index];
    }

    /// Simulates one round of the cellular automaton.
    pub fn round(&mut self) {
        self.rounds += 1;
        if Self::occurs(&mut self.rng, self.spawn_car_at_entrance_probability) {
            if let None = self.cells[0].car() {
                self.cells[0].put_car(Car::new(self.max_speed, self.max_speed));
                self.n_spawned_cars += 1.0;
            }
        }

        let mut cells_to_next_car = self.max_speed;
        // Prepare wrap-around look-ahead for last vehicles.
        for i in 0..cells_to_next_car {
            if let Some(_) = self.cells[Into::<usize>::into(i)].car() {
                cells_to_next_car = i;
                break;
            }
        }

        // Iterate over cars in reverse to avoid having to look ahead.
        let n_cells = self.cells.len();
        for rev_i in 1..=n_cells {
            let i = n_cells - rev_i;
            let car = self.cells[i].take_car();
            match car {
                Some(mut car) => {
                    if car.rounds() == self.rounds {
                        // Car has already been moved.
                        // This is likely due to a wrap-around.
                        self.cells[i].put_car(car);
                        continue;
                    }

                    // -- round update procedure --
                    // 1. increase speed by one
                    car.increase_speed();

                    // handle case that car exists the road
                    let dilly_dally = Self::occurs(&mut self.rng, self.dilly_dally_probability);
                    let on_exit_index = i + Into::<usize>::into(
                        car.speed() - if dilly_dally { 1 } else { 0 }
                    );
                    if Self::occurs(&mut self.rng, self.remove_car_on_exit_probability) &&
                       on_exit_index >= n_cells {
                        // Car has left the road and will not wrap around.
                        self.removed_cars_average_speed_sum += car.average_speed();
                        self.n_removed_cars += 1.0;
                        continue;
                    }

                    // 2. decrease speed to avoid hitting the next car
                    car.decrease_speed_to(cells_to_next_car);
                    // 3. potentially dilly_dally
                    if dilly_dally {
                        car.decrease_speed();
                    }

                    car.record();
                    cells_to_next_car = 0;

                    let target_index = i + Into::<usize>::into(car.speed());
                    self.cells[target_index % n_cells].put_car(car);
                }
                None => {
                    cells_to_next_car += 1;
                }
            }
            
        }
    }
}

impl fmt::Display for Road {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cars_count = format!("({}) ", self.n_cars());
        let mut road = String::with_capacity(self.cells.len() + cars_count.len());
        road += &cars_count;
        for cell in &self.cells {
            if let Some(car) = cell.car() {
                road += &car.speed().to_string();
            }
            else {
                road += "_"
            }
        }
        return write!(f, "{}", road);
    }
}

