use std::path::PathBuf;
use crate::Road;
use image::{ImageError, Rgb, RgbImage};

const SEPERATOR_COLOR: Rgb<u8> = Rgb([90, 90, 90]); // Rgb([255, 255, 255]);

pub struct ImageDrawer {
    image: RgbImage,
    current_row: u32,
    max_speed: u8,
    seperator: bool
}

impl ImageDrawer {
    pub fn new(road: &Road, rounds: u32) -> Self {
        let seperator = road.lanes() > 1;
        let round_height = road.lanes() + if seperator { 1 } else { 0 };
        let height = round_height * rounds;
        Self {
            image: RgbImage::new(road.length(), height),
            current_row: height,
            max_speed: road.max_speed(),
            seperator
        }
    }

    pub fn placeholder() -> Self {
        Self {
            image: RgbImage::new(0, 0),
            current_row: 0,
            max_speed: 0,
            seperator: false
        }
    }

    pub fn add_snapshot(&mut self, road: &Road) {
        if self.current_row == 0 {
            panic!("Image is already full.");
        }
        for lane in road.cells() {
            self.current_row -= 1;
            for (x, cell) in lane.iter().enumerate() {
                if let Some(car) = cell.car() {
                    self.image.put_pixel(
                        TryInto::<u32>::try_into(x).unwrap(),
                        self.current_row,
                        self.get_color(car.speed())
                    );
                }
            }
        }
        if self.seperator {
            self.current_row -= 1;
            for x in 0..self.image.width() {
                self.image.put_pixel(x, self.current_row, SEPERATOR_COLOR);
            }
        }
    }

    fn get_color(&self, speed: u8) -> Rgb<u8> {
        let speed_norm: f32 = Into::<f32>::into(speed) / Into::<f32>::into(self.max_speed);
        let mut red = 255;
        let mut green = 255;
        if speed_norm <= 0.5 {
            green = (255.0 * 2.0 * speed_norm).floor() as u8;
        } else {
            red = (255.0 * 2.0 * (1.0 - speed_norm)).floor() as u8;
        }
        Rgb([
            red,
            green,
            0
        ])
    }

    pub fn save(&self, filepath: PathBuf) -> Result<(), ImageError> {
        self.image.save(filepath)
    }
}
