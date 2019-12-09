use std::io::prelude::*;
use std::io::BufReader;

use intcode::Machine;

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin);

    let program_state = reader
        .split(b',')
        .map(|v: std::io::Result<Vec<u8>>| String::from_utf8(v.unwrap()))
        .filter(|v| {
            assert!(v.is_ok());
            v.is_ok()
        })
        .map(|v| v.ok())
        .map(|v| v.unwrap())
        .map(|v| v.replace('\n', "").parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let original_state = program_state.clone();

    // We have to muck with program memory before execution given the instructions: "before running
    // the program, replace position 1 with the value 12 and replace position 2 with the value 2"
    // What is 100 * noun + verb? (For example, if noun=12 and verb=2, the answer would be 1202.)
    // machine.memory[1] = 12;  <--- noun
    // machine.memory[2] = 2;   <--- verb
    // machine.execute();
    // println!("value at memory position 0: {}", machine.memory[0]);

    for noun in 0..99 {
        for verb in 0..99 {
            println!("noun: {}, verb: {}", noun, verb);
            let (_, mut machine) = Machine::new(original_state.clone());
            machine.set_noun(noun);
            machine.set_verb(verb);
            machine.execute();
            if machine.output() == 19690720 {
                println!(
                    "100 * noun + verb == 100 * {} + {} == {}",
                    noun,
                    verb,
                    100 * noun + verb
                );
                return Ok(());
            }
        }
    }

    Ok(())
}
