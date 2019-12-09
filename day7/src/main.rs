use std::io::prelude::*;
use std::io::BufReader;

use itertools::Itertools;

use intcode::Machine;

fn main() -> Result<(), std::io::Error> {
    let program_str = include_str!("../input1.txt");
    let program_state = program_str
        .split(",")
        .map(|v| v.replace('\n', "").parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let original_state = program_state.clone();

    let max_out_signal = (0..5).permutations(5).map(|perm| {
        perm.iter().fold(0, |in_signal, phase_setting| {
            let (rx, mut machine) = Machine::new(original_state.clone());
            machine.set_input_string(format!("{}\n{}", phase_setting, in_signal));
            machine.execute();
            if let Some(last) = rx.iter().last() {
                last
            } else {
                -1000
            }
        })
    }).max();

    println!("output: {:?}", max_out_signal);

    let max_out_signal = (5..10).permutations(5).map(|perm| {
        let mut out_rx = vec![];
        let mut machines = vec![];
        for _ in 0..5 {
            let (rx, machine) = Machine::new(original_state.clone());
            out_rx.push(rx);
            machines.push(machine);
        }
        perm.iter().enumerate().for_each(|(i, v)| {
           machines[i].set_input_string(v.to_string()); 
        });

        for (i, rx) in out_rx.drain(0..4).enumerate() {
            machines[i+1].set_input(intcode::InputType::IntReceiver(rx));
        }
        machines[0].set_input_string("0".to_string());

        for rx in out_rx.drain(..) {
            machines[0].set_input(intcode::InputType::IntReceiver(rx));
        }

        for machine in machines.drain(0..4) {
            machine.execute_async();
        }
        machines[0].execute(); // block on the last one finishing

        println!("{:?}", machines[0]);
        if let Some(last) = machines[0].get_output().iter().last() {
            *last
        } else {
            -1000
        }
    }).max();

    println!("output: {:?}", max_out_signal);


    Ok(())
}
