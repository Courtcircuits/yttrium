use std::rc::Rc;

use grammar::{
    automaton::{Automaton, BasicAutomaton},
    state::create_state,
    transition::{CharTransition, Transition},
};

pub mod grammar;

fn main() {
    let a = Rc::new(create_state(false));
    let b = Rc::new(create_state(true));
    let buffer = "asdsadsa".to_string();

    let mut transition_a = CharTransition::new(a.clone(), b.clone(), buffer.clone());

    let mut automaton = BasicAutomaton::new()
        .add_state(a.clone())
        .add_state(b.clone())
        .add_transition(Box::new(transition_a));
}
