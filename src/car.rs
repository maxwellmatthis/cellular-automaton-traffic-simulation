pub struct Car {
    max_speed: u8,
    speed: u8,
    rounds: u32,
    distance: u32
}

impl Car {
    pub fn new(max_speed: u8, initial_speed: u8) -> Self {
        return Self {
            max_speed,
            speed: initial_speed,
            rounds: 0,
            distance: 0
        }
    }

    pub fn speed(&self) -> u8 {
        return self.speed;
    }

    pub fn rounds(&self) -> u32 {
        return self.rounds;
    }

    // Returns the average number of cells driven per round.
    pub fn average_speed(&self) -> f64 {
        return Into::<f64>::into(self.distance) / Into::<f64>::into(self.rounds);
    }

    // Records the current round by incrementing the rounds counter and adding the distance
    // traveled to the total.
    pub fn record(&mut self) {
        self.rounds += 1;
        self.distance += Into::<u32>::into(self.speed());
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

