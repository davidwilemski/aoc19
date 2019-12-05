use std::io::prelude::*;
use std::io::BufReader;

use intcode::Machine;

fn main() -> Result<(), std::io::Error> {
    // let stdin = std::io::stdin();
    // let reader = BufReader::new(stdin);
    let program_str = include_str!("../input1.txt");

    let program_state = program_str
        .split(",")
        .map(|v| v.replace('\n', "").parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let original_state = program_state.clone();

    let mut machine = Machine::new(original_state.clone());
    machine.execute();

    Ok(())
}
