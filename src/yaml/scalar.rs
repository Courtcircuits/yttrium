use std::rc::Rc;

use crate::grammar::{
    state::{create_state, State},
    state_machine::{StateMachine, StateMachineBuilder},
    transition::{
        self, create_char_transitions, EpsilonTransition, GroupTransition, IndentationOperation,
    },
};

pub fn scalar_state_machine(indentation: i32) -> StateMachine {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz ".to_string();
    let state_start = Rc::new(create_state(true, "start"));
    let state_end = Rc::new(create_state(true, "end"));
    let transitions_alpha = create_char_transitions(
        state_start.clone(),
        state_end.clone(),
        alphabet,
        transition::IndentationOperation::BYPASS,
    );
    let transition_e = EpsilonTransition::new(state_end.clone(), state_start.clone());
    let automaton = StateMachineBuilder::new(state_start, " ", indentation)
        .add_state(state_end)
        .add_transitions(transitions_alpha)
        .add_transition(Rc::new(transition_e))
        .build();

    automaton
}

pub fn scalar_transition(
    from: Rc<State>,
    to: Rc<State>,
    indentation: i32,
    operation: IndentationOperation,
) -> GroupTransition {
    GroupTransition::new(from, to, scalar_state_machine(indentation), operation)
}

#[cfg(test)]
mod tests {

    use super::scalar_state_machine;

    #[test]
    fn test_scalar_state_machine_recognize_words() {
        let word = "Bonjour je suis tristan";
        let mut machine = scalar_state_machine(0);
        let (result, offset) = machine.validate(word.to_string());
        assert_eq!(result, true);
        assert_eq!(offset, word.len());
    }

    #[test]
    fn test_scalar_state_machine_doesnt_recognize_words() {
        let word = "wqejklwq;s"; // ; is an invalid character
        let mut machine = scalar_state_machine(0);
        let (result, offset) = machine.validate(word.to_string());
        assert_eq!(result, true);
        assert_eq!(offset, word.len() - 2);
    }
}
