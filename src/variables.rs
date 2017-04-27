use std::collections::HashMap;
use std::collections::LinkedList;
use std::sync::Mutex;

use btree::BTree;

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
    pub fn solve(&self) -> VariableState {
        let mut s = String::new();
        let mut state = VariableState::Undefined;
        for ref rule in &self.rules {
            let rule_state = {
                match rule.solve() {
                    VariableState::True => VariableState::True,
                    _ => VariableState::False
                }
            };
            if state == VariableState::Undefined {
                state = rule_state.clone();
            }
            else if state != rule_state {
                return VariableState::Undefined;
            }
            s.push_str(&rule.to_string());
            s.push_str(&"\n".to_string());
        }
        print!("{}", s);
        state
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

pub fn reset() {

    let mut variables = VARIABLEMAP.lock().unwrap();

    for i in 0..VARS.len() {

        let var = variables.get_mut(&VARS[i..i+1]).unwrap();

        for ref mut rule in &mut var.rules {

            rule.state = VariableState::Undefined;
        }
    }
}

