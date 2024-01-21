use std::env;

use depth_analyzer::*;

fn main() {

    let args :Vec<String> = env::args().collect();
    let img = image::open(&args[1]).unwrap();

    let mut sectors = DangerSectors::new();
    sectors.analyze(&img);

    let instruction = get_instruction(&sectors);

    match instruction {
        Instruction::Stop => println!("STOP"),
        Instruction::Right => println!("RIGHT"),
        Instruction::Left => println!("LEFT"),
        Instruction::Nil => println!("NIL"),
    }

}
