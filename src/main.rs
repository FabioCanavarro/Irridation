#[macro_use]
extern crate nom;

#[macro_use]
extern crate clap;

use assembler::base_assembler::Assembler;
use clap::{App, Arg, SubCommand};
use std::{fs::File, io::Read, path::Path};
use vm::Vm;

pub mod assembler;
pub mod instruction;
pub mod repl;
pub mod vm;

fn start_repl() {
    let mut user_repl = repl::Repl::new();
    user_repl.run();
}

fn read_file(tmp: &str) -> String {
    let filename = Path::new(tmp);
    let fl = File::open(filename);
    match fl {
        Ok(mut f) => {
            let mut file_content = String::new();
            match f.read_to_string(&mut file_content) {
                Ok(_) => file_content,
                Err(e) => {
                    println!("{:?}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(_) => {
            println!("Failed to read file");
            std::process::exit(1);
        }
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let target_file = matches.value_of("INPUT_FILE");
    match target_file {
        Some(fl) => {
            let program = read_file(fl);
            let mut vm = Vm::new();
            let mut asm = Assembler::new();
            let program = asm.assemble(&program);

            if let Some(p) = program {
                vm.add_bytes(p);
                vm.run();
                std::process::exit(0);
            }
        }
        None => start_repl(),
    }
}
