use std::cmp::Ordering;
use crate::flip_flop::FlipFlop;

#[derive(Debug)]
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

    /// Returns the speed in cells per round. (`1cell/round = 7.5m/s`)
    pub fn speed(&self) -> u8 {
        self.speed
    }

    /// Converts the speed to an RGB color based on the percentage of the max speed.
    pub fn speed_rgb(&self) -> [u8; 3] {
        let speed_norm: f32 = Into::<f32>::into(self.speed()) / Into::<f32>::into(self.max_speed);
        let mut red = 255;
        let mut green = 255;
        if speed_norm <= 0.5 {
            green = (255.0 * 2.0 * speed_norm).floor() as u8;
        } else {
            red = (255.0 * 2.0 * (1.0 - speed_norm)).floor() as u8;
        }
        [red, green, 0]
    }

    /// Returns the distance in cells. (`1cell = 7.5m`)
    pub fn distance(&self) -> u32 {
        self.distance
    }

    /// Returns the number of rounds that the speed has increased compared to the last round.
    pub fn accelerations(&self) -> u32 {
        self.accelerations
    }

    /// Returns the number of rounds that the speed has decreased compared to the last round.
    pub fn deaccelerations(&self) -> u32 {
        self.deaccelerations
    }

    /// Finishes the simulation round for the car. (breaking and recording)
    pub fn finish(&mut self, cells_to_next_car: u8, dilly_dally: bool) {
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

    pub fn flip_flop_unsync(&mut self, other: &FlipFlop) -> bool {
        self.overflow_flip_flop.unsync(other)
    }
    
    /// Increases the speed by one if the maximum speed has not yet been reached.
    pub fn increase_speed(&mut self) {
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

