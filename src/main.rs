use std::env;

fn main() {

    let mut image_config = depth_analyzer::config::ImageConfig::new(&mut env::args());

    let mut sectors = depth_analyzer::DangerSectors::new();

    sectors.analyze(&mut image_config);

    println!("{}", sectors.get_instruction().to_string());
}
