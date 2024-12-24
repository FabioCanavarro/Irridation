pub mod vm;
pub mod instruction;
pub mod repl;
fn main() {
    println!("Hello, world!");
    let mut user_repl = repl::Repl::new();
    user_repl.run(); 
}
