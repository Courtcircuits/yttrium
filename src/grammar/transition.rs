use std::rc::Rc;

use super::state::State;

pub enum ErrorTransition {
    InvalidTransition,
}

pub struct CharTransition {
    //using rc because state can be shared between multiple transitions but no mutation should
    //occur
    pub from: Rc<State>,
    pub to: Rc<State>,
    pub value: String,
}

pub struct EpsilonTransition {
    pub from: Rc<State>,
    pub to: Rc<State>,
}

pub trait Transition {
    fn from(&self) -> Rc<State>;
    fn new(from: Rc<State>, to: Rc<State>, check: String) -> Self;
    fn to(&self, buffer: String, offset: i32) -> Result<Rc<State>, ErrorTransition>;
}

impl Transition for CharTransition {
    fn new(from: Rc<State>, to: Rc<State>, check: String) -> CharTransition {
        CharTransition {
            from,
            to,
            value: check,
        }
    }
    fn from(&self) -> Rc<State> {
        self.from.clone()
    }
    fn to(&self, buffer: String, offset: i32) -> Result<Rc<State>, ErrorTransition> {
        if buffer.chars().nth(offset as usize).unwrap().to_string() == self.value {
            Ok(self.to.clone())
        } else {
            Err(ErrorTransition::InvalidTransition)
        }
    }
}

impl Transition for EpsilonTransition {
    fn new(from: Rc<State>, to: Rc<State>, _useless: String) -> EpsilonTransition {
        EpsilonTransition { from, to }
    }
    fn from(&self) -> Rc<State> {
        self.from.clone()
    }
    fn to(&self, _buffer: String, _offset: i32) -> Result<Rc<State>, ErrorTransition> {
        Ok(self.to.clone())
    }
}
