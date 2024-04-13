use crate::car::Car;

pub struct Cell {
    car: Option<Car>,
    cars_passed: i32
}

impl Cell {
    pub fn new() -> Self {
        Self {
            car: None,
            cars_passed: 0
        }
    }

    pub fn car(&self) -> &Option<Car> {
        &self.car
    }

    pub fn take_car(&mut self) -> Option<Car> {
        self.car.take()
    }

    pub fn put_car(&mut self, car: Car) {
        if self.car.is_some() {
            panic!("Cannot put car into a cell that already contains a car.");
        }
        self.car = Some(car);
    }

    /// Records that a car has passed the cell.
    pub fn pass(&mut self) {
        self.cars_passed += 1;
    }

    /// Returns the cars per round that have come by this cell.
    pub fn flow(&self, rounds: u32) -> f64 {
        Into::<f64>::into(self.cars_passed) / Into::<f64>::into(rounds)
    }
}

