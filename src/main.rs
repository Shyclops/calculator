extern crate calculator;

fn main() {
	println!("{}", calculator::reverse_polish_notation::parser::parse(format!("32-50*06+74/24nil")));
}
