/*
** lexer
*/
fn lexer(s: &str) -> Option<&str> {

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

mod btree;

// create a rule from user's input
fn new_rule(rule_str: String) {

    let mut tree = btree::BTree::new();

    let mut i = 0;
    while i < rule_str.len() {
        let token = match lexer(&rule_str[i..]) {
            Some(token) => token,
            None => {
                println!("Invalid token: '{}'", &rule_str[i..]);
                break ;
            }
        };

        println!("New token {}", token);

        tree.insert(token);

        i += token.len();
    }

    tree.display();
}

extern crate regex;

use std::io::BufRead; /* stdin().lock() */
use std::io::Write;   /* stdout().flush() */
use regex::Regex;

#[derive(PartialEq)]
enum Expect {
	Letter,
	Operator,
	Greater
}

fn check_syntax(str: &str) -> bool {
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

fn main() {
    let re = Regex::new("[[:space:]]").unwrap();
    let stdin = std::io::stdin();

    // read lines
    print!("> ");
    std::io::stdout().flush().unwrap();
    for line in stdin.lock().lines() {
        let s = line.unwrap();
        let rule = re.replace_all(&s, "").to_string();

		if ! check_syntax(rule.as_str()) {
			println!("syntax_error");
			continue;
		}

        new_rule(rule);

        print!("> ");
        std::io::stdout().flush().unwrap();
    }
}

