use std::collections::HashMap;
use std::collections::LinkedList;
use std::sync::Mutex;

use btree::BTree;

use MyOption::MyOption;

#[derive(Debug, PartialEq, Clone)]
pub enum VariableState {
    Undefined,
    True,
    False
}

#[derive(Clone)]
pub struct Variable {
    pub state: VariableState,
    pub rules: LinkedList<BTree>
}

impl Variable {
    pub fn solve(&self, closed: String) -> MyOption<VariableState> {
        let mut s = String::new();
        let mut state = VariableState::Undefined;
        for ref rule in &self.rules {
            let rule_state = {
                match rule.solve(closed.clone()) {
                    MyOption::Some(state) => state,
                    MyOption::Error(s) => return MyOption::Error(s)
                }
            };
            if state == VariableState::Undefined {
                state = rule_state.clone();
            }
            else if state != rule_state {
                return MyOption::Error(format!("Inconsistant state {:?} != {:?}", state, rule_state));
            }
            s.push_str(&rule.to_string());
            s.push_str(&"\n".to_string());
        }
        if state != VariableState::Undefined {
            print!("{}", s);
        }
        MyOption::Some(state)
    }
}

pub const VARS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

lazy_static! {
    pub static ref VARIABLEMAP: Mutex<HashMap<&'static str, Variable>> = {

        let mut map = HashMap::new();

        for i in 0..VARS.len() {

            let var = &VARS[i..i+1];

            map.insert(var, Variable { state: VariableState::Undefined, rules: LinkedList::new() });
        }

        Mutex::new(map)
    };
}

