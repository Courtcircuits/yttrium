use core::fmt;
use std::rc::Rc;

use tracing::debug;

use super::{
    state::{create_state, State},
    state_machine::{StateMachine, StateMachineBuilder},
};

pub enum ErrorTransition {
    InvalidTransition,
}

#[derive(Clone)]
pub enum IndentationOperation {
    BYPASS = 2,
    INCREMENT = 1,
    DESINCREMENT = -1,
    CONSERVE = 0,
    RESET = -2,
}
pub struct CharTransition {
    //using rc because state can be shared between multiple transitions but no mutation should
    //occur
    pub from: Rc<State>,
    pub to: Rc<State>,
    pub value: String,
    pub indentation_operation: IndentationOperation,
}

pub struct EpsilonTransition {
    pub from: Rc<State>,
    pub to: Rc<State>,
}

pub struct GroupTransition {
    pub from: Rc<State>,
    pub to: Rc<State>,
    pub value: Box<StateMachine>,
    pub indentation_operation: IndentationOperation,
}

pub trait Transition {
    fn from(&self) -> Rc<State>;
    fn to(
        &self,
        buffer: String,
        offset: usize,
        current_indentation: i32,
        indentation_character: String,
    ) -> Result<(Rc<State>, usize), ErrorTransition>;
    fn indentation_operation(&self) -> IndentationOperation;
}

impl Transition for GroupTransition {
    fn from(&self) -> Rc<State> {
        self.from.clone()
    }

    fn to(
        &self,
        buffer: String,
        offset: usize,
        _current_indentation: i32,
        _indentation_character: String,
    ) -> Result<(Rc<State>, usize), ErrorTransition> {
        let (validation, new_offset) =
            (*self.value).validate_from(buffer, offset, _current_indentation);
        if validation {
            Ok((self.to.clone(), new_offset - offset))
        } else {
            Err(ErrorTransition::InvalidTransition)
        }
    }

    fn indentation_operation(&self) -> IndentationOperation {
        self.indentation_operation.clone()
    }
}

impl GroupTransition {
    pub fn new(
        from: Rc<State>,
        to: Rc<State>,
        state_machine: StateMachine,
        indentation_operation: IndentationOperation,
    ) -> Self {
        GroupTransition {
            from,
            to,
            value: Box::new(state_machine),
            indentation_operation,
        }
    }
}

impl Transition for CharTransition {
    fn from(&self) -> Rc<State> {
        self.from.clone()
    }
    fn to(
        &self,
        buffer: String,
        offset: usize,
        current_indentation: i32,
        indentation_character: String,
    ) -> Result<(Rc<State>, usize), ErrorTransition> {
        if buffer.chars().nth(offset).unwrap().to_string() == self.value {
            debug!("is: {}", self.to.label);
            match self.indentation_operation {
                IndentationOperation::BYPASS => Ok((self.to.clone(), 1)),
                IndentationOperation::INCREMENT => {
                    let offset = offset as i32;
                    for n in (offset + 1)..(offset + 2 + current_indentation) {
                        if buffer.chars().nth(n as usize).unwrap().to_string()
                            == indentation_character
                        {
                            continue;
                        }
                        return Err(ErrorTransition::InvalidTransition);
                    }
                    debug!("adding offset : {} ", current_indentation + 2);
                    Ok((self.to.clone(), (current_indentation + 2) as usize))
                }
                IndentationOperation::DESINCREMENT => {
                    let offset = offset as i32;
                    if current_indentation == 0 {
                        return Err(ErrorTransition::InvalidTransition);
                    }
                    for n in (offset + 1)..(offset + 1 + current_indentation - 1) {
                        if buffer.chars().nth(n as usize).unwrap().to_string()
                            == indentation_character
                        {
                            continue;
                        }
                        return Err(ErrorTransition::InvalidTransition);
                    }
                    Ok((self.to.clone(), (current_indentation) as usize))
                }
                IndentationOperation::CONSERVE => {
                    let offset = offset as i32;
                    for n in (offset + 1)..(offset + 1 + current_indentation) {
                        if buffer.chars().nth(n as usize).unwrap().to_string()
                            == indentation_character
                        {
                            continue;
                        }
                        return Err(ErrorTransition::InvalidTransition);
                    }
                    Ok((self.to.clone(), (current_indentation + 1) as usize))
                }
                IndentationOperation::RESET => Ok((self.to.clone(), 1)),
            }
        } else {
            debug!("not: {}", self.to.label);
            Err(ErrorTransition::InvalidTransition)
        }
    }

