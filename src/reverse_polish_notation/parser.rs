enum Expression {
	Operator(char),
	Val(i32),
	Nil
}

pub fn parse(input: String) -> String {							// parses into reverse polish notation
	let tokens = tokenize(&input);
	
	input
}

fn tokenize(input: &String) -> Vec<Expression> {
	let mut tokens = Vec::new();
	
	let mut cs = Vec::new();

	for e in input.chars() {
		cs.push(e);
	}

	let mut i = 0;

	while i < cs.len() {
		let x = cs[i];
		if is_digit(x) {										// handles numeric input
		
			let mut val = x as i32 - '0' as i32; 				// begins val count
			while i < cs.len() - 1 && is_digit(cs[i + 1]) { 	// checks for trailing digits
				i = i + 1;
				val = val * 10 + (cs[i] as i32 - '0' as i32);
			}
			tokens.push(Expression::Val(val));
			
		} else if is_operator(x) {								// handles symbolic operators
		
			tokens.push(Expression::Operator(x));
		
		} else { 												// handles other operators containing letters
		
			tokens.push(Expression::Nil);
		
		}
		i = i + 1;
	}
	
	tokens
}

fn is_digit(c: char) -> bool {
	c >= '0' && c <= '9'
}

fn is_operator(c: char) -> bool {
	c == '+' ||
	c == '-' ||
	c == '*' ||
	c == '/' ||
	c == '(' ||
	c == ')' ||
	c == '^'
}
