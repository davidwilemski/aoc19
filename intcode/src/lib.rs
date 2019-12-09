use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};

trait ReadLine<T> {
    fn read_line(&self, buf: &mut String) -> std::io::Result<usize>;
}

#[derive(Debug)]
pub enum InputType {
    StringCursor(io::Cursor<String>),
    IntReceiver(Receiver<i64>),
}

#[derive(Debug)]
pub struct Machine {
    memory: Vec<i64>,
    program_counter: i64,
    input: VecDeque<InputType>,
    output_tx: Option<SyncSender<i64>>,
    output: Vec<i64>,
}

#[derive(Debug)]
struct Instruction {
    length: usize,
    opcode: i32,
    mode_op1: i32,
    mode_op2: i32,
    mode_op3: i32, // basically do not use
}

impl Machine {
    pub fn new(memory: Vec<i64>) -> (Receiver<i64>, Self) {
        // "available memory should be much larger than the initial program"
        // I'm also setting an upper bount on size at 2^30 for memory reasons
        let mut extended_mem = vec![0; (memory.len() * 1024).min(1073741824)];
        for (i, val) in memory.iter().enumerate() {
            extended_mem[i] = *val;
        }
        let (tx, rx) = sync_channel(1024);
        (rx, Machine {
            memory: extended_mem,
            program_counter: 0,
            input: VecDeque::new(),
            output_tx: Some(tx),
            output: vec![],
        })
    }

    pub fn execute(self: &mut Self) {
        loop {
            let instr = self.get_instr_and_modes();
            println!("tid {:?}: Instr: {:?}", thread_id(), instr);
            match instr.opcode {
                99 => {
                    println!("tid {:?}: HALT", thread_id());
                    self.output_tx = None;
                    break;
                }
                1 => {
                    let op1 = self.load_with_mode(self.program_counter + 1, instr.mode_op1);
                    let op2 = self.load_with_mode(self.program_counter + 2, instr.mode_op2);
                    let out_reg = self.load(self.program_counter + 3);
                    println!("tid {:?}: ADD {} {} into {}", thread_id(), op1, op2, out_reg);

                    self.store(out_reg, op1 + op2);
                }
                2 => {
                    let op1 = self.load_with_mode(self.program_counter + 1, instr.mode_op1);
                    let op2 = self.load_with_mode(self.program_counter + 2, instr.mode_op2);
                    let out_reg = self.load(self.program_counter + 3);
                    println!("tid {:?}: MULT {} {} into {}", thread_id(), op1, op2, out_reg);

                    self.store(out_reg, op1 * op2);
                }
                3 => {
                    //assert!(mode_op1 == 1);
                    let op1_addr = self.load(self.program_counter + 1);
                    let line = self.get_input();
                    println!("tid {:?}: STORE_INPUT {} to {}", thread_id(), line, op1_addr);

                    self.store(op1_addr, line.trim().parse::<i64>().unwrap());
                }
                4 => {
                    let val = self.load_with_mode(self.program_counter + 1, instr.mode_op1);
                    println!("tid {:?}: OUTPUT val: {}", thread_id(), val);

                    self.output.push(val);
                    match &self.output_tx {
                        Some(tx) => tx.send(val).unwrap_or(()),
                        None => panic!("missing output tx"),
                    }
                }
                5 => {  // jump-if-true
                    let op1 = self.load_with_mode(self.program_counter + 1, instr.mode_op1);
                    let op2 = self.load_with_mode(self.program_counter + 2, instr.mode_op2);
                    if op1 != 0 {
                        self.program_counter = op2;
                        continue; // we don't want to increment the PC like normal
                    }
                }
                6 => {  // jump-if-false
                    let op1 = self.load_with_mode(self.program_counter + 1, instr.mode_op1);
                    let op2 = self.load_with_mode(self.program_counter + 2, instr.mode_op2);
                    if op1 == 0 {
                        self.program_counter = op2;
                        continue; // we don't want to increment the PC like normal
                    }
                }
                7 => {  // less-than
                    let op1 = self.load_with_mode(self.program_counter + 1, instr.mode_op1);
                    let op2 = self.load_with_mode(self.program_counter + 2, instr.mode_op2);
                    let out_reg = self.load(self.program_counter + 3);
                    if op1 < op2 {
                        self.store(out_reg, 1);
                    } else {
                        self.store(out_reg, 0);
                    }
                }
                8 => {  // equals
                    let op1 = self.load_with_mode(self.program_counter + 1, instr.mode_op1);
                    let op2 = self.load_with_mode(self.program_counter + 2, instr.mode_op2);
                    let out_reg = self.load(self.program_counter + 3);
                    if op1 == op2 {
                        self.store(out_reg, 1);
                    } else {
                        self.store(out_reg, 0);
                    }
                }
                _ => {
                    panic!("something broke!");
                }
            }

            // println!("{:?}", self);
            self.program_counter += match instr.opcode {
                1|2|7|8 => 4,
                3|4 => 2,
                5|6 => 3,
                _ => unreachable!("invalid opcode")
            }
        }

    }

