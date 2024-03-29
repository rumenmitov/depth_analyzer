use std::{process, path::PathBuf};

pub enum ModelProximityColor {
    RED,
    WHITE
}

pub struct ImageConfig {
    /// Specifies which color to use as an indicator for proximity.
    pub proximity_color :ModelProximityColor,
    /// Specifies the value a pixel must have in order to be considered to be of
    /// **proximity color**.
    pub threshold :u8,
    /// Specifies option for watching a directory for new images.
    pub watch_dir :Option<PathBuf>,
    pub img :Option<image::DynamicImage>,
}

impl ImageConfig {
    pub fn new(args :&mut impl Iterator<Item=String>) -> ImageConfig {
        args.next();

        let mut image_config = ImageConfig {
            proximity_color: ModelProximityColor::RED,
            threshold: 150,
            watch_dir: None,
            img: None,
        };

        let mut argv = args.into_iter().peekable();

        loop {
            match argv.peek() {
                Some(arg_option) if arg_option == "-c" || arg_option == "--color" => {

                    image_config.proximity_color = match argv.next() {
                        Some(arg_option)  => {
                            if arg_option.to_lowercase() == "red" {
                                ModelProximityColor::RED

                            } else if arg_option.to_lowercase() == "white" {
                                ModelProximityColor::WHITE

                            } else {
                                eprintln!("Error: Please specify either RED or WHITE as a proximity color.\
For more information please use the -h option. \n");
                                process::exit(1);

                            }
                        },

                        None => image_config.proximity_color
                    };

                    argv.next();
                },

                Some(arg_option) if arg_option == "-t" || arg_option == "--threshold" => {

                    image_config.threshold = match argv.next() {
                        Some(arg_option) => {
                            let threshold :u8 = match arg_option.trim().parse() {
                                Ok(num) => num,
                                _ => {
                                    eprintln!("Error: Invalid threshold entered! Please provide a value between 0 and 255!\
For more information please use the -h option.\n");

                                    process::exit(1);
                                }
                            };

                            threshold
                        },

                        None => image_config.threshold
                    };

                    argv.next();

                },

                Some(arg_option) if arg_option == "-w" || arg_option == "--watch" => {
                    // NOTE: Move argv iterator once more, as -w flag is only peeked.
                    argv.next();

                    image_config.watch_dir = match argv.next() {
                        Some(path) => Some(PathBuf::from(path)),
                        None => Some(
                            std::env::current_dir()
                            .expect("Error! Could not read current directory.")
                            )
                    };

                    argv.next();
                }

                Some(arg_option) if arg_option == "-h" || arg_option == "--help" => {
                    println!("\
\n
  /----------------/
 / Depth Analyzer /
/----------------/


Program that analyzes an image processed by depth-detection AI models.


Version: 

    0.2.0

Usage:

    depth-analyzer /path/to/image.[ jpg | jpeg | png | webp ]

Options:

    -h, --help                          Displays this help menu.

    -v, --version                       Displays version.

    -c, --color [ RED | WHITE ]         Specifies which color to 
                                        use as an indicator for proximity.

    -t, --threshold [ 0 .. 255 ]        Specifies the value a pixel must have in order to be 
                                        considered to be of the proximity color.

    -w, --watch [ /path/to/images ]     Analyze images as they come in. If no path is
                                        provided, current directory is used.

Possible Results (in order of precedence):

    FORWARD
    RIGHT
    LEFT
    STOP

");
                    argv.next();
                }, 

                Some(arg_option) if arg_option == "-v" || arg_option == "--version" => {
                    println!("\
\nDepth Analyzer v0.2.0

");
                    process::exit(0);
                },

                Some(img_path) => {

                    image_config.img = match image::open(img_path) {
                        Ok(img_result) => Some(img_result),
                        _ => {
                            eprintln!("Error: Could not open file: {}", img_path); 
                            process::exit(1);
                        }
                    };

                    argv.next();
                },

                None => break,
            }

        }

        image_config
    }
}

pub fn check_threshold(proximity_color :&ModelProximityColor, threshold :u8, rgba :&image::Rgba<u8>) -> bool {
    match proximity_color {

        ModelProximityColor::RED =>  rgba[0] >= threshold,

        ModelProximityColor::WHITE => rgba[0] >= threshold && 
            rgba[1] >= threshold && 
            rgba[2] >= threshold,

    }
}
