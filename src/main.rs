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

mod variables;
use variables::{ VariableMap, VariableState };

mod btree;
use btree::BTree;

// create a rule from user's input
fn new_rule(rule: &str) -> BTree {

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

        println!("New token {}", token);

        tree.insert(token);

        i += token.len();
    }

    tree.display();

    let result = tree.solve();
    println!("Result: {:?}", result);

    tree
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

fn main() {

    let mut rules = LinkedList::new();

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

                    if let Some(var_ref) = variables.get_mut(var_name) {

                        *var_ref = VariableState::True;
                    }
                    else {

                        println!("Invalid Variable: {}", var_name);
                    }
                }

            },
            "?" => {

            },
            _ => {

                if !check_syntax(rule.as_str()) {

                    println!("syntax error");
                }
                else {

                    let tree = new_rule(rule.as_str());

                    rules.push_back(tree);
                }
            }
        }

        write_prompt();
    }
}