    fn indentation_operation(&self) -> IndentationOperation {
        self.indentation_operation.clone()
    }
}

impl CharTransition {
    pub fn new(
        from: Rc<State>,
        to: Rc<State>,
        check: String,
        indentation_operation: IndentationOperation,
    ) -> CharTransition {
        CharTransition {
            from,
            to,
            value: check,
            indentation_operation,
        }
    }
}

impl fmt::Debug for CharTransition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} -> {} -> {:?}", self.from, self.value, self.to)
    }
}

impl Transition for EpsilonTransition {
    fn from(&self) -> Rc<State> {
        self.from.clone()
    }
    fn to(
        &self,
        _buffer: String,
        _offset: usize,
        _current_indentation: i32,
        _indentation_character: String,
    ) -> Result<(Rc<State>, usize), ErrorTransition> {
        Ok((self.to.clone(), 0))
    }
    fn indentation_operation(&self) -> IndentationOperation {
        IndentationOperation::BYPASS
    }
}

impl EpsilonTransition {
    pub fn new(from: Rc<State>, to: Rc<State>) -> EpsilonTransition {
        EpsilonTransition { from, to }
    }
}

pub fn create_char_transitions(
    from: Rc<State>,
    to: Rc<State>,
    alphabet: String,
    indentation_operation: IndentationOperation,
) -> Vec<Rc<dyn Transition>> {
    let mut transitions: Vec<Rc<dyn Transition>> = Vec::new();
    let letters: Vec<&str> = alphabet.split("").collect();
    for letter in letters {
        transitions.push(Rc::new(CharTransition::new(
            from.clone(),
            to.clone(),
            letter.to_string(),
            indentation_operation.clone(),
        )));
    }
    transitions
}

pub fn create_word_transition(
    from: Rc<State>,
    to: Rc<State>,
    word: String,
    indentation_operation: IndentationOperation,
    current_indentation: i32,
) -> Rc<dyn Transition> {
    let mut states: Vec<Rc<State>> = Vec::new();
    let mut transitions: Vec<Rc<dyn Transition>> = Vec::new();
    let mut start = Rc::new(create_state(true, "zob"));

    for i in 0..word.len() + 1 {
        let state = Rc::new(create_state(
            i == word.len(),
            format!("letter-{}", i).as_str(),
        ));

        if i == 0 {
            start = state.clone();
        }

        // println!("state : {:?}", state);
        states.push(state.clone());

        if i > 0 {
            let char = word.chars().nth(i - 1).unwrap().to_string();
            let transition = Rc::new(CharTransition::new(
                states[i - 1].clone(),
                state.clone(),
                char.clone(),
                IndentationOperation::BYPASS,
            ));
            transitions.push(transition);
        }
    }

    let state_machine = StateMachineBuilder::new(start.clone(), " ", current_indentation)
        .add_states(states.clone())
        .add_transitions(transitions)
        .build();

    Rc::new(GroupTransition::new(
        from,
        to,
        state_machine,
        indentation_operation,
    ))
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::grammar::{state::create_state, state_machine::StateMachineBuilder};

    use super::{create_word_transition, IndentationOperation};

    #[test]
    fn test_word_transition() {
        let word = "---".to_string();
        let start = Rc::new(create_state(false, "start"));
        let end = Rc::new(create_state(true, "end"));
        let transition = create_word_transition(
            start.clone(),
            end.clone(),
            word.clone(),
            IndentationOperation::BYPASS,
            0,
        );

        let machine = StateMachineBuilder::new(start, " ", 0)
            .add_state(end)
            .add_transition(transition)
            .build();

        let (result, offset) = machine.validate(word.clone());
        assert_eq!(result, true);
        assert_eq!(word.len(), offset);
    }
}
