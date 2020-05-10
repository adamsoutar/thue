mod rules;
mod parser;
mod interpreter;

fn main() {
    let prog = parser::parse(String::from("
.::=~Hello, world!
::=
.
"));
    interpreter::run_program(prog);
}
