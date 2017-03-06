#[macro_use]
extern crate lazy_static;

mod variables;
use variables::{ VARIABLEMAP, VariableState };

mod btree;
use btree::BTree;

mod syntax;

extern crate regex;

use std::io::BufRead; /* stdin().lock() */
use std::io::Write;   /* stdout().flush() */
use regex::Regex;

fn write_prompt() {

    print!("> ");
    std::io::stdout().flush().unwrap();
}

fn solve_query(vars: String) -> bool {

	'solve: loop {

		for i in 0..vars.len() {

			let var_name = &vars[i..i+1];
			let mut var = {
                let mut variables = VARIABLEMAP.lock().unwrap();

                variables.get_mut(var_name).unwrap().clone()
            };

            let mut state = VariableState::Undefined;

			for ref mut rule in &mut var.rules {

				if rule.state != VariableState::Undefined {

                    state = rule.state.clone();
                    continue ;
                }

                rule.state = rule.solve();
                if rule.state == VariableState::Undefined {

                    continue ;
                }

                if state == VariableState::Undefined {

                    state = rule.state.clone();
                }
                else if state != rule.state {

                    println!("Error {:?} != {:?}", state, rule.state);
                    return false;
                }
			}

            {
                let mut variables = VARIABLEMAP.lock().unwrap();

                let var = variables.get_mut(var_name).unwrap();

                if var.state != state {

                    var.state = state;
                    continue 'solve;
                }
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

                if !syntax::variables(&vars) {

                    println!("syntax error");
                }
                else {

                    for i in 0..vars.len() {
                        let mut variables = VARIABLEMAP.lock().unwrap();
                        let var_name = &vars[i..i+1];
                        let var = variables.get_mut(var_name).unwrap();

                        var.state = VariableState::True;
                    }
                }

            },
            "?" => {

                let vars = &rule[1..];

                if !syntax::variables(&vars) {

                    println!("syntax error");
                }
                else {

                    for i in 0..vars.len() {
                        let mut variables = VARIABLEMAP.lock().unwrap();
                        let var_name = &vars[i..i+1];
                        let var = variables.get_mut(var_name).unwrap();
                        
                        var.state = VariableState::Undefined;
                    }

                    if ! solve_query(vars.to_string()) {
                        println!("contradiction");
                    }
                    
                    for i in 0..vars.len() {
                        let mut variables = VARIABLEMAP.lock().unwrap();
                        let var_name = &vars[i..i+1];
                        let var = variables.get_mut(var_name).unwrap();

                        println!("{} is {:?}", var_name, var.state);
                    }
                }
            },
            _ => {

                if !syntax::rule(rule.as_str()) {

                    println!("syntax error");
                }
                else {

                    let (letters, rule) = syntax::lexer_parser(rule.as_str());

                    for i in 0..letters.len() {
						let mut variables = VARIABLEMAP.lock().unwrap();
                        let var_name = &letters[i..i+1];
                        let var = variables.get_mut(var_name).unwrap();

                        let mut var_rule = rule.clone();

						if i + 2 < letters.len() && &letters[i+2..i+2] == "!" {

                            if let Some(ref mut root) = var_rule.root {

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

