use std::string::String;

#[derive(Debug)]
enum State {
    Undefined,
    Var,
    And,
    Or,
    Xor,
    ThenBegin,
    ThenEnd,
    ParenthesisOpen,
    ParenthesisClose,
}

fn state_is_valid(state: &State, newState: &State) -> bool {

    match (state, newState) {
        (&State::Undefined,        &State::Var             ) => return true,
        (&State::Undefined,        &State::ParenthesisOpen ) => return true,
        (&State::Var,              &State::And             ) => return true,
        (&State::Var,              &State::Or              ) => return true,
        (&State::Var,              &State::Xor             ) => return true,
        (&State::Var,              &State::ThenBegin       ) => return true,
        (&State::Var,              &State::ParenthesisClose) => return true,
        (&State::ThenBegin,        &State::ThenEnd         ) => return true,
        (&State::ThenEnd,          &State::Var             ) => return true,
        (&State::And,              &State::Var             ) => return true,
        (&State::Or,               &State::Var             ) => return true,
        (&State::Xor,              &State::Var             ) => return true,
        (&State::ParenthesisOpen,  &State::Var             ) => return true,
        (&State::ParenthesisClose, &State::And             ) => return true,
        (&State::ParenthesisClose, &State::Or              ) => return true,
        (&State::ParenthesisClose, &State::Xor             ) => return true,
        (&State::ParenthesisClose, &State::ThenBegin       ) => return true,
        _ => return false
    }
 
}

#[derive(Debug)]
enum TokenId {
    Undefined,
    Incomplete,
    Var,
    And,
    Or,
    Xor,
    Then,
    ParenthesisOpen,
    ParenthesisClose
}

struct Token {
    id: TokenId,
    s: String
}

fn lexer(c: char) -> (State, Token) {

    let newState =
    match c {
        '+' => State::And,
        '|' => State::Or,
        '^' => State::Xor,
        '=' => State::ThenBegin,
        '>' => State::ThenEnd,
        '(' => State::ParenthesisOpen,
        ')' => State::ParenthesisClose,
        _ => if c.is_uppercase() { State::Var } else { State::Undefined /* Error */ }
    };

    let token =
    match newState {
        State::Var              => Token { id: TokenId::Var,              s: c.to_string()    },
        State::And              => Token { id: TokenId::And,              s: c.to_string()    },
        State::Or               => Token { id: TokenId::Or,               s: c.to_string()    },
        State::Xor              => Token { id: TokenId::Xor,              s: c.to_string()    },
        State::ThenEnd          => Token { id: TokenId::Then,             s: "=>".to_string() },
        State::ParenthesisOpen  => Token { id: TokenId::ParenthesisOpen,  s: c.to_string()    },
        State::ParenthesisClose => Token { id: TokenId::ParenthesisClose, s: c.to_string()    },
        State::Undefined        => Token { id: TokenId::Undefined,        s: c.to_string()    }, /* Error */
        _                       => Token { id: TokenId::Incomplete,       s: c.to_string()    }
    };

    return (newState, token);
}

fn new_rule(rule_str: String) {

    let mut state = State::Undefined;

    for c in rule_str.chars() {
        let (newState, token) = lexer(c);

        println!("New token: {} id {:?}", token.s, token.id);

        if !state_is_valid(&state, &newState) {
            println!("Unexpected token: '{}'", token.s);
        }

        // parse()

        state = newState;
    }
}

extern crate regex;

use std::io::BufRead; /* stdin().lock() */
use std::io::Write;   /* stdout().flush() */
use regex::Regex;

fn main() {
    let re = Regex::new("[[:space:]]").unwrap();
    let stdin = std::io::stdin();

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
