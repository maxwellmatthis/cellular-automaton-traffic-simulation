use std::cmp::Ordering;
use crate::flip_flop::FlipFlop;

pub struct Car {
    max_speed: u8,
    last_speed: u8,
    speed: u8,
    distance: u32,
    accelerations: u32,
    deaccelerations: u32,
    overflow_flip_flop: FlipFlop
}

impl Car {
    pub fn new(max_speed: u8, initial_speed: u8) -> Self {
        Self {
            max_speed,
            last_speed: initial_speed,
            speed: initial_speed,
            distance: 0,
            accelerations: 0,
            deaccelerations: 0,
            overflow_flip_flop: FlipFlop::new()
        }
    }

    pub fn speed(&self) -> u8 {
        self.speed
    }

    pub fn distance(&self) -> u32 {
        self.distance
    }

    pub fn accelerations(&self) -> u32 {
        self.accelerations
    }

    pub fn deaccelerations(&self) -> u32 {
        self.deaccelerations
    }
    
    /// Simulates one step for the car.
    pub fn round(&mut self, cells_to_next_car: u8, dilly_dally: bool) {
        self.increase_speed();
        self.decrease_speed_to(cells_to_next_car);
        if dilly_dally {
            self.decrease_speed();
        }
        self.record();
    }

    /// Records the current round
    fn record(&mut self) {
        self.distance += self.speed() as u32;
        match self.speed.cmp(&self.last_speed) {
            Ordering::Greater => self.accelerations += 1,
            Ordering::Less => self.deaccelerations += 1,
            Ordering::Equal => ()
        }
        self.last_speed = self.speed;
    }

    pub fn flip_flop_sync(&mut self, other: &FlipFlop) -> bool {
        self.overflow_flip_flop.sync(other)
    }
    
    /// Increases the speed by one if the maximum speed has not yet been reached.
    fn increase_speed(&mut self) {
        if self.speed == self.max_speed {
            return; 
        }
        self.speed += 1;
    }

    /// Decreases the speed by one if the car is not already stopped.
    fn decrease_speed(&mut self) { 
        if self.speed == 0 {
            return;
        }
        self.speed -= 1;
    }

    /// Decreases the speed by a specified amount.
    ///
    /// Note: `0 < to < current_speed`
    fn decrease_speed_to(&mut self, to: u8) {
        if self.speed < to {
            return;
        }
        self.speed = to;
    }
}

