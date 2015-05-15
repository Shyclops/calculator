extern crate calculator;

fn main() {
	for ex in calculator::reverse_polish_notation::parser::parse(format!("32-50*06+74/24nil")) {
		match ex {
			calculator::reverse_polish_notation::parser::Expression::Operator(o)	=> println!("{}", o),
			calculator::reverse_polish_notation::parser::Expression::Val(v)	 		=> println!("{}", v),
			calculator::reverse_polish_notation::parser::Expression::Nil			=> println!("nil")
		}
	}
}
