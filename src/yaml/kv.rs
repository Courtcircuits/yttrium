use std::rc::Rc;

use crate::grammar::{
    state::create_state,
    state_machine::{StateMachine, StateMachineBuilder},
    transition::{CharTransition, IndentationOperation},
};

use super::{scalar::scalar_transition, value::value_transition};

pub fn kv_state_machine(indentation: i32) -> StateMachine {
    let begin = Rc::new(create_state(false, "start"));
    let key = Rc::new(create_state(false, "key"));
    let column = Rc::new(create_state(false, "column"));
    let value = Rc::new(create_state(true, "value"));

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

    let automaton = StateMachineBuilder::new(begin, " ", indentation)
        .add_states(vec![key, column, value])
        .add_transitions(vec![b_k, k_c, c_v])
        .build();

    automaton
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
    fn test_kv_state_machine_recognize_sequence() {
        let kv = "salut:
 -poulet
 -poulet";
        let machine = kv_state_machine(0);
        let (result, offset) = machine.validate(kv.to_string());
        assert_eq!(result, true);
        assert_eq!(kv.len(), offset);
    }
}
