#[macro_use]
extern crate lazy_static;

use std::env;

use std::error::Error;

use std::io::BufReader;
use std::fs::File;

mod variables;
use variables::{ VARIABLEMAP, VariableState };

mod btree;

mod my_option;
use my_option::MyOption;

mod syntax;

extern crate regex;

use std::io::BufRead; /* stdin().lock() */
use std::io::Write;   /* stdout().flush() */
use regex::Regex;

fn write_prompt() {

    print!("> ");
    std::io::stdout().flush().unwrap();
}

#[allow(dead_code)]
fn solve_all(vars: String) -> bool {

    let success = true;
    
    's: loop {
        for i in 0..variables::VARS.len() {
            let name = &variables::VARS[i..i+1];
            let var = {
                let mut variables = VARIABLEMAP.lock().unwrap();
                variables.get_mut(name).unwrap().clone()
            };
            if var.state == VariableState::Undefined {

                match var.solve("".to_string()) {
                    MyOption::Some(state) => {
                        let mut variables = VARIABLEMAP.lock().unwrap();
                        let name = &variables::VARS[i..i+1];
                        let ref mut var = variables.get_mut(name).unwrap();
                        var.state = state;
                    },
                    MyOption::Error(s) => {
                        println!("Error: {}", s);
                        return false;
                    }
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
		let var = {
            let mut variables = VARIABLEMAP.lock().unwrap();
            variables.get_mut(name).unwrap().clone()
        };
        if var.state != VariableState::Undefined {
            println!("{} is {:?}", name, var.state);
            continue ;
        }
        match var.solve("".to_string()) {
            my_option::MyOption::Some(state) => match state {
                VariableState::Undefined => println!("{} is false", name),
                _ => println!("{} is {:?}", name, state)
            },
            my_option::MyOption::Error(s) => println!("Error: {}", s)
        }
	}

	return true;
}

fn handle_new_line(line: String) {

    match &line[..1] {

        "=" => {

            let vars = &line[1..];

            if !syntax::variables(&vars) {

                println!("syntax error: {}", line);
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

            if ! syntax::variables(&vars) { println!("syntax error: {}", line); }
            else { solve_query(vars.to_string()); }
        },
        "@" => {
            variables::reset();
        },
        _ => {

            if !syntax::rule(line.as_str()) {

                println!("syntax error: {}", line);
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

    if args.len() > 2 {
        println!("Invalid number of arguments");
        return;
    }

    if args.len() == 2 {
        let file = match File::open(&args[1]) {
            Err(why) => {
                println!("couldn't open {}: {}", args[1], why.description());
                return;
            },
            Ok(file) => file,
        };

        let file_content = BufReader::new(&file);

        for line in file_content.lines() {
            let s = line.unwrap();

            if s.chars().next().unwrap() == '#' {
                println!("{}", s);
                continue ;
            }

            let rule = re.replace_all(&s, "").to_string();

            if rule.len() == 0 {
                write_prompt();
                continue ;
            }

            handle_new_line(rule);
        }
    }
    else {

    write_prompt();
    for line in stdin.lock().lines() {

        let s = line.unwrap();
        let rule = re.replace_all(&s, "").to_string();

        if rule.len() == 0 {

            write_prompt();
            continue ;
        }

        if rule == "exit" {
            return;
        }

        handle_new_line(rule);

        write_prompt();
    }
    }
}
