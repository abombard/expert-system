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

pub const VARS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

lazy_static! {
    pub static ref VARIABLEMAP: Mutex<HashMap<&'static str, Variable>> = {

        let mut map = HashMap::new();

        for i in 0..VARS.len() {

            let var = &VARS[i..i+1];

            map.insert(var, Variable { state: VariableState::False, rules: LinkedList::new() });
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

