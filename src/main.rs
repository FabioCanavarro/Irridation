#[macro_use]
extern crate nom;
extern crate clap;
use std::{fs::File, io::Read, path::Path};

use clap::{App,Arg,SubCommand};
pub mod assembler;
pub mod instruction;
pub mod repl;
pub mod vm;

fn start_repl(){
    let mut user_repl = repl::Repl::new();
    user_repl.run();
}

fn read_file(tmp: &str) -> String{
    let filename = Path::new(tmp);
    let fl = File::open(filename);
    match fl {
        Ok(mut f) => {
            let mut file_content = String::new();
            match f.read_to_string(&mut file_content) {
                Ok(_) => file_content,
                Err(e) => {
                    println!("{:?}",e);
                    std::process::exit(1);
                }
            }
            
        },
        Err(_) => {
            println!("Failed to read file");
            std::process::exit(1);

        }
    }
}


fn main() {
    start_repl();
}
