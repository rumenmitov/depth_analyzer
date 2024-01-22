use std::env;

fn main() {

    let image_config = depth_analyzer::config::ImageConfig::new(&mut env::args());

    let mut sectors = depth_analyzer::DangerSectors::new();

    // TODO: Replace &img with &imgage_config
    sectors.analyze(image_config);

    println!("{}", sectors.get_instruction().to_string());
}
