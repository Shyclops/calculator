extern crate calculator;

fn main() {
	println!("{}", calculator::reverse_polish_notation::parser::parse(format!("3-5*6+7/4")));
}