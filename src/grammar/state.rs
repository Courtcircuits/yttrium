use core::fmt;
use std::hash::Hash;
use std::hash::Hasher;

pub struct State {
    pub is_final: bool,
    pub label: String,
}

pub fn create_state(is_final: bool, label: &str) -> State {
    State::new(is_final, label.to_string())
}

impl State {
    pub fn new(is_final: bool, label: String) -> State {
        State { is_final, label }
    }
    pub fn is_final(&self) -> bool {
        self.is_final
    }
    pub fn equals(&self, other: &State) -> bool {
        self.label == other.label
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if (*self).is_final {
            return write!(f, "|{}|", self.label);
        }
        write!(f, "{}", self.label)
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.label.hash(state);
    }
}
