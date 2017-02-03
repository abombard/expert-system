use std::string::String;

#[derive(Debug)]
enum TokenId {
    Undefined,
    Var,
    And,
    Or,
    Xor,
    Then,
    ParenthesisOpen,
    ParenthesisClose
}

// Need to borrow the type because of ownership
fn state_is_valid(state: &TokenId, newState: &TokenId) -> bool {

    match (state, newState) {
        (&TokenId::Undefined,        &TokenId::Var             ) => return true,
        (&TokenId::Undefined,        &TokenId::ParenthesisOpen ) => return true,
        (&TokenId::Var,              &TokenId::And             ) => return true,
        (&TokenId::Var,              &TokenId::Or              ) => return true,
        (&TokenId::Var,              &TokenId::Xor             ) => return true,
        (&TokenId::Var,              &TokenId::Then            ) => return true,
        (&TokenId::Var,              &TokenId::ParenthesisClose) => return true,
        (&TokenId::Then,             &TokenId::Var             ) => return true,
        (&TokenId::And,              &TokenId::Var             ) => return true,
        (&TokenId::Or,               &TokenId::Var             ) => return true,
        (&TokenId::Xor,              &TokenId::Var             ) => return true,
        (&TokenId::ParenthesisOpen,  &TokenId::Var             ) => return true,
        (&TokenId::ParenthesisClose, &TokenId::And             ) => return true,
        (&TokenId::ParenthesisClose, &TokenId::Or              ) => return true,
        (&TokenId::ParenthesisClose, &TokenId::Xor             ) => return true,
        (&TokenId::ParenthesisClose, &TokenId::Then            ) => return true,
        _ => return false
    }
 
}

struct Token {
    id: TokenId,
    s: String
}

// identify the new state and the token
fn lexer(s: &str) -> Option<Token> {

    if s.len() == 0 {
        return None
    }

    if s.len() >= 2 {
        let option = match &s[..2] {
            "=>" => Some(Token { id: TokenId::Then,             s: "=>".to_string() }),
            _    => None
        };
        if let Some(token) = option {
            return Some(token);
        }
    }

    let option = match &s[..1] {
        "+"  => Some(Token { id: TokenId::And,              s: "+".to_string()  }),
        "|"  => Some(Token { id: TokenId::Or,               s: "|".to_string()  }),
        "^"  => Some(Token { id: TokenId::Xor,              s: "^".to_string()  }),
        "("  => Some(Token { id: TokenId::ParenthesisOpen,  s: "(".to_string()  }),
        ")"  => Some(Token { id: TokenId::ParenthesisClose, s: ")".to_string()  }),
        _ => if s.chars().next().unwrap().is_uppercase() {
                Some(Token { id: TokenId::Var, s: s.chars().next().unwrap().to_string() })
            } else {
                None
            }
    };
    if let Some(token) = option {
        return Some(token);
    }

    return Some(Token { id: TokenId::Undefined, s: s.chars().next().unwrap().to_string() });
}

// create a rule from user's input
fn new_rule(rule_str: String) {

    let mut state = TokenId::Undefined;

    let mut i = 0;
    while let Some(token) = lexer(&rule_str[i..]) {

        println!("New token: {} id {:?}", token.s, token.id);

        if !state_is_valid(&state, &token.id) {
            println!("Unexpected token: '{}'", token.s);
        }

        // parse()

        state = token.id;
        i += token.s.len();
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
