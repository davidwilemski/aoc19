use std::io::prelude::*;
use std::io::BufReader;

use itertools::Itertools;

use intcode::Machine;

fn main() -> Result<(), std::io::Error> {
    let program_str = include_str!("../input1.txt");
    let program_state = program_str
        .split(",")
        .map(|v| v.replace('\n', "").parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let original_state = program_state.clone();

    let max_out_signal = (0..5).permutations(5).map(|perm| {
        perm.iter().fold(0, |in_signal, phase_setting| {
            let mut machine = Machine::new(original_state.clone());
            machine.set_input(format!("{}\n{}", phase_setting, in_signal));
            machine.execute();
            if let Some(last) = machine.get_output().last() {
                *last
            } else {
                -1000
            }
        })
    }).max();

    println!("output: {:?}", max_out_signal);


    Ok(())
}
