#[macro_use]
extern crate lazy_static;

use std::env;

use std::error::Error;

use std::io::BufReader;
use std::fs::File;

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

fn solve_all(vars: String) -> bool {

    let mut success = true;
    
    's: loop {
        for i in 0..variables::VARS.len() {
            let name = &variables::VARS[i..i+1];
            let mut var = {
                let mut variables = VARIABLEMAP.lock().unwrap();
                variables.get_mut(name).unwrap().clone()
            };
            if var.state == VariableState::Undefined {
                let mut state = VariableState::Undefined;
                for ref mut rule in &mut var.rules {
                    rule.state = rule.solve();
                    if state == VariableState::Undefined {
                        state = rule.state.clone();
                    }
                    else if state != rule.state {
                        println!("Error {:?} != {:?}", state, rule.state);
                        state = VariableState::Undefined;
                        success = false
                    }
                }

                {
                    let mut variables = VARIABLEMAP.lock().unwrap();
                    let name = &variables::VARS[i..i+1];
                    let ref mut var = variables.get_mut(name).unwrap();
                    var.state = state;
                }

                if var.state != VariableState::Undefined {
                    continue 's;
                }
            }
        }
        break ;
    }

    if ! success {
        println!("Error");
    } else {
        for i in 0..vars.len() {
            let mut variables = VARIABLEMAP.lock().unwrap();
            let name = &vars[i..i+1];
            let var = variables.get_mut(name).unwrap();

            if var.state == VariableState::Undefined {
                println!("{} is false", name);
            }
            else {
                println!("{} is {:?}", name, var.state);
            }
        }
    }

    success
}

fn solve_query(vars: String) -> bool {

	for i in 0..vars.len() {

		let name = &vars[i..i+1];
		let mut var = {
            let mut variables = VARIABLEMAP.lock().unwrap();
            variables.get_mut(name).unwrap().clone()
        };
        if var.state != VariableState::Undefined {
            println!("{} is {:?}", name, var.state);
            continue ;
        }
        let state = var.solve();
        if state == VariableState::Undefined {
            println!("{} is false", name);
        } else {
            println!("{} is {:?}", name, state);
        }
	}
	return true;
}

fn handle_new_line(line: String) {

    match &line[..1] {

        "=" => {

            let vars = &line[1..];

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

            let vars = &line[1..];

            if ! syntax::variables(&vars) { println!("syntax error"); }
            else { solve_query(vars.to_string()); }
        },
        _ => {

            if !syntax::rule(line.as_str()) {

                println!("syntax error");
            }
            else {

                let (letters, rule) = syntax::lexer_parser(line.as_str());

                for i in 0..letters.len() {
                    let var_name = &letters[i..i+1];

                    if !var_name.chars().next().unwrap().is_uppercase() {
                        continue ;
                    }

					let mut variables = VARIABLEMAP.lock().unwrap();
                    let var = variables.get_mut(var_name).unwrap();

                    let mut var_rule = rule.clone();

					if i + 1 < letters.len() && &letters[i+1..i+2] == "!" {

                        if let Some(ref mut root) = var_rule.root {

                            root.n = true;
                        }
					}

                    var.rules.push_back(var_rule);
                }

            }
        }
    }

}

fn main() {

    let re = Regex::new("[[:space:]]").unwrap();
    let stdin = std::io::stdin();
    let args: Vec<String> = env::args().collect();

    if (args.len() != 2) {
        panic!("Invalid number of arguments");
    }

    let file = match File::open(&args[1]) {
        Err(why) => panic!("couldn't open {}: {}", args[1], why.description()),
        Ok(file) => file,
    };

    let file_content = BufReader::new(&file);

    for line in file_content.lines() {
        let s = line.unwrap();
        let rule = re.replace_all(&s, "").to_string();

        if rule.len() == 0 {
            write_prompt();
            continue ;
        }

        handle_new_line(rule);
    }

    write_prompt();
    for line in stdin.lock().lines() {

        let s = line.unwrap();
        let rule = re.replace_all(&s, "").to_string();

        if rule.len() == 0 {

            write_prompt();
            continue ;
        }

        handle_new_line(rule);

        write_prompt();
    }
}
