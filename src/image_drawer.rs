use std::path::PathBuf;
use crate::Road;
use image::{ImageError, Rgb, RgbImage};

const SEPERATOR_COLOR: Rgb<u8> = Rgb([0, 60, 180]); // Rgb([255, 255, 255]);
const BLOCK_COLOR: Rgb<u8> = Rgb([180, 0, 180]);

#[derive(Debug)]
pub struct ImageDrawer {
    image: RgbImage,
    current_row: u32,
    road_lanes: u32,
    seperator: bool,
}

impl ImageDrawer {
    pub fn new(road: &Road, rounds: u32) -> Self {
        let seperator = road.lanes() > 1;
        let round_height = road.lanes() + if seperator { 1 } else { 0 };
        let height = round_height * rounds;
        Self {
            image: RgbImage::new(road.length(), height),
            current_row: height,
            road_lanes: road.lanes(),
            seperator
        }
    }

    pub fn placeholder() -> Self {
        Self {
            image: RgbImage::new(0, 0),
            current_row: 0,
            road_lanes: 0,
            seperator: false
        }
    }

    pub fn take_snapshot(&mut self, road: &Road) {
        if self.current_row == 0 {
            panic!("Image is already full.");
        }

        let last_row = self.current_row - self.road_lanes;
        self.current_row -= self.road_lanes;
        for (y, lane) in road.cells().iter().enumerate() {
            for (x, cell) in lane.iter().enumerate() {
                if cell.blocked() {
                    self.image.put_pixel(
                        TryInto::<u32>::try_into(x).unwrap(),
                        last_row + y as u32,
                        BLOCK_COLOR
                    );
                } else if let Some(car) = cell.car() {
                    self.image.put_pixel(
                        TryInto::<u32>::try_into(x).unwrap(),
                        last_row + y as u32,
                        Rgb(car.speed_rgb())
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

    pub fn save(&self, filepath: PathBuf) -> Result<(), ImageError> {
        self.image.save(filepath)
    }
}

