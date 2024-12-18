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
    let state_start = Rc::new(create_state(false, "start_word"));
    let state_end_pair = Rc::new(create_state(true, "end_pair_word"));
    let state_end_impair = Rc::new(create_state(true, "end_impair_word"));
    let transitions_alpha_start = create_char_transitions(
        state_start.clone(),
        state_end_impair.clone(),
        alphabet.clone(),
        transition::IndentationOperation::BYPASS,
    );
    let transitions_alpha_pair = create_char_transitions(
        state_end_impair.clone(),
        state_end_pair.clone(),
        alphabet.clone(),
        transition::IndentationOperation::BYPASS,
    );
    let transitions_alpha_impair = create_char_transitions(
        state_end_pair.clone(),
        state_end_impair.clone(),
        alphabet,
        transition::IndentationOperation::BYPASS,
    );
    let automaton = StateMachineBuilder::new(state_start, " ", indentation)
        .add_states(vec![state_end_pair, state_end_impair])
        .add_transitions(transitions_alpha_pair)
        .add_transitions(transitions_alpha_start)
        .add_transitions(transitions_alpha_impair)
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
        let machine = scalar_state_machine(0);
        let (result, offset) = machine.validate(word.to_string());
        assert_eq!(result, true);
        assert_eq!(offset, word.len());
    }

    #[test]
    fn test_scalar_state_machine_recognize_words_pair() {
        let word = "Bon;j";
        let machine = scalar_state_machine(0);
        let (result, offset) = machine.validate(word.to_string());
        assert_eq!(result, true);
        assert_eq!(offset, word.len() - 2);
    }

    #[test]
    fn test_scalar_state_machine_doesnt_recognize_words() {
        let word = "wqejklwq;s"; // ; is an invalid character
        let machine = scalar_state_machine(0);
        let (result, offset) = machine.validate(word.to_string());
        assert_eq!(result, true);
        assert_eq!(offset, word.len() - 2);
    }
}
