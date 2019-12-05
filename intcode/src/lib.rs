#[derive(Debug, Clone)]
pub struct Machine {
    memory: Vec<i32>,
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
            let (instr, mode_op1, mode_op2, _) = self.get_instr_and_modes();
            println!("{}, {}, {}, {}", instr, mode_op1, mode_op2, "xx");
            match instr {
                99 => {
                    println!("HALT");
                    break;
                }
                1 => {
                    let op1 = self.load_with_mode(self.program_counter + 1, mode_op1);
                    let op2 = self.load_with_mode(self.program_counter + 2, mode_op2);
                    let out_reg = self.load(self.program_counter + 3);
                    println!("ADD {} {} into {}", op1, op2, out_reg);

                    self.store(out_reg, op1 + op2);
                }
                2 => {
                    let op1 = self.load_with_mode(self.program_counter + 1, mode_op1);
                    let op2 = self.load_with_mode(self.program_counter + 2, mode_op2);
                    let out_reg = self.load(self.program_counter + 3);
                    println!("MULT {} {} into {}", op1, op2, out_reg);

                    self.store(out_reg, op1 * op2);
                }
                3 => {
                    //assert!(mode_op1 == 1);
                    let op1_addr = self.load(self.program_counter + 1);
                    let mut input = String::new();
                    let stdin = std::io::stdin();
                    println!("input an i32 value: ");
                    stdin.read_line(&mut input).unwrap();
                    println!("STORE_INPUT {} to {}", input, op1_addr);

                    self.store(op1_addr, input.trim().parse::<i32>().unwrap());
                }
                4 => {
                    let addr = self.load(self.program_counter + 1);
                    let val = self.load(addr);
                    println!("OUTPUT addr {}: val: {}", addr, val);
                }
                _ => {
                    panic!("something broke!");
                }
            }

            println!("{:?}", self);
            match instr {
                3|4 => {
                    self.program_counter += 2;
                }
                _ => {
                    self.program_counter += 4;
                }
            }

        }
    }

    // ABCDE
    //  1002

    // DE - two-digit opcode,      02 == opcode 2
    // C - mode of 1st parameter,  0 == position mode
    // B - mode of 2nd parameter,  1 == immediate mode
    // A - mode of 3rd parameter,  0 == position mode,
       //                               omitted due to being a leading zero
    // Parameters that an instruction writes to will never be in immediate mode.
    // return opcode first and return modes as items 1, 2, 3 of tuple
    fn get_instr_and_modes(self: &Self) -> (i32, i32, i32, i32) {

        let instr = self.load(self.program_counter);
        let digits_str = instr.to_string();
        let digits = (0..digits_str.len()).map(|i| digits_str.get(i..i+1).unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        println!("{:?}", digits);

        match digits_str.len() {
            1 => {
                (digits[0], 0, 0, 0)
            }
            2 => {
                (digits[0] * 10 + digits[1], 0, 0, 0)
            }
            3 => {
                (digits[1] * 10 + digits[2], digits[0], 0, 0)
            }
            4 => {
                println!("digits[1]: {}, digits[0]: {}, digits[1] * 10 + digits[0]: {}", digits[1], digits[0], digits[1] * 10 + digits[0]);
                (digits[2] * 10 + digits[3], digits[1], digits[0], 0)
            }
            5 => {
                unreachable!("theoretically possible but shouldn't happen because param 3 should never be in immediate mode: {:?}", digits);
            }
            _ => {
                unreachable!("input integer too long: {:?}", digits);
            }
        }
    }

    pub fn output(self: &Self) -> i32 {
        self.memory[0]
    }

    pub fn set_noun(self: &mut Self, noun: i32) {
        self.memory[1] = noun;
    }

    pub fn set_verb(self: &mut Self, verb: i32) {
        self.memory[2] = verb;
    }

    fn load(self: &Self, addr: i32) -> i32 {
        let result = self.memory[addr as usize];
        println!("LOADING addr: {}, val: {}", addr, result);
        result
    }

    fn load_with_mode(self: &Self, addr: i32, mode: i32) -> i32 {
        match mode {
            0 => self.load(self.load(addr)),
            1 => self.load(addr),
            _ => unreachable!("invalid mode: {}", mode)
        }
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

    #[test]
    fn it_handles_input_instr_opcode_3() {
        let mut m = Machine::new(vec![3, 0, 99]);
        m.execute();
        println!("{:?}", m.memory);
        assert!(m.memory == vec![50, 0, 99]);
    }
}
