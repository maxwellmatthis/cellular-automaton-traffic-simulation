use std::cmp::Ordering;
use std::cmp;
use std::str::FromStr;
use serde::Serialize;
use crate::flip_flop::FlipFlop;

#[derive(Debug)]
pub struct Car {
    max_speed: u8,
    acceleration_time: u8,
    acceleration_time_accumulated: u8,
    last_speed: u8,
    speed: u8,
    distance: u32,
    accelerations: u32,
    deaccelerations: u32,
    overflow_flip_flop: FlipFlop
}

impl Car {
    pub fn new(vehicle_blueprint: &VehicleBlueprint) -> Self {
        const INITIAL_SPEED: u8 = 0;
        Self {
            max_speed: vehicle_blueprint.max_speed,
            acceleration_time: vehicle_blueprint.acceleration_time,
            acceleration_time_accumulated: 0,
            last_speed: INITIAL_SPEED,
            speed: INITIAL_SPEED,
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
        self.acceleration_time_accumulated += 1;
        if self.acceleration_time_accumulated != self.acceleration_time {
            return;
        }
        self.acceleration_time_accumulated = 0;
        if self.speed == self.max_speed {
            return; 
        }
        self.speed += 1;
    }

    /// Decreases the speed by one if the car is not already stopped.
    fn decrease_speed(&mut self) { 
        self.acceleration_time_accumulated = 0;
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
            self.acceleration_time_accumulated = cmp::min(self.acceleration_time_accumulated, to);
            return;
        }
        self.speed = to;
    }
}

#[derive(Serialize, Debug)]
pub struct VehicleBlueprint {
    max_speed: u8,
    acceleration_time: u8,
    traffic_density: f32,
}

impl VehicleBlueprint {
    pub fn max_speed(&self) -> u8 {
        self.max_speed
    }

    pub fn acceleration_time(&self) -> u8 {
        self.acceleration_time
    }

    pub fn traffic_density(&self) -> f32 {
        self.traffic_density
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseVehicleBlueprintError;

impl FromStr for VehicleBlueprint {
    type Err = ParseVehicleBlueprintError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: String = s.replace(' ', "");
        let inner = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .ok_or(ParseVehicleBlueprintError)?;

        let split: Vec<&str> = inner.split(',').collect();
        let (max_speed, acceleration_time, traffic_density) = if split.len() == 3 {
            (
                split[0].parse::<u8>().map_err(|_| ParseVehicleBlueprintError)?,
                split[1].parse::<u8>().map_err(|_| ParseVehicleBlueprintError)?,
                split[2].parse::<f32>().map_err(|_| ParseVehicleBlueprintError)?
            )
        } else {
            return Err(ParseVehicleBlueprintError);
        };

        Ok(VehicleBlueprint { max_speed, acceleration_time, traffic_density })
    }
}

