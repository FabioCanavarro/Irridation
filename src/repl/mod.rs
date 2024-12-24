use crate::vm::Vm; 
use std::{self, result};
use std::io;
use std::io::Write;
use std::num::ParseIntError;


pub struct Repl{
    vm: Vm,
    command_buffer: Vec<String>,
}

impl Default for Repl{
    fn default() -> Self {
        Repl::new()
    }
}

impl Repl{
    pub fn new() -> Repl{
        Repl{
            vm: Vm::new(),
            command_buffer: vec![]
        }
    }
fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError>{
    let split = i.split(" ").collect::<Vec<&str>>();
    let mut results: Vec<u8> = vec![];
    for hex_string in split {
        let byte = u8::from_str_radix(hex_string, 16);
        match byte {
            Ok(result) => {
                results.push(result);
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(results)
}
    pub fn run(&mut self){
        println!("Welcome to Iridation, This is the REPL, HAVE FUN!!!");
        loop{
            let mut buffer = String::new();
            let stdin = io::stdin();
            print!(">> ");
            io::stdout().flush().expect("Could not flush");
            stdin.read_line(&mut buffer).expect("Can't read line");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());
            match buffer{ 
                ".quit" => {
                    println!("Farewell my good sir, may time let our path cross each other once more."); 
                    std::process::exit(0);
                },
                ".history" => {
                    self.command_buffer.iter().for_each(|x| println!("{}",x));
                },
                ".program" =>{
                    println!("Intructions currently in VM program vector");
                    self.vm.program.iter().for_each(|x| println!("{x}"));
                    println!("End of program listing")
                },
                ".registers" => {
                    println!("Registers currently in the VM");
                    println!("{:#?}",self.vm.registers);
                    println!("End of registers listing");
                },
                _ => {
                    let result = self.parse_hex(buffer);
                    match result{
                        Ok(byte) => {
                            byte.into_iter().for_each(|x| self.vm.add_byte(x))
                        },
                        Err(_) => println!("Unable to decode hex string, please enter 4 group of 2 hex char")
                    }
                    self.vm.run_once();
                } 
            } 
        } 
    } 


}

