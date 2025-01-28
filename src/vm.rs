use crate::{
    assembler::{PIE_HEADER_LENGTH, PIE_HEADER_PREFIX},
    instruction::Opcode,
};

// Emulate cpu
#[derive(Debug, PartialEq)]
pub struct Vm {
    pub registers: [i32; 32],
    pc: usize,
    heap: Vec<u8>,
    pub program: Vec<u8>,
    remainder: u32,
    equal_flag: bool,
    ro_data: Vec<u8>,
}
impl Default for Vm {
    fn default() -> Self {
        Self::new()
    }
}
impl Vm {
    pub fn new() -> Vm {
        Vm {
            registers: [0; 32],
            pc: 64,
            heap: vec![],
            program: vec![],
            remainder: 0,
            equal_flag: false,
            ro_data: vec![],
        }
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    fn match_opcode(&mut self, opcode: Opcode) -> bool {
        match opcode {
            Opcode::LOAD => {
                let register: usize = self.next_8_bits() as usize;
                let number: u16 = self.next_16_bits();
                self.registers[register] = number as i32;
            }
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            }
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            }
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            }
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }
            Opcode::JMPB => {
                self.pc -= self.registers[self.next_8_bits() as usize] as usize;
            }
            Opcode::JMPF => {
                self.pc += self.registers[self.next_8_bits() as usize] as usize;
            }
            Opcode::EQ => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = val1.eq(&val2);
                self.next_8_bits();
            }
            Opcode::NEQ => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = !val1.eq(&val2);
                self.next_8_bits();
            }
            Opcode::GT => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = val1 > val2;
                self.next_8_bits();
            }
            Opcode::LT => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = val1 < val2;
                self.next_8_bits();
            }
            Opcode::GTQ => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = val1 >= val2;
                self.next_8_bits();
            }
            Opcode::LTQ => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = val1 <= val2;
                self.next_8_bits();
            }
            Opcode::JEQ => {
                if self.equal_flag {
                    self.pc = self.registers[self.next_8_bits() as usize] as usize;
                }
            }
            Opcode::JNEQ => {
                if !self.equal_flag {
                    self.pc = self.registers[self.next_8_bits() as usize] as usize;
                }
            }
            Opcode::NOP => {
                self.next_8_bits();
                self.next_8_bits();
                self.next_8_bits();
            }
            Opcode::AlOC => {
                let register = self.registers[self.next_8_bits() as usize];
                let new_heap_size = self.heap.len() as i32 + register;
                self.heap.resize(new_heap_size as usize, 0);
            }
            Opcode::INC => {
                self.registers[self.next_8_bits() as usize] += 1;
                self.next_8_bits();
                self.next_8_bits();
            }
            Opcode::DEC => {
                self.registers[self.next_8_bits() as usize] -= 1;
                self.next_8_bits();
                self.next_8_bits();
            }
            Opcode::PTRS => {
                let start = self.next_16_bits() as usize;
                let mut end = start;
                let slice = self.ro_data.as_slice();

                while slice[end] != 0 {
                    end += 1
                }

                let result = std::str::from_utf8(&slice[start..end]);
                match result {
                    Ok(s) => {
                        print!("{}", s);
                    }
                    Err(e) => {
                        println!("Error decoding string for prts instruction: {:#?}", e)
                    }
                };
            }
            _ => {
                println!("This is not an opcode");
                return true;
            }
        }
        false
    }

    pub fn run(&mut self) {
        let mut done: bool = false;
        while !done {
            done = self.execute_once();
        }
    }

    fn execute_once(&mut self) -> bool {
        if self.pc >= (self.program.len() - 1) {
            return true;
        }
        let opcode = self.decode_opcode();
        self.match_opcode(opcode)
    }

    pub fn run_once(&mut self) {
        self.execute_once();
    }

    pub fn next_8_bits(&mut self) -> u8 {
        let result: u8 = self.program[self.pc];
        self.pc += 1;

        result
    }

    pub fn next_16_bits(&mut self) -> u16 {
        let result: u16 =
            ((self.program[self.pc] as u16) << 8) | (self.program[self.pc + 1] as u16);
        self.pc += 2;
        result
    }
    pub fn add_byte(&mut self, v: u8) {
        self.program.push(v)
    }

    pub fn add_bytes(&mut self, mut v: Vec<u8>) {
        self.program.append(&mut v)
    }

    fn verify_header(&self) -> bool {
        if self.program[0..4] != PIE_HEADER_PREFIX {
            return false;
        }
        true
    }
}

