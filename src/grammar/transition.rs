use std::{env::current_dir, rc::Rc};

use tracing::{debug, info};

use super::{state::State, state_machine::StateMachine};

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
