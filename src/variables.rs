use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, PartialEq, Clone)]
pub enum VariableState {
    Undefined,
    Unsolved,
    True,
    False
}

lazy_static! {
    pub static ref VariableMap: Mutex<HashMap<&'static str, VariableState>> = {
        let mut map = HashMap::new();

        map.insert("A", VariableState::False);
        map.insert("B", VariableState::False);
        map.insert("C", VariableState::False);
        map.insert("D", VariableState::False);
        map.insert("E", VariableState::False);
        map.insert("F", VariableState::False);
        map.insert("G", VariableState::False);
        map.insert("I", VariableState::False);
        map.insert("J", VariableState::False);
        map.insert("K", VariableState::False);
        map.insert("L", VariableState::False);
        map.insert("M", VariableState::False);
        map.insert("N", VariableState::False);
        map.insert("O", VariableState::False);
        map.insert("P", VariableState::False);
        map.insert("Q", VariableState::False);
        map.insert("R", VariableState::False);
        map.insert("S", VariableState::False);
        map.insert("T", VariableState::False);
        map.insert("U", VariableState::False);
        map.insert("V", VariableState::False);
        map.insert("W", VariableState::False);
        map.insert("X", VariableState::False);
        map.insert("Y", VariableState::False);
        map.insert("Z", VariableState::False);

        Mutex::new(map)
    };
}
