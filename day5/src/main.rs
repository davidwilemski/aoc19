use std::io::prelude::*;
use std::io::BufReader;

use intcode::Machine;

fn main() -> Result<(), std::io::Error> {
    // let stdin = std::io::stdin();
    // let reader = BufReader::new(stdin);
    let program_str = include_str!("../input1.txt");

    let program_state = program_str
        .split(",")
        .map(|v| v.replace('\n', "").parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let original_state = program_state.clone();

    let (_, mut machine) = Machine::new(original_state.clone());
    machine.set_input_string("1\n".to_string());
    machine.execute();

    println!("program output: {:?}", machine.get_output());

    Ok(())
}
