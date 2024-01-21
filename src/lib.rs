//! # Depth Analyzer
//!
//! Program that analyzes an image processed by [MiDaS AI](https://github.com/isl-org/MiDaS/tree/master).
//!
//! ## Usage
//!
//! `$ depth-analyzer /path/to/image.[jpg | png | webp]`
use image::GenericImageView;


#[derive(PartialEq, Debug)]
pub struct DangerSectors {
    left :bool,
    right :bool,
    center :bool
}

impl DangerSectors {
    pub fn new() -> DangerSectors {
        DangerSectors {
            left: false,
            right: false,
            center: false
        }
    }

    /// Modifies the dangerous sectors by analyzing the image.
    pub fn analyze(&mut self, img :&image::DynamicImage) {
        let image_width = img.dimensions().0;

        for (x, _, rgba) in img.pixels() {
            
            if rgba[0] >= 150 {

                if x < image_width / 3 {
                    self.left = true;

                } else if x < 2 * image_width / 3 {
                    self.center = true;

                } else {
                    self.right = true;

                }
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Instruction {
    Stop,
    Right,
    Left,
    Nil
}

/// Based on the dangers in each sector, determines the outcome for the user.
///
/// NOTE: Order of precedence is as follows: Center, Right, Left, Stop.
pub fn get_instruction(sectors :&DangerSectors) -> Instruction {
    if sectors.center == false {
        Instruction::Nil

    } else if sectors.right == false {
        Instruction::Right

    } else if sectors.left == false {
        Instruction::Left

    } else {
        Instruction::Stop

    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn all_dangers() {
        let sectors = DangerSectors {
            left: true,
            right: true,
            center: true
        };

        assert_eq!(Instruction::Stop, get_instruction(&sectors));
    }

    #[test]
    fn danger_left() {
        let sectors = DangerSectors {
            left: true,
            right: false,
            center: false
        };

        assert_eq!(Instruction::Nil, get_instruction(&sectors));
    }
    
    #[test]
    fn danger_right() {
        let sectors = DangerSectors {
            left: false,
            right: true,
            center: false
        };

        assert_eq!(Instruction::Nil, get_instruction(&sectors));

    }

    #[test]
    fn danger_center() {
        let sectors = DangerSectors {
            left: false,
            right: false,
            center: true
        };

        assert_eq!(Instruction::Right, get_instruction(&sectors));

    }

    #[test]
    fn orange_test() {
        let img = image::open("test_images/orange.png").unwrap();

        let mut sectors = DangerSectors::new();
        sectors.analyze(&img);

        let correct_sectors = DangerSectors {
            left: true,
            right: true,
            center: true
        };

        assert_eq!(correct_sectors, sectors);
    }

    #[test]
    fn blue_test() {
        let img = image::open("test_images/blue.jpg").unwrap();

        let mut sectors = DangerSectors::new();
        sectors.analyze(&img);

        let correct_sectors = DangerSectors {
            left: false,
            right: false,
            center: false
        };

        assert_eq!(correct_sectors, sectors);
    }
}
