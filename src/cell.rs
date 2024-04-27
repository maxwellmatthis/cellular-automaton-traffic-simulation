use std::str::FromStr;
use crate::car::Car;

#[derive(Debug)]
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

    pub fn put_car(&mut self, car: Car) -> Result<(), Car> {
        if self.car.is_some() {
            // panic!("Cannot put car into a cell that already contains a car.");
            return Err(car);
        }
        self.car = Some(car);
        Ok(())
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

#[derive(Debug, PartialEq)]
pub struct CellLocation {
    lane: usize,
    index: usize
}

impl CellLocation {
    pub fn lane(&self) -> usize {
        self.lane
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCellLocationError;

impl FromStr for CellLocation {
    type Err = ParseCellLocationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lane, index) = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.split_once(','))
            .ok_or(ParseCellLocationError)?;

        let lane = lane.parse::<usize>().map_err(|_| ParseCellLocationError)?;
        let index = index.parse::<usize>().map_err(|_| ParseCellLocationError)?;

        Ok(CellLocation { lane, index })
    }
}
