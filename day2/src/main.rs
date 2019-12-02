use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Clone)]
struct Machine {
    memory: Vec<i32>,
    program_counter: i32,
}

impl Machine {
    fn new(memory: Vec<i32>) -> Self {
        Machine {
            memory,
            program_counter: 0,
        }
    }

    fn pc(self: &Self) -> i32 {
        self.program_counter
    }

    fn execute(self: &mut Self) {
        loop {
            match self.load(self.program_counter) {
                99 => {
                    println!("HALT");
                    break;
                }
                1 => {
                    let op1 = self.load(self.load(self.pc() + 1));
                    let op2 = self.load(self.load(self.pc() + 2));
                    let out_reg = self.load(self.pc() + 3);
                    println!("ADD {} {} into {}", op1, op2, out_reg);

                    self.store(out_reg, op1 + op2);
                }
                2 => {
                    let op1 = self.load(self.load(self.pc() + 1));
                    let op2 = self.load(self.load(self.pc() + 2));
                    let out_reg = self.load(self.pc() + 3);
                    println!("MULT {} {} into {}", op1, op2, out_reg);

                    self.store(out_reg, op1 * op2);
                }
                _ => {
                    panic!("something broke!");
                }
            }

            println!("{:?}", self);
            self.program_counter += 4;
        }
    }

    fn load(self: &Self, addr: i32) -> i32 {
        let result = self.memory[addr as usize];
        println!("LOADING addr: {}, val: {}", addr, result);
        result
    }

    fn store(self: &mut Self, addr: i32, val: i32) {
        self.memory[addr as usize] = val;
    }
}

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
        .map(|v| v.replace('\n', "").parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

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
            let mut machine = Machine::new(original_state.clone());
            machine.memory[1] = noun;
            machine.memory[2] = verb;
            machine.execute();
            if machine.memory[0] == 19690720 {
                println!("100 * noun + verb == 100 * {} + {} == {}", noun, verb, 100 * noun + verb);
                return Ok(());
            }
        }
    }


    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_handles_test_cases() {

        let mut m = Machine::new(vec![1,9,10,3,2,3,11,0,99,30,40,50]);
        m.execute();
        println!("{:?}", m.memory);
        assert!(m.memory == vec![3500,9,10,70,2,3,11,0,99,30,40,50]);

        let mut m = Machine::new(vec![1,0,0,0,99]);
        m.execute();
        println!("{:?}", m.memory);
        assert!(m.memory == vec![2,0,0,0,99]);

        let mut m = Machine::new(vec![2,3,0,3,99]);
        m.execute();
        println!("{:?}", m.memory);
        assert!(m.memory == vec![2,3,0,6,99]);

        let mut m = Machine::new(vec![2,4,4,5,99,0]);
        m.execute();
        println!("{:?}", m.memory);
        assert!(m.memory == vec![2,4,4,5,99,9801]);

        let mut m = Machine::new(vec![1,1,1,4,99,5,6,0,99]);
        m.execute();
        println!("{:?}", m.memory);
        assert!(m.memory == vec![30,1,1,4,2,5,6,0,99]);
    }
}
