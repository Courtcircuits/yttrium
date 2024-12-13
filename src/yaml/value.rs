use std::rc::Rc;

use crate::grammar::{
    state::{create_state, State},
    state_machine::{StateMachine, StateMachineBuilder},
    transition::{CharTransition, GroupTransition, IndentationOperation},
};

use super::{scalar::scalar_transition, sequence::sequence_transition};

pub fn value_state_machine(indentation: i32) -> StateMachine {
    let begin = Rc::new(create_state(false, "start")); //value can't be empty
    let scalar = Rc::new(create_state(true, "scalar"));
    let multiline = Rc::new(create_state(false, "multiline"));
    let sequence = Rc::new(create_state(true, "sequence"));

    let b_s = Rc::new(scalar_transition(
        begin.clone(),
        scalar.clone(),
        indentation,
        crate::grammar::transition::IndentationOperation::BYPASS,
    ));

    let b_m = Rc::new(CharTransition::new(
        begin.clone(),
        multiline.clone(),
        "\n".to_string(),
        crate::grammar::transition::IndentationOperation::INCREMENT,
    ));
    let m_s = Rc::new(sequence_transition(
        multiline.clone(),
        sequence.clone(),
        indentation,
        crate::grammar::transition::IndentationOperation::BYPASS,
    ));

    let automaton = StateMachineBuilder::new(begin, " ", indentation)
        .add_transitions(vec![b_s, b_m, m_s])
        .add_states(vec![scalar, multiline, sequence])
        .build();
    automaton
}

pub fn value_transition(
    from: Rc<State>,
    to: Rc<State>,
    indentation: i32,
    operation: IndentationOperation,
) -> GroupTransition {
    GroupTransition::new(from, to, value_state_machine(indentation), operation)
}

#[cfg(test)]
mod tests {
    use super::value_state_machine;

    #[test]
    fn test_value_state_machine_recognize_scalar() {
        let val = "ewqewq";
        let machine = value_state_machine(0);

        let (result, offset) = machine.validate(val.to_string());
        assert_eq!(result, true);
        assert_eq!(val.len(), offset);
    }

    #[test]
    fn test_value_state_machine_recognize_seq() {
        let val = "
 -test
 -ewq
 -adssca";
        let machine = value_state_machine(0);

        let (result, offset) = machine.validate(val.to_string());
        assert_eq!(result, true);
        assert_eq!(val.len(), offset);
    }
}