pub fn prepend_header(mut b: Vec<u8>) -> Vec<u8> {
    let mut header = vec![];
    for byte in PIE_HEADER_PREFIX {
        header.push(byte);
    }

    while header.len() < PIE_HEADER_LENGTH {
        header.push(0);
    }

    header.append(&mut b);
    header
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_new() {
        let x = Vm::new();
        assert_eq!(x.registers[0], 0);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut vm = Vm::new();
        vm.program = vec![0, 0, 0, 0];
        vm.program = prepend_header(vm.program);
        vm.run();
        println!("{}", vm.pc);
        assert_eq!(vm.pc, 68);
    }

    #[test]
    fn test_opcode_igl() {
        let mut vm = Vm::new();
        vm.program = vec![253, 1, 1, 1];
        vm.program = prepend_header(vm.program);
        vm.run();
        println!("{}", vm.pc);
        assert_eq!(vm.pc, 65);
    }

    #[test]
    fn test_load_opcode() {
        let mut vm = Vm::new();
        vm.program = vec![0, 0, 1, 244];
        vm.program = prepend_header(vm.program);
        vm.run();
        assert_eq!(vm.registers[0], 500);
    }

    #[test]
    fn test_add_opcode() {
        let mut vm = Vm::new();
        // Load 1 into register 0
        vm.add_bytes(vec![0, 0, 0, 1]);
        // Load 2 into register 1
        vm.add_bytes(vec![0, 1, 0, 2]);
        // Add register 0 and register 1, store result in register 2
        vm.add_bytes(vec![1, 0, 1, 2]);

        vm.program = prepend_header(vm.program);
        vm.run();

        vm.registers.iter().for_each(|x| println!("{}", x));
        assert_eq!(vm.registers[2], 3); // Corrected assertion: 1 + 2 = 3
    }

    #[test]
    fn test_jmp_opcode() {
        let mut vm = Vm::new();
        vm.registers[0] = 1;
        vm.program = vec![5, 0, 0, 0];
        vm.program = prepend_header(vm.program);
        vm.run_once();
        assert_eq!(vm.pc, 1);
    }
    #[test]
    fn test_jmpb_opcode() {
        let mut vm = Vm::new();
        vm.registers[0] = 1;
        vm.program = vec![7, 0, 0, 0];
        vm.program = prepend_header(vm.program);
        vm.run_once();
        assert_eq!(vm.pc, 65);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut vm = Vm::new();
        vm.registers[0] = 1;
        vm.program = vec![6, 0, 0, 0];
        vm.program = prepend_header(vm.program);
        vm.run();
        assert_eq!(vm.pc, 67);
    }

    #[test]
    fn test_eq() {
        let mut vm = Vm::new();
        vm.registers[0] = 1;
        vm.registers[1] = 1;
        vm.program = vec![8, 0, 1, 2];
        vm.program = prepend_header(vm.program);
        vm.run();
        assert!(vm.equal_flag);
        vm.registers[0] = 32;
        vm.pc = 64;
        vm.run();
        assert!(!vm.equal_flag);
    }

    #[test]
    fn test_jeq_and_jneq_opcode() {
        let mut vm = Vm::new();
        vm.registers[0] = 7;
        vm.registers[1] = 1;
        vm.equal_flag = true;
        vm.program = vec![14, 0, 0, 0, 15, 1, 0, 0];
        vm.program = prepend_header(vm.program);
        vm.run_once();
        assert_eq!(vm.pc, 7);
        vm.pc = 0;
        vm.equal_flag = false;
        vm.run_once();
        assert_eq!(vm.pc, 1);
    }

    #[test]
    fn test_nop_opcode() {
        let mut vm = Vm::new();
        vm.program = vec![16, 0, 0, 0, 1, 0, 0, 1];
        vm.program = prepend_header(vm.program);
        vm.run_once();
        assert_eq!(vm.pc, 68);
    }

    #[test]
    fn test_aloc_opcode() {
        let mut vm = Vm::new();
        vm.registers[0] = 100;
        vm.program = vec![17, 0, 0, 0];
        vm.program = prepend_header(vm.program);
        vm.run_once();

        assert_eq!(vm.heap.len(), 100);
    }

    #[test]
    fn test_inc_opcode() {
        let mut vm = Vm::new();
        vm.registers[0] = 2;
        vm.program = vec![18, 0, 0, 0];
        vm.program = prepend_header(vm.program);
        vm.run_once();

        assert_eq!(vm.registers[0], 3);
    }

    #[test]
    fn test_dec_opcode() {
        let mut vm = Vm::new();
        vm.registers[0] = 2;
        vm.program = vec![19, 0, 0, 0];
        vm.program = prepend_header(vm.program);
        vm.run_once();

        assert_eq!(vm.registers[0], 1);
    }
}
