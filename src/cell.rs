use crate::car::Car;

pub struct Cell {
    car: Option<Car>,
    cars_passed: i32
}

impl Cell {
    pub fn new() -> Self {
        return Self {
            car: None,
            cars_passed: 0
        }
    }

    pub fn car(&self) -> &Option<Car> {
        return &self.car;
    }

    pub fn take_car(&mut self) -> Option<Car> {
        return self.car.take();
    }

    pub fn put_car(&mut self, car: Car) {
        if car.speed() != 0 {
            // Car must have moved, so its not the same car that
            // occupied the cell before last `take_car` call.
            self.cars_passed += 1;
        }
        if self.car.is_some() {
            panic!("Cannot put car into a cell that already contains a car.");
        }
        self.car = Some(car);
    }

    /// Returns the cars per round that have come by this cell.
    pub fn flow(&self, rounds: u32) -> f64 {
        return Into::<f64>::into(self.cars_passed) / Into::<f64>::into(rounds);
    }
}

