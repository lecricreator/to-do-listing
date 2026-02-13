mod action;
mod arg;
pub mod errors;
pub mod gestionary_file;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        println!("To-do-rustline need minimum 1 argument for execute the program.");
        return;
    }
    arg::start_program(&args[1..]);
}
