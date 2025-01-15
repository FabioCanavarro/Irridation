#[macro_use]
extern crate nom;
extern crate clap;
use clap::{App,Arg,SubCommand};
pub mod assembler;
pub mod instruction;
pub mod repl;
pub mod vm;

fn start_repl(){
    let mut user_repl = repl::Repl::new();
    user_repl.run();
}

fn main() {
    start_repl();
}
