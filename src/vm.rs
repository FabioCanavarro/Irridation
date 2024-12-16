use std::usize;

use crate::instruction::{self, Opcode};

// Emulate cpu
#[derive(Debug,PartialEq)]
pub struct Vm {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32
}

impl Vm{
    pub fn new() -> Vm{
        Vm{
            registers:[0; 32],
            pc: 0,
            program: vec![],
            remainder: 0

        }
    }
    fn decode_opcode(&mut self) -> Opcode{
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc+=1;
        opcode
    }
    pub fn run(&mut self){
        loop{
            if self.pc >= self.program.len(){
                break;
            }

            match self.decode_opcode(){
                Opcode::HLT => {
                    println!("Opcode HLT");
                    return;
                },
                Opcode::LOAD =>{
                    let register: usize = self.next_8_bits() as usize;
                    let number: u16 = self.next_16_bits();
                    self.registers[register] = number as i32;
                    continue
                },
                Opcode::ADD => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];
                    self.registers[self.next_8_bits() as usize] = register1 + register2;
                },
                Opcode::MUL => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];
                    self.registers[self.next_8_bits() as usize] = register1 * register2;
                },
                Opcode::SUB => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];
                    self.registers[self.next_8_bits() as usize] = register1 - register2;
                },
                Opcode::DIV => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];
                    self.registers[self.next_8_bits() as usize] = register1 / register2;
                    self.remainder = (register1 % register2) as u32;
                },
                _ => {
                    println!("This is not an opcode");
                    return;
                }

            }
            
        }
    }
    fn execute_once(&mut self) -> bool{
        if self.pc >= self.program.len(){
            return false;
        }

        match self.decode_opcode(){
            Opcode::HLT => {
                println!("Opcode HLT");
            },
            Opcode::LOAD =>{
                let register: usize = self.next_8_bits() as usize;
                let number: u16 = self.next_16_bits();
                self.registers[register] = number as i32;
            },
            Opcode::ADD => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];
                    self.registers[self.next_8_bits() as usize] = register1 + register2;
            },
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            },
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            },
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            },
            _ => {
                println!("This is not an opcode");
            }
        
        }
        true
    }


    pub fn run_once(&mut self){
        self.execute_once();
    }


    pub fn next_8_bits(&mut self) -> u8{
        let result: u8 = self.program[self.pc];
        self.pc +=1;
        result
    }

    pub fn next_16_bits(&mut self) -> u16{
        let result:u16 = ((self.program[self.pc] as u16) << 8) | (self.program[self.pc] as u16);
        self.pc +=2;
        result
    }


}

# [cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn check_new(){
        let x = Vm::new();
        assert_eq!(x.registers[0],0);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut vm = Vm::new();
        vm.program = vec![0,0,0,0];
        vm.run();
        assert_eq!(vm.pc,1);
    }
    
    #[test]
    fn test_opcode_igl() {
        let mut vm = Vm::new();
        vm.program = vec![1,1,1,1];
        vm.run();
        assert_eq!(vm.pc,1);


        
    }

}

