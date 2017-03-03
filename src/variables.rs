use std::collections::HashMap;
use std::collections::LinkedList;
use std::sync::Mutex;

use BTree;

#[derive(Debug, PartialEq, Clone)]
pub enum VariableState {
    Undefined,
    True,
    False
}

pub struct Variable <'a> {
    pub state: VariableState,
    pub rules: LinkedList<&BTree>
}

lazy_static! {
    pub static ref VariableMap: Mutex<HashMap<&'static str, Variable>> = {

        let mut map = HashMap::new();

        map.insert("A", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("B", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("C", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("D", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("E", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("F", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("G", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("I", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("J", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("K", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("L", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("M", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("N", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("O", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("P", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("Q", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("R", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("S", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("T", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("U", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("V", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("W", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("X", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("Y", Variable { state: VariableState::False, rules: LinkedList::new() });
        map.insert("Z", Variable { state: VariableState::False, rules: LinkedList::new() });

        Mutex::new(map)
    };
}

