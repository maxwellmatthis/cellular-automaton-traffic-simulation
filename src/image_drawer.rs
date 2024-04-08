use crate::Road;
use image::{ImageError, Rgb, RgbImage};

pub struct ImageDrawer {
    image: RgbImage,
    current_row: u32,
    max_speed: u8,
}

impl ImageDrawer {
    pub fn new(road: &Road, rounds: u32) -> Self {
        return Self {
            image: RgbImage::new(TryInto::<u32>::try_into(road.cells().len()).unwrap(), rounds),
            current_row: rounds,
            max_speed: road.max_speed()
        };
    }

    pub fn add_snapshot(&mut self, road: &Road) {
        if self.current_row == 0 {
            panic!("Image is already full.");
        }
        self.current_row -= 1;
        for (x, cell) in road.cells().iter().enumerate() {
            if let Some(car) = cell.car() {
                self.image.put_pixel(
                    TryInto::<u32>::try_into(x).unwrap(),
                    self.current_row,
                    self.get_color(car.speed())
                );
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
        return Rgb([
            red,
            green,
            0
        ])
    }

    pub fn save(&self, filepath: String) -> Result<(), ImageError> {
        return self.image.save(filepath);
    }
}
