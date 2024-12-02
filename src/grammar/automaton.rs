use std::rc::Rc;

use super::{
    state::State,
    transition::{CharTransition, Transition},
};

pub struct BasicAutomaton {
    pub states: Vec<Rc<State>>,
    pub transitions: Vec<Box<dyn Transition>>,
    pub start: Rc<State>,
}

pub trait Automaton {
    fn new() -> Self;
    fn validate(&self, buffer: String) -> bool;
    fn add_transition(&mut self, transition: Box<dyn Transition>) -> Self;
    fn add_state(&mut self, state: Rc<State>) -> Self;
}

impl Automaton for BasicAutomaton {
    fn validate(&self, buffer: String) -> bool {
        let mut current_state = self.start.clone();
        let mut offset = 0;
        while offset < buffer.len() as i32 {
            let mut found = false;
            for transition in self.transitions.iter() {
                if transition.from().equals(&current_state) {
                    match transition.to(buffer.clone(), offset) {
                        Ok(next_state) => {
                            current_state = next_state;
                            offset += 1;
                            found = true;
                            break;
                        }
                        Err(_) => {}
                    }
                }
            }
            if !found {
                return false;
            }
        }
        current_state.is_final()
    }
    fn add_transition(&mut self, transition: Box<CharTransition>) -> Self {
        self.transitions.push(transition);
        self
    }
    fn add_state(&mut self, state: Rc<State>) -> Self {
        self.states.push(state);
        self
    }

    fn new() -> BasicAutomaton {
        BasicAutomaton {
            states: Vec::new(),
            transitions: Vec::new(),
            start: Rc::new(State::new(false)),
        }
    }
}
