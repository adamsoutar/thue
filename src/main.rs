use std::env;
use std::fs;
mod rules;
mod parser;
mod interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please pass a path to a file containing Thue code as a cmd-line arg.")
    }

    let code = fs::read_to_string(&args[1])
        .expect("Couldn't read the source file");

    let prog = parser::parse(code);
    interpreter::run_program(prog);
}
