#[derive(PartialEq)]
enum Expect {
	Letter,
	Operator,
	Greater
}

pub fn rule(str: &str) -> bool {

	let mut p = 0;
	let mut expect = Expect::Letter;
	let mut allow_parenthesis = true;

	for c in str.chars() {
		match c {
			'=' => {
				if expect != Expect::Operator || allow_parenthesis == false || p != 0 {
					return false;
				}
				expect = Expect::Greater;
			},
			'>' => {
				if expect != Expect::Greater {
					return false;
				}
				expect = Expect::Letter;
				allow_parenthesis = false;
			},
			'(' => {
				if expect != Expect::Letter || allow_parenthesis == false {
					return false;
				}
				p += 1;
			},
			')' => {
				if expect != Expect::Operator || allow_parenthesis == false || p == 0 {
					return false;
				}
				p -= 1;
			},
			'!' => {
				if expect != Expect::Letter {
					return false;
				}
			},
			'+' | '|' | '^' => {
				if expect != Expect::Operator {
					return false;
				}
				expect = Expect::Letter;
			},
			_ => {
				if c.is_uppercase() {
					if expect != Expect::Letter {
						return false;
					}
					expect = Expect::Operator;
				}
				else {
					return false;
				}
			}
		};
	}
	if expect != Expect::Operator || allow_parenthesis == true {
		return false;
	}

	return true;
}

pub fn variables(vars: &str) -> bool {
    
    for c in vars.chars() {
        
        if !c.is_uppercase() {

            return false;
        }
    }

    true
}

pub fn lexer(s: &str) -> Option<&str> {

    if s.len() >= 2 {
        let ok = match &s[..2] {
            "=>" => true,
            _    => false
        };
        if ok {
            return Some(&s[..2]);
        }
    }

    let ok = match &s[..1] {
        "+" | "|" | "^" | "(" | ")" | "=" | "?" | "!" => true,
        _ => s.chars().next().unwrap().is_uppercase()
    };
    if ok {
        return Some(&s[..1]);
    }

    return None;
}

use BTree;

pub fn lexer_parser(rule: &str) -> (String, BTree) {

    let mut tree = BTree::new();

    let mut i = 0;
    while i < rule.len() {

        let token = match lexer(&rule[i..]) {

            Some(token) => token,
            None => {
                println!("Invalid token: '{}'", &rule[i..]);
                break ;
            }
        };

        tree.insert(token);

        i += token.len();
    }

    tree.close();

    let letters = tree.get_rhs().to_string().replace("+", "");

    (letters, tree)
}

