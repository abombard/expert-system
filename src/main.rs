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
        "+" | "|" | "^" | "(" | ")" | "=" | "?" => true,
        _ => s.chars().next().unwrap().is_uppercase()
    };
    if ok {
        return Some(&s[..1]);
    }

    return None;
}



/*
** parser
*/
fn parse(token: &str) {
    
}

// create a rule from user's input
fn new_rule(rule_str: String) {

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

        parse(&token);

        i += token.len();
    }

}

extern crate regex;

use std::io::BufRead; /* stdin().lock() */
use std::io::Write;   /* stdout().flush() */
use regex::Regex;

fn main() {
    let re = Regex::new("[[:space:]]").unwrap();
    let stdin = std::io::stdin();

    // read lines
    print!("> ");
    std::io::stdout().flush().unwrap();
    for line in stdin.lock().lines() {
        let s = line.unwrap();
        let rule = re.replace_all(&s, "");

        new_rule(rule.to_string());

        print!("> ");
        std::io::stdout().flush().unwrap();
    }
}

