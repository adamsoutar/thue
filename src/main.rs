mod rules;
mod parser;

fn main() {
    parser::parse(String::from("
.::=~Hello, world!
::=
.
"));
}
