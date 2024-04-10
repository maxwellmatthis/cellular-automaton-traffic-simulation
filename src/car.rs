use std::cmp::Ordering;

pub struct Car {
    max_speed: u8,
    last_speed: u8,
    speed: u8,
    rounds: u32,
    distance: u32,
    accelerations: u32,
    deaccelerations: u32
}

impl Car {
    pub fn new(max_speed: u8, initial_speed: u8) -> Self {
        Self {
            max_speed,
            last_speed: initial_speed,
            speed: initial_speed,
            rounds: 0,
            distance: 0,
            accelerations: 0,
            deaccelerations: 0
        }
    }

    pub fn speed(&self) -> u8 {
        self.speed
    }

    pub fn rounds(&self) -> u32 {
        self.rounds
    }

    /// Returns the average number of cells driven per round.
    pub fn average_speed(&self) -> f64 {
        Into::<f64>::into(self.distance) / Into::<f64>::into(self.rounds)
    }

    pub fn accelerations(&self) -> u32 {
        self.accelerations
    }

    pub fn deaccelerations(&self) -> u32 {
        self.deaccelerations
    }

    /// Records the current round
    pub fn record(&mut self) {
        self.rounds += 1;
        self.distance += Into::<u32>::into(self.speed());
        match self.speed.cmp(&self.last_speed) {
            Ordering::Greater => self.accelerations += 1,
            Ordering::Less => self.deaccelerations += 1,
            Ordering::Equal => ()
        }
        self.last_speed = self.speed;
    }

    /// Increases the speed by one if the maximum speed has not yet been reached.
    pub fn increase_speed(&mut self) {
        if self.speed == self.max_speed {
            return; 
        }
        self.speed += 1;
    }

    /// Decreases the speed by one if the car is not already stopped.
    pub fn decrease_speed(&mut self) { 
        if self.speed == 0 {
            return;
        }
        self.speed -= 1;
    }

    /// Decreases the speed by a specified amount.
    ///
    /// Note: `0 < to < current_speed`
    pub fn decrease_speed_to(&mut self, to: u8) {
        if self.speed < to {
            return;
        }
        self.speed = to;
    }
}

