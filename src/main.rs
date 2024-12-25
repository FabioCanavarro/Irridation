#[macro_use]
extern crate nom;
pub mod vm;
pub mod instruction;
pub mod repl;
pub mod assembler;
fn main() {
    println!("Hello, world!");
    let mut user_repl = repl::Repl::new();
    user_repl.run(); 
}
