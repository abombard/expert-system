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

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::sync::Mutex;

mod variables;
use variables::{ VariableMap, VariableState, Variable };

mod btree;
use btree::BTree;

// create a rule from user's input
fn new_rule(rule: &str) -> (String, BTree) {

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

    let letters = tree.extract_rhs().replace("+", "");

    (letters, tree)
}

extern crate regex;

use std::io::BufRead; /* stdin().lock() */
use std::io::Write;   /* stdout().flush() */
use regex::Regex;
use std::collections::LinkedList;

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

fn write_prompt() {

    print!("> ");
    std::io::stdout().flush().unwrap();
}

fn solve_query(vars: String) -> bool {
	let mut variables = VariableMap.lock().unwrap();
	'toto: loop {
		for i in 0..vars.len() {
			let var_name = &vars[i..i+1];
			let var = variables.get_mut(var_name).unwrap();
			let mut reboot = false;

			println!("ping");
			for ref mut rule in &mut var.rules {
println!("pong");
				let mut prev_state = rule.state.clone();
				println!("{:?}", prev_state);
				if rule.state == VariableState::Undefined {
					rule.state = rule.solve();
					if rule.state != VariableState::Undefined {
						if rule.state != var.state && var.state != VariableState::Undefined {
							return false;
						}
						var.state = rule.state.clone();
						if prev_state != rule.state {
							reboot = true;
						}
					}
				}
			}
			if reboot == true {
					continue 'toto;
			}
		}
		break;
	}
	return true;
}

fn main() {

    let re = Regex::new("[[:space:]]").unwrap();
    let stdin = std::io::stdin();

    write_prompt();
    for line in stdin.lock().lines() {

        let s = line.unwrap();
        let rule = re.replace_all(&s, "").to_string();

        match &rule[..1] {

            "=" => {

                let vars = &rule[1..];

                for i in 0..vars.len() {
					let mut variables = VariableMap.lock().unwrap();
                    let var_name = &vars[i..i+1];
                    let var = variables.get_mut(var_name).unwrap();

                    var.state = VariableState::True;
                }

            },
            "?" => {

                let vars = &rule[1..];

                for i in 0..vars.len() {
					let mut variables = VariableMap.lock().unwrap();
                    let var_name = &vars[i..i+1];
                    let var = variables.get_mut(var_name).unwrap();
					
					var.state = VariableState::Undefined;
                }

				if ! solve_query(vars.to_string()) {
					println!("contradiction");
				}
				
				for i in 0..vars.len() {
					let mut variables = VariableMap.lock().unwrap();
					let var_name = &vars[i..i+1];
					let var = variables.get_mut(var_name).unwrap();

					println!("{} is {:?}", var_name, var.state);
				}
            },
            _ => {

                if !check_syntax(rule.as_str()) {

                    println!("syntax error");
                }
                else {

                    let (letters, rule) = new_rule(rule.as_str());

                    for i in 0..letters.len() {
						let mut variables = VariableMap.lock().unwrap();
                        let var_name = &letters[i..i+1];
                        let var = variables.get_mut(var_name).unwrap();
						rule.display();
let mut var_rule = rule.clone();
						var_rule.display();
						if &letters[i+1..i+2] == "!" {
								let mut root_ref = var_rule.root_list.front_mut().unwrap();
								if let Some(ref mut root) = root_ref.root {
									root.n = true;
								}
						}

                        var.rules.push_back(var_rule);
                    }

                }
            }
        }

        write_prompt();
    }
}

