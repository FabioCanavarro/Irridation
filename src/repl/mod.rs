#![allow(dead_code)]

use crate::assembler::program_parser::program;
use crate::vm::Vm;
use nom::types::CompleteStr;
use std::fs::File;
use std::io::Write;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::path::Path;

pub struct Repl {
    vm: Vm,
    command_buffer: Vec<String>,
}

impl Default for Repl {
    fn default() -> Self {
        Repl::new()
    }
}

impl Repl {
    pub fn new() -> Repl {
        Repl {
            vm: Vm::new(),
            command_buffer: vec![],
        }
    }
    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(" ").collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_string in split {
            let byte = u8::from_str_radix(hex_string, 16);
            match byte {
                Ok(result) => {
                    results.push(result);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
    }
    pub fn run(&mut self) {
        println!("Welcome to Iridation, This is the REPL, HAVE FUN!!!");
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();
            print!(">> ");
            io::stdout().flush().expect("Could not flush");
            stdin.read_line(&mut buffer).expect("Can't read line");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());
            match buffer {
                ".quit" => {
                    println!(
                        "Farewell my good sir, may time let our path cross each other once more."
                    );
                    std::process::exit(0);
                }
                ".history" => {
                    self.command_buffer.iter().for_each(|x| println!("{}", x));
                }
                ".program" => {
                    println!("Intructions currently in VM program vector");
                    self.vm.program.iter().for_each(|x| println!("{x}"));
                    println!("End of program listing")
                }
                ".registers" => {
                    println!("Registers currently in the VM");
                    println!("{:#?}", self.vm.registers);
                    println!("End of registers listing");
                }
                ".clear" => {
                    self.vm.program = vec![];
                    println!("VM program vector is cleared!!!");
                }
                ".load_file" => {
                    print!("Please enter the path to the file: ");
                    io::stdout().flush().expect("Unable to flush");
                    let mut tmp = String::new();
                    stdin.read_line(&mut tmp).expect("Unable to read the line");
                    let tmp = tmp.trim();
                    let filename = Path::new(&tmp);
                    let mut f;
                    match File::open(Path::new(&filename)){
                        Ok(file) => f = file,
                        Err(_e) => {println!("This is not a path to a .iasm file"); continue;}
                    }
                    let mut contents = String::new();
                    match f.read_to_string(&mut contents){
                        Err(_) => println!("Error while reading the file"),
                        Ok(b) => println!("{} byte was appended",b)
                    }
                    let program = match program(CompleteStr(&contents)) {
                        Ok((_, program)) => program,
                        Err(e) => {
                            println!("Unable to parse input {:?}", e);
                            continue;
                        }
                    };
                    self.vm.program.append(&mut program.to_bytes());
                }
                _ => {
                    let parsed_program = program(CompleteStr(buffer));
                    if parsed_program.is_err() {
                        println!("Unable to parse input");
                        continue;
                    }
                    let (_, result) = parsed_program.unwrap();
                    let bytecode = result.to_bytes();
                    for byte in bytecode {
                        self.vm.add_byte(byte);
                    }
                    self.vm.run_once();
                }
            }
        }
    }
}
