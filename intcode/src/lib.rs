#[derive(Debug, Clone)]
pub struct Machine {
    pub memory: Vec<i32>,
    program_counter: i32,
}

impl Machine {
    pub fn new(memory: Vec<i32>) -> Self {
        Machine {
            memory,
            program_counter: 0,
        }
    }

    pub fn execute(self: &mut Self) {
        loop {
            match self.load(self.program_counter) {
                99 => {
                    println!("HALT");
                    break;
                }
                1 => {
                    let op1 = self.load(self.load(self.program_counter + 1));
                    let op2 = self.load(self.load(self.program_counter + 2));
                    let out_reg = self.load(self.program_counter + 3);
                    println!("ADD {} {} into {}", op1, op2, out_reg);

                    self.store(out_reg, op1 + op2);
                }
                2 => {
                    let op1 = self.load(self.load(self.program_counter + 1));
                    let op2 = self.load(self.load(self.program_counter + 2));
                    let out_reg = self.load(self.program_counter + 3);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_handles_test_cases() {
        let mut m = Machine::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        m.execute();
        println!("{:?}", m.memory);
        assert!(m.memory == vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);

        let mut m = Machine::new(vec![1, 0, 0, 0, 99]);
        m.execute();
        println!("{:?}", m.memory);
        assert!(m.memory == vec![2, 0, 0, 0, 99]);

        let mut m = Machine::new(vec![2, 3, 0, 3, 99]);
        m.execute();
        println!("{:?}", m.memory);
        assert!(m.memory == vec![2, 3, 0, 6, 99]);

        let mut m = Machine::new(vec![2, 4, 4, 5, 99, 0]);
        m.execute();
        println!("{:?}", m.memory);
        assert!(m.memory == vec![2, 4, 4, 5, 99, 9801]);

        let mut m = Machine::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        m.execute();
        println!("{:?}", m.memory);
        assert!(m.memory == vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
