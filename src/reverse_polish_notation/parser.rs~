pub enum Expression {
	Operator(String),
	Val(i32),
	Nil
}

pub fn parse(input: String) -> Vec<Expression> {
	let expressions = tokenize(&input);

	shunting_yard_parser(expressions)	
}

fn shunting_yard_parser(input: Vec<Expression>) -> Vec<Expression> {
	// parses into reverse polish notation using the shunting yard algorithm

	let mut output: Vec<Expression> = Vec::new();
	let mut operators: Vec<Expression> = Vec::new();
	
	for ex in input {
		match ex {
			Expression::Operator(o)	=> {},
			Expression::Val(v)		=> output.push(ex),
			Expression::Nil			=> {}
		}
	}
	
	output
}

fn tokenize(input: &String) -> Vec<Expression> {
	// breaks the string into tokens (either a value or a operator)
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
		
			tokens.push(Expression::Operator(x.to_string()));
		
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

fn get_precedence(operator: String) -> i8 {
	1
}
