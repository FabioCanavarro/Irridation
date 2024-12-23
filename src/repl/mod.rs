use crate::vm::Vm;
use std;
use std::io;
use std::io::Write;


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
    pub fn run(&self){
        println!("Welcome to Iridation, This is the REPL, HAVE FUN!!!");
        loop{
            let mut buffer = String::new();
            let stdin = io::stdin();
            print!(">> ");
            io::stdout().flush().expect("Could not flush");
            stdin.read_line(&mut buffer).expect("Can't read line");
            buffer = buffer.trim().to_string();
            self.command_buffer.push(buffer);
            match buffer{
                String::from(.quit") => {
                    println!("Farewell my good sir, may time let our path cross each other once more.");
                    std::process::exit(0);
                },
                _ => {
                    println!("Invalid Input");
                }
            }

        }
    }
}
