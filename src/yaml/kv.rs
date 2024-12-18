use std::{io::Stdout, iter::Scan, rc::Rc};

use tracing::debug;

use crate::grammar::{
    state::{create_state, State},
    state_machine::{StateMachine, StateMachineBuilder},
    transition::{CharTransition, EpsilonTransition, GroupTransition, IndentationOperation},
};

use super::{scalar::scalar_transition, value::value_transition};

pub fn kv_state_machine(indentation: i32) -> StateMachine {
    let begin = Rc::new(create_state(false, "start"));
    let key = Rc::new(create_state(false, "key"));
    let column = Rc::new(create_state(false, "column"));
    let value = Rc::new(create_state(true, "value"));
    let nested_kv = Rc::new(create_state(false, "nested_kv"));
    let next_kv = Rc::new(create_state(false, "next_kv"));

    let b_k = Rc::new(scalar_transition(
        begin.clone(),
        key.clone(),
        indentation,
        IndentationOperation::BYPASS,
    ));

    let k_c = Rc::new(CharTransition::new(
        key.clone(),
        column.clone(),
        ":".to_string(),
        IndentationOperation::BYPASS,
    ));

    let c_v = Rc::new(value_transition(
        column.clone(),
        value.clone(),
        indentation,
        IndentationOperation::BYPASS,
    ));

    let c_n = Rc::new(CharTransition::new(
        column.clone(),
        nested_kv.clone(),
        "\n".to_string(),
        IndentationOperation::INCREMENT,
    ));

    let n_b = Rc::new(scalar_transition(
        nested_kv.clone(),
        key.clone(),
        indentation,
        IndentationOperation::BYPASS,
    ));

    let val_next = Rc::new(CharTransition::new(
        value.clone(),
        next_kv.clone(),
        "\n".to_string(),
        IndentationOperation::CONSERVE,
    ));

    let next_key = Rc::new(scalar_transition(
        next_kv.clone(),
        key.clone(),
        indentation,
        IndentationOperation::BYPASS,
    ));

    let automaton = StateMachineBuilder::new(begin, " ", indentation)
        .add_states(vec![key, column, value, nested_kv, next_kv])
        .add_transitions(vec![b_k, k_c, c_v, c_n, n_b, val_next, next_key])
        .build();

    debug!("built kv state machine");

    automaton
}

pub fn kv_transition(
    from: Rc<State>,
    to: Rc<State>,
    indentation: i32,
    operation: IndentationOperation,
) -> GroupTransition {
    GroupTransition::new(from, to, kv_state_machine(indentation), operation)
}

#[cfg(test)]
mod tests {

    use super::kv_state_machine;

    #[test]
    fn test_kv_state_machine_recognize_kv() {
        let kv = "salut:poulet";
        let machine = kv_state_machine(0);
        let (result, offset) = machine.validate(kv.to_string());
        assert_eq!(result, true);
        assert_eq!(kv.len(), offset);
    }

    #[test]
    fn test_kv_state_machine_recognize_kv_nested() {
        let kv = "salut:
 test:zob";
        let machine = kv_state_machine(0);
        let (result, offset) = machine.validate(kv.to_string());
        assert_eq!(result, true);
        assert_eq!(kv.len(), offset);
    }

    #[test]
    fn test_kv_state_machine_recognize_kv_next() {
        let kv = "salut:zob
test:zob";
        let machine = kv_state_machine(0);
        let (result, offset) = machine.validate(kv.to_string());
        assert_eq!(result, true);
        assert_eq!(kv.len(), offset);
    }

    #[test]
    fn test_kv_state_machine_recognize_sequence() {
        let kv = "salut:
 -poulet
 -deux
 -trois";
        let machine = kv_state_machine(0);
        let (result, offset) = machine.validate(kv.to_string());
        assert_eq!(result, true);
        assert_eq!(kv.len(), offset);
    }

    #[test]
    fn test_kv_state_machine_dont_recognize_sequence() {
        let kv = "salut:

 -poulet
 -deux
 -trois";
        let machine = kv_state_machine(0);
        let (result, offset) = machine.validate(kv.to_string());
        assert_eq!(result, false);
        assert_eq!("salut:".len(), offset);
    }

    #[test]
    fn test_kv_state_machine_dont_recognize_sequence_indent() {
        let kv = "salut:
 -poule
    -deux
 -trois";
        let machine = kv_state_machine(0);
        let (result, _) = machine.validate(kv.to_string());
        assert_eq!(result, false);
        assert_ne!("salut:".len(), kv.len());
    }
}
