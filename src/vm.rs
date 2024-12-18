use std::usize;

use crate::instruction::Opcode;

// Emulate cpu
#[derive(Debug,PartialEq)]
pub struct Vm {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
    equal_flag: bool
}
impl Default for Vm{
    fn default() -> Self {
        Self::new()
    }
}
impl Vm{
    pub fn new() -> Vm{
        Vm{
            registers:[0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
            equal_flag: false

        }
    }

    fn decode_opcode(&mut self) -> Opcode{
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc+=1;
        opcode
    }

    fn match_opcode(&mut self,opcode: Opcode){
        match opcode{
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
            Opcode::JMP => {
                    let target = self.registers[self.next_8_bits() as usize];
                    self.pc = target as usize;
            },
            Opcode::JMPB => {
                self.pc -= self.registers[self.next_8_bits() as usize] as usize;
            },
            Opcode::JMPF => {
                self.pc += self.registers[self.next_8_bits() as usize] as usize;
            },
            Opcode::EQ => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = val1.eq(&val2);
                self.next_8_bits();
            },
            Opcode::NEQ => {
            },
            Opcode::GT => {
            },
            Opcode::LT =>{
            },
            Opcode::GTQ => {
            },
            Opcode::LTQ =>{
            },
            Opcode::JEQ => {
            },
            _ => {
                println!("This is not an opcode");
            }       
        }
    }

    pub fn run(&mut self){
        let mut done: bool = false;
        while !done{
            done = self.execute_once();
        }
    }

    fn execute_once(&mut self) -> bool{
        if self.pc >= self.program.len(){
            return false;
        }
        let opcode = self.decode_opcode();
        self.match_opcode(opcode);
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
        let result:u16 = ((self.program[self.pc] as u16) << 8) | (self.program[self.pc + 1] as u16);
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
        println!("{}",vm.pc);
        assert_eq!(vm.pc,1);
    }
    
    #[test]
    fn test_opcode_igl() {
        let mut vm = Vm::new();
        vm.program = vec![20,1,1,1];
        vm.run();
        println!("{}",vm.pc);
        assert_eq!(vm.pc,1);
    }

    #[test]
    fn test_load_opcode(){
        let mut vm = Vm::new();
        vm.program = vec![1,0,1,244];
        vm.run();
        assert_eq!(vm.registers[0],500);
    }

    #[test]
    fn test_jmp_opcode(){
        let mut vm = Vm::new();
        vm.registers[0] = 1;
        vm.program = vec![6,0,0,0];
        vm.run_once();
        assert_eq!(vm.pc,1);
    }
    #[test]
    fn test_jmpfb_opcode(){
        let mut vm = Vm::new();
        vm.registers[0] = 1;
        vm.program = vec![8,0,0,0];
        vm.run();
        assert_eq!(vm.pc,1);
        vm.program = vec![7,0,0,0];
        vm.run();
        assert_eq!(vm.pc,2);
    }
    #[test]
    fn test_eq(){
        let mut vm = Vm::new();
        vm.registers[0] =1;
        vm.registers[1] =1;
        vm.program= vec![9,0,1,2];
        vm.run();
        assert(vm.equal_flag);
    }

}