    pub fn execute_async(mut self: Self) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || {
            self.execute();
        })
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
    fn get_instr_and_modes(self: &Self) -> Instruction {

        let instr = self.load(self.program_counter);
        let digits_str = instr.to_string();
        let digits = (0..digits_str.len()).map(|i| digits_str.get(i..i+1).unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        // println!("{:?}", digits);

        match digits_str.len() {
            1 => {
                Instruction { length: digits.len(), opcode: digits[0], mode_op1: 0, mode_op2: 0, mode_op3: 0}
            }
            2 => {
                Instruction { length: digits.len(), opcode: digits[0] * 10 + digits[1], mode_op1: 0, mode_op2: 0, mode_op3: 0}
            }
            3 => {
                Instruction { length: digits.len(), opcode: digits[1] * 10 + digits[2], mode_op1: digits[0], mode_op2: 0, mode_op3: 0}
            }
            4 => {
                Instruction { length: digits.len(), opcode: digits[2] * 10 + digits[3], mode_op1: digits[1], mode_op2: digits[0], mode_op3: 0}
            }
            5 => {
                unreachable!("theoretically possible but shouldn't happen because param 3 should never be in immediate mode: {:?}", digits);
            }
            _ => {
                unreachable!("input integer too long: {:?}", digits);
            }
        }
    }

    pub fn output(self: &Self) -> i64 {
        self.memory[0]
    }

    fn get_input(&mut self) -> String {
        if let Some(input_type) = self.input.pop_front() {
            match input_type {
                InputType::StringCursor(mut c) => {
                    let mut input = String::new();
                    match c.read_line(&mut input) {
                        Ok(_) => {
                            if input != "" {
                                self.input.push_front(InputType::StringCursor(c));
                                input
                            }
                            else {
                                self.get_input()
                            }
                        },
                        Err(_) => self.get_input(),
                    }
                },
                InputType::IntReceiver(rx) => {
                    println!("tid {:?}: waiting on rx for input...", thread_id());
                    match rx.recv() {
                        Ok(input) => {
                            self.input.push_front(InputType::IntReceiver(rx));
                            input.to_string()
                        }
                        Err(_) => self.get_input()
                    }

                },
            }
        } else {
            println!("tid {:?}: input an i64 value: ", thread_id());
            let mut input = String::new();
            let stdin = std::io::stdin();
            stdin.read_line(&mut input).unwrap();
            input
        }
    }

    pub fn get_output(&self) -> &Vec<i64> {
        &self.output
    }

    pub fn set_input(self: &mut Self, input: InputType) {
        self.input.push_back(input);
    }

    pub fn set_input_string(self: &mut Self, input: String) {
        self.input.push_back(InputType::StringCursor(io::Cursor::new(input)));
    }


    pub fn set_noun(self: &mut Self, noun: i64) {
        self.memory[1] = noun;
    }

    pub fn set_verb(self: &mut Self, verb: i64) {
        self.memory[2] = verb;
    }

    fn load(self: &Self, addr: i64) -> i64 {
        let result = self.memory[addr as usize];
        println!("tid {:?}: LOADING addr: {}, val: {}", thread_id(), addr, result);
        result
    }

    fn load_with_mode(self: &Self, addr: i64, mode: i32) -> i64 {
        match mode {
            0 => self.load(self.load(addr)),
            1 => self.load(addr),
            _ => unreachable!("invalid mode: {}", mode)
        }
    }

    fn store(self: &mut Self, addr: i64, val: i64) {
        self.memory[addr as usize] = val;
    }
}

fn thread_id() -> std::thread::ThreadId {
    std::thread::current().id()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_handles_test_cases() {
        let (_, mut m) = Machine::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        m.execute();
        println!("{:?}", m.memory);
        assert!(m.memory == vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);

        let (_, mut m) = Machine::new(vec![1, 0, 0, 0, 99]);
        m.execute();
        println!("{:?}", m.memory);
        assert!(m.memory == vec![2, 0, 0, 0, 99]);

        let (_, mut m) = Machine::new(vec![2, 3, 0, 3, 99]);
        m.execute();
        println!("{:?}", m.memory);
        assert!(m.memory == vec![2, 3, 0, 6, 99]);

        let (_, mut m) = Machine::new(vec![2, 4, 4, 5, 99, 0]);
        m.execute();
        println!("{:?}", m.memory);
        assert!(m.memory == vec![2, 4, 4, 5, 99, 9801]);

        let (_, mut m) = Machine::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        m.execute();
        println!("{:?}", m.memory);
        assert!(m.memory == vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn it_handles_input_instr_opcode_3() {
        let (_, mut m) = Machine::new(vec![3, 0, 99]);
        m.set_input_string("50".to_string());
        m.execute();
        println!("{:?}", m.memory);
        assert_eq!(m.memory, vec![50, 0, 99]);
    }

    #[test]
    fn it_supports_large_numbers() {
        let (rx, mut m) = Machine::new(vec![104,1125899906842624,99]);
        m.execute();

        assert_eq!(rx.recv().unwrap(), 1125899906842624);
    }
}
