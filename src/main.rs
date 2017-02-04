use std::string::String;

/*
** tree
*/


/*
** lexer
*/
#[derive(Debug)]
enum TokenId {
    Undefined,
    Var,
    And,
    Or,
    Xor,
    Then,
    ParenthesisOpen,
    ParenthesisClose,
    Equal,
    InterrogationPoint
}

// simple check on 2 near token
fn state_is_valid(state: &TokenId, newState: &TokenId) -> bool {

    match (state, newState) {
        (&TokenId::Undefined,          &TokenId::Var               ) => return true,
        (&TokenId::Undefined,          &TokenId::ParenthesisOpen   ) => return true,
        (&TokenId::Undefined,          &TokenId::Equal             ) => return true,
        (&TokenId::Undefined,          &TokenId::InterrogationPoint) => return true,
        (&TokenId::Equal,              &TokenId::Var               ) => return true,
        (&TokenId::InterrogationPoint, &TokenId::Var               ) => return true,
        (&TokenId::Var,                &TokenId::And               ) => return true,
        (&TokenId::Var,                &TokenId::Or                ) => return true,
        (&TokenId::Var,                &TokenId::Xor               ) => return true,
        (&TokenId::Var,                &TokenId::Then              ) => return true,
        (&TokenId::Var,                &TokenId::ParenthesisClose  ) => return true,
        (&TokenId::Then,               &TokenId::Var               ) => return true,
        (&TokenId::And,                &TokenId::Var               ) => return true,
        (&TokenId::Or,                 &TokenId::Var               ) => return true,
        (&TokenId::Xor,                &TokenId::Var               ) => return true,
        (&TokenId::ParenthesisOpen,    &TokenId::Var               ) => return true,
        (&TokenId::ParenthesisClose,   &TokenId::And               ) => return true,
        (&TokenId::ParenthesisClose,   &TokenId::Or                ) => return true,
        (&TokenId::ParenthesisClose,   &TokenId::Xor               ) => return true,
        (&TokenId::ParenthesisClose,   &TokenId::Then              ) => return true,
        _ => return false
    }
 
}

struct Token {
    id: TokenId,
    s: String
}

// identify the new state and the token
fn lexer(s: &str) -> Option<Token> {

    if s.len() >= 2 {
        let option = match &s[..2] {
            "=>" => Some(TokenId::Then),
            _    => None
        };
        if let Some(tokenId) = option {
            return Some(Token { id: tokenId, s: s[..2].to_string() });
        }
    }

    let option = match &s[..1] {
        "+"  => Some(TokenId::And),
        "|"  => Some(TokenId::Or),
        "^"  => Some(TokenId::Xor),
        "("  => Some(TokenId::ParenthesisOpen),
        ")"  => Some(TokenId::ParenthesisClose),
        "="  => Some(TokenId::Equal),
        "?"  => Some(TokenId::InterrogationPoint),
        _ => match s.chars().next().unwrap().is_uppercase() {
                true  => Some(TokenId::Var),
                false => None
            }
    };
    if let Some(tokenId) = option {
        return Some(Token { id: tokenId, s: s[..1].to_string() });
    }

    return None;
}

/*
** parser
*/
fn parse(token: &Token) {
    
}

// create a rule from user's input
fn new_rule(rule_str: String) {

    let mut state = TokenId::Undefined;

    let mut i = 0;
    while i < rule_str.len() {
        let token = match lexer(&rule_str[i..]) {
            Some(token) => token,
            None => {
                println!("Invalid token: '{}'", &rule_str[i..]);
                break ;
            }
        };

        println!("New token {}, id {:?}", token.s, token.id);

        if !state_is_valid(&state, &token.id) {
            println!("Unexpected token: '{}'", token.s);
        }

        parse(&token);

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
