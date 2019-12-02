use crate::computer::Instruction::{Start, Halt};

pub struct Computer {
    instruction_pointer: usize,
    instruction: Instruction,
    memory: Vec<i64>
}

#[derive(PartialOrd, PartialEq, Debug)]
enum Instruction {
    Start,
    Add { input1: i64, input2: i64, output: i64 },
    Multiply { input1: i64, input2: i64, output: i64 },
    Halt
}

impl Computer {
    pub fn new(memory: Vec<i64>) -> Self {
        Computer {
            instruction_pointer: 0,
            instruction: Start,
            memory
        }
    }

    pub fn get_memory(&self, address: i64) -> i64 {
        self.memory[address as usize]
    }

    pub fn set_noun(&mut self, value: i64) -> &mut Self {
        self.memory[1] = value;
        self
    }

    pub fn set_verb(&mut self, value: i64) -> &mut Self {
        self.memory[2] = value;
        self
    }

    fn set_memory(&mut self, address: i64, value: i64) {
        self.memory[address as usize] = value;
    }

    pub fn execute(&mut self) {
        while self.instruction != Halt {
            self.execute_next_instruction();
        }
    }

    pub fn execute_next_instruction(&mut self) {
        self.get_next_instruction();
        self.execute_instruction();
    }

    fn get_parameter(&self, number: i64) -> i64 {
        self.memory[self.instruction_pointer + number as usize]
    }

    fn get_next_instruction(&mut self) {
        match &self.instruction {
            Start | Halt => {},
            Instruction::Add {..} | Instruction::Multiply {..} => self.instruction_pointer += 4,
        }

        let opcode = self.memory[self.instruction_pointer];
        match opcode {
            99 => self.instruction = Instruction::Halt,
            1 => self.instruction = Instruction::Add {
                input1: self.get_memory(self.get_parameter(1)),
                input2: self.get_memory(self.get_parameter(2)),
                output: self.get_parameter(3),
            },
            2 => self.instruction = Instruction::Multiply {
                input1: self.get_memory(self.get_parameter(1)),
                input2: self.get_memory(self.get_parameter(2)),
                output: self.get_parameter(3),
            },
            _ => unreachable!()
        }
    }

    fn execute_instruction(&mut self) {
        match self.instruction {
            Instruction::Add { input1: i1, input2: i2, output: o} => self.set_memory(o, i1 + i2),
            Instruction::Multiply { input1: i1, input2: i2, output: o} => self.set_memory(o, i1 * i2),
            _ => {},
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_1() {
        let input = vec![1,0,0,0,99];
        let should_be = vec![2,0,0,0,99];

        let mut computer = Computer::new(input);
        computer.execute();

        assert_eq!(computer.memory, should_be);
    }

    #[test]
    fn test_program_2() {
        let input = vec![2,3,0,3,99];
        let should_be = vec![2,3,0,6,99];

        let mut computer = Computer::new(input);
        computer.execute();

        assert_eq!(computer.memory, should_be);
    }

    #[test]
    fn test_program_3() {
        let input = vec![2,4,4,5,99,0];
        let should_be = vec![2,4,4,5,99,9801];

        let mut computer = Computer::new(input);
        computer.execute();

        assert_eq!(computer.memory, should_be);
    }

    #[test]
    fn test_program_4() {
        let input = vec![1,1,1,4,99,5,6,0,99];
        let should_be = vec![30,1,1,4,2,5,6,0,99];

        let mut computer = Computer::new(input);
        computer.execute();

        assert_eq!(computer.memory, should_be);
    }
}

