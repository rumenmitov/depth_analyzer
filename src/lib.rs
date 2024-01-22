//! # Depth Analyzer
//!
//! Program that analyzes an image processed by [MiDaS AI](https://github.com/isl-org/MiDaS/tree/master).
//!
//! ## Usage
//!
//! `$ depth-analyzer /path/to/image.[jpg | png | webp]`


use std::fmt::Display;

use image::GenericImageView;

/// The value for the color that indicates that an object is close.
static PROXIMITY_COLOR :u8 = 150;


#[derive(PartialEq, Debug)]
pub enum Instruction {
    Left,
    Forward,
    Right,
    Stop
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Right => write!(f, "{}", "RIGHT"),
            Self::Left => write!(f, "{}", "LEFT"),
            Self::Forward => write!(f, "{}", "FORWARD"),
            Self::Stop => write!(f, "{}", "STOP"),
        }
    }
}

/// Each sector of the image has a **total red pixels** and a *total pixel* count.
/// A **red pixel** is a pixel that has a Red value higher than 150.
#[derive(PartialEq, Debug)]
pub struct DangerSectors {
    left :(u32, u32),
    center :(u32, u32),
    right :(u32, u32),
}

impl DangerSectors {
    pub fn new() -> DangerSectors {
        DangerSectors {
            left: (0, 0),
            center: (0, 0),
            right: (0, 0),
        }
    }

    /// Updates sectors by analyzing the image.
    pub fn analyze(&mut self, img :&image::DynamicImage) {
        let image_width = img.dimensions().0;

        for (x, _, rgba) in img.pixels() {
            
            if x < image_width / 3 {

                if rgba[0] >= PROXIMITY_COLOR {
                    self.left.0 += 1;
                }
                self.left.1 += 1;

            } else if x < 2 * image_width / 3 {

                if rgba[0] >= PROXIMITY_COLOR {
                    self.center.0 += 1;
                }
                self.center.1 += 1;

            } else {

                if rgba[0] >= PROXIMITY_COLOR {
                    self.right.0 += 1;
                }
                self.right.1 += 1;

            }
        }
    }

    /// Gets the ratio of *red pixels* versus *total pixels* for each sector.
    fn get_ratios(&self) -> (f32, f32, f32) {

        (
            self.left.0 as f32 / self.left.1 as f32,
            self.center.0  as f32 / self.center.1 as f32,
            self.right.0 as f32 / self.right.1 as f32,
        )
    }

    /// Based on the dangers in each sector, determines the outcome for the user.
    ///
    /// NOTE: The precedence is **Forward**, **Right**, **Left**
    /// NOTE: If the ratio of Red pixels in each sector is >= 50% then **Stop**.
    pub fn get_instruction(&self) -> Instruction {
        let (left, center, right) = self.get_ratios();

        if left >= 0.5 && center >= 0.5 && right >= 0.5 {
            Instruction::Stop

        } else if center <= right && center <= left {
            Instruction::Forward

        } else if right < center && right <= left {
            Instruction::Right

        } else {
            Instruction::Left

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orange() {
        let img = image::open("test_images/orange.png").unwrap();

        let mut sectors = DangerSectors::new();
        sectors.analyze(&img);

        assert_eq!(Instruction::Stop, sectors.get_instruction());
    }

    #[test]
    fn blue() {
        let img = image::open("test_images/blue.jpg").unwrap();

        let mut sectors = DangerSectors::new();
        sectors.analyze(&img);

        assert_eq!(Instruction::Forward, sectors.get_instruction());
    }

    #[test]
    fn room() {
        let img = image::open("test_images/empty_room.png").unwrap();

        let mut sectors = DangerSectors::new();
        sectors.analyze(&img);

        assert_eq!(Instruction::Forward, sectors.get_instruction());
    }

    #[test]
    fn chair() {
        let img = image::open("test_images/chair.png").unwrap();

        let mut sectors = DangerSectors::new();
        sectors.analyze(&img);

        assert_eq!(Instruction::Right, sectors.get_instruction());
    }
    
}
