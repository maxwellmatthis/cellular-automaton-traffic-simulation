use std::{ops::Range, str::FromStr};
use crate::car::Car;

#[derive(Debug)]
pub struct PutCarErrorInformation {
    pub cell_blocked: bool,
    pub new_car: Car,
}

#[derive(Debug)]
pub struct Cell {
    car: Option<Car>,
    cars_passed: i32,
    blocked: bool,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            car: None,
            cars_passed: 0,
            blocked: false,
        }
    }

    /// Returns a read-only refrence to the car contained in the cell if there is one.
    pub fn car(&self) -> &Option<Car> {
        &self.car
    }

    /// Blocks the cell. Cars will not be able to use this cell anymore.
    pub fn block(&mut self) {
        self.blocked = true;
    }

    /// Returns whether the cell is blocked.
    pub fn blocked(&self) -> bool {
        self.blocked
    }

    /// Returns whether the cell is free, meaning it contains no car and is not blocked, hence
    /// theoretically driveable.
    pub fn free(&self) -> bool {
        !self.blocked() && self.car().is_none()
    }

    /// Takes the car from the cell if there is one.
    pub fn take_car(&mut self) -> Option<Car> {
        self.car.take()
    }

    /// Tries to put a car into the cell. Fails if the cell is blocked or already contains another
    /// car.
    pub fn put_car(&mut self, car: Car) -> Result<(), PutCarErrorInformation> {
        if self.blocked() || self.car().is_some() {
            return Err(PutCarErrorInformation { cell_blocked: self.blocked(), new_car: car });
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

        let lane = lane.replace(' ', "").parse::<usize>().map_err(|_| ParseCellLocationError)?;
        let index = index.replace(' ', "").parse::<usize>().map_err(|_| ParseCellLocationError)?;

        Ok(CellLocation { lane, index })
    }
}

#[derive(Debug, PartialEq)]
pub struct CellLocationRange {
    lane: usize,
    start: usize,
    end: usize,
}

impl CellLocationRange {
    pub fn lane(&self) -> usize {
        self.lane
    }

    pub fn indexes(&self) -> Range<usize> {
        Range { start: self.start, end: self.end }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCellLocationRangeError;

impl FromStr for CellLocationRange {
    type Err = ParseCellLocationRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: String = s.replace(' ', "");
        let (lane, indexes_str) = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.split_once(','))
            .ok_or(ParseCellLocationRangeError)?;

        let lane = lane.parse::<usize>().map_err(|_| ParseCellLocationRangeError)?;

        let (start, end) = match indexes_str.split_once('-') {
            Some((start, end)) => {
                let start = start.parse::<usize>().map_err(|_| ParseCellLocationRangeError)?;
                let end = end.parse::<usize>().map_err(|_| ParseCellLocationRangeError)?;
                (start, end)
            },
            None => {
                let single = indexes_str.parse::<usize>().map_err(|_| ParseCellLocationRangeError)?;
                (single, single + 1)
            }
        };

        Ok(CellLocationRange { lane, start, end })
    }
}

