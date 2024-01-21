use std::{env, process};

use depth_analyzer::*;

fn main() {

    let args :Vec<String> = env::args().collect();
    let img = match image::open(&args[1]) {
        Ok(img_result) => img_result,
        _ => {
            eprintln!("Could not open file: {}", args[1]); 
            process::exit(1);
        }
    };

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
