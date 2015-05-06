enum Token {
	Operator(char),
	Val(i32),
	Nil
}

pub fn parse(input: String) -> String {
	let tokens = tokenize(&input);
	
	for token in tokens {
		/*match token {
			Operator(c)	=> print!("{}", c),
			Val(v)		=> print!("{}", v),
			Nil			=> {}
		}*/
	}
	println!("");
	println!("");
	input
}

fn tokenize(input: &String) -> Vec<Token> {
	let mut tokens = Vec::new();
	let mut val = 0;
	
	for c in input.chars() {
		if c >= '0' && c <= '9' {
			val = val * 10 + c as i32 - '0' as i32;
		} else {
			tokens.push(Token::Val(val));
			tokens.push(Token::Operator(c));
		}
	}
	
	tokens
}
