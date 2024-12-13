use std::{collections::HashMap, rc::Rc};

use super::{state::State, transition::Transition};

pub struct StateMachine {
    pub states: Vec<Rc<State>>,
    pub transitions: Vec<Rc<dyn Transition>>,
    pub start: Rc<State>,
    pub current_indentation: i32,
    pub indentation_character: String,
}

impl StateMachine {
    pub fn new(start: Rc<State>, indentation_character: String, current_indentation: i32) -> Self {
        Self {
            states: Vec::new(),
            transitions: Vec::new(),
            start,
            current_indentation,
            indentation_character,
        }
    }

    pub fn validate(&self, buffer: String) -> (bool, usize) {
        self.validate_from(buffer, 0, 0)
    }

    pub fn check(&self, buffer: String) -> bool {
        let (validated, offset) = self.validate(buffer.clone());
        validated && offset == buffer.len()
    }

    pub fn validate_from(&self, buffer: String, from: usize, indentation: i32) -> (bool, usize) {
        let mut current_state = self.start.clone();
        let mut offset = from;
        let mut current_indentation = indentation;
        let mut transition_map: HashMap<Rc<State>, Vec<&Rc<(dyn Transition)>>> = HashMap::new();

        //init data structure so it gets dropped by the borrow checker when no longer needed
        for transition in &self.transitions {
            let from = transition.from();
            transition_map
                .entry(from)
                .or_insert_with(Vec::new)
                .push(transition);
        }

        while offset < buffer.len() {
            if !transition_map.contains_key(&current_state) {
                if offset < 1 {
                    return (false, offset);
                }
                return (current_state.is_final(), offset);
            }

            let transitions = transition_map.get(&current_state).unwrap();
            for transition in transitions {
                match transition.to(
                    buffer.clone(),
                    offset,
                    current_indentation,
                    self.indentation_character.clone(),
                ) {
                    Ok(next_state) => {
                        let (new_current_state, offset_inc) = next_state;
                        current_state = new_current_state;
                        offset += offset_inc;
                        if offset < 1 {
                            return (false, offset);
                        }

                        match transition.indentation_operation() {
                            super::transition::IndentationOperation::BYPASS => {
                                current_indentation += 0;
                            }
                            super::transition::IndentationOperation::INCREMENT => {
                                current_indentation += 1;
                            }
                            super::transition::IndentationOperation::DESINCREMENT => {
                                current_indentation -= 1;
                            }
                            super::transition::IndentationOperation::CONSERVE => {
                                current_indentation += 0;
                            }
                            super::transition::IndentationOperation::RESET => {
                                current_indentation += 0;
                            }
                        };
                        break;
                    }
                    Err(ErrorTransition) => {}
                };
            }
        }

        if offset < 1 {
            return (false, offset);
        }
        (current_state.is_final(), offset)
    }
}

pub struct StateMachineBuilder {
    transitions: Vec<Rc<dyn Transition>>,
    states: Vec<Rc<State>>,
    current_indentation: i32,
    start: Rc<State>,
    indentation_character: String,
}

impl StateMachineBuilder {
    pub fn new(start: Rc<State>, indentation_character: &str, current_indentation: i32) -> Self {
        let indentation_character = indentation_character.to_string();
        let state_machine = StateMachineBuilder {
            states: vec![start.clone()],
            transitions: Vec::new(),
            start,
            current_indentation,
            indentation_character,
        };
        state_machine
    }

    pub fn add_transition(&mut self, transition: Rc<dyn Transition>) -> &mut Self {
        self.transitions.push(transition);
        self
    }

    pub fn add_transitions(&mut self, transitions: Vec<Rc<dyn Transition>>) -> &mut Self {
        for transition in transitions {
            self.add_transition(transition);
        }
        self
    }

    pub fn add_state(&mut self, state: Rc<State>) -> &mut Self {
        self.states.push(state);
        self
    }

    pub fn add_states(&mut self, states: Vec<Rc<State>>) -> &mut Self {
        for state in states {
            self.add_state(state);
        }
        self
    }

    pub fn build(&self) -> StateMachine {
        StateMachine {
            states: self.states.clone(),
            current_indentation: self.current_indentation,
            indentation_character: self.indentation_character.clone(),
            start: self.start.clone(),
            transitions: self.transitions.clone(),
        }
    }
}
