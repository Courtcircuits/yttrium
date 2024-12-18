use std::rc::Rc;

use crate::grammar::{
    state::create_state,
    state_machine::{StateMachine, StateMachineBuilder},
    transition::{CharTransition, EpsilonTransition, GroupTransition, IndentationOperation},
};

use super::sequence::sequence_state_machine;

fn three_tick_machine(indentation: i32) -> StateMachine {
    let begin = Rc::new(create_state(false, "start"));
    let first = Rc::new(create_state(false, "first"));
    let second = Rc::new(create_state(false, "second"));
    let last = Rc::new(create_state(false, "last"));
    let eol = Rc::new(create_state(true, "eol"));

    let b_f = Rc::new(CharTransition::new(
        begin.clone(),
        first.clone(),
        "-".to_string(),
        IndentationOperation::BYPASS,
    ));

    let f_s = Rc::new(CharTransition::new(
        first.clone(),
        second.clone(),
        "-".to_string(),
        IndentationOperation::BYPASS,
    ));

    let s_l = Rc::new(CharTransition::new(
        second.clone(),
        last.clone(),
        "-".to_string(),
        IndentationOperation::BYPASS,
    ));

    let l_e = Rc::new(CharTransition::new(
        last.clone(),
        eol.clone(),
        "\n".to_string(),
        IndentationOperation::CONSERVE,
    ));

    StateMachineBuilder::new(begin, " ", indentation)
        .add_transitions(vec![b_f, f_s, s_l, l_e])
        .add_states(vec![first, second, last, eol])
        .build()
}

pub fn document_state_machine(indentation: i32) -> StateMachine {
    let begin = Rc::new(create_state(false, "start")); //value can't be empty
    let start_body = Rc::new(create_state(false, "start_body"));
    let body = Rc::new(create_state(false, "body"));
    let end_body = Rc::new(create_state(true, "end_body"));

    let b_s = Rc::new(GroupTransition::new(
        begin.clone(),
        start_body.clone(),
        three_tick_machine(indentation),
        IndentationOperation::BYPASS,
    ));

    let s_b = Rc::new(GroupTransition::new(
        start_body.clone(),
        body.clone(),
        sequence_state_machine(indentation),
        IndentationOperation::BYPASS,
    ));

    let b_s_e = Rc::new(EpsilonTransition::new(body.clone(), start_body.clone()));
    let s_b_e = Rc::new(EpsilonTransition::new(start_body.clone(), body.clone()));

    let b_e = Rc::new(GroupTransition::new(
        body.clone(),
        end_body.clone(),
        three_tick_machine(indentation),
        IndentationOperation::BYPASS,
    ));

    StateMachineBuilder::new(begin, " ", indentation)
        .add_transitions(vec![b_s, s_b, b_s_e, s_b_e, b_e])
        .add_states(vec![start_body, body, end_body])
        .build()
}

// #[cfg(test)]
// mod tests {
//     use std::result;

//     use super::document_state_machine;

//     #[test]
//     fn test_document_state_machine_recognize_empty_doc() {
//         let val = "---
// ---
// ";
//         let machine = document_state_machine(0);
//         let (result, offset) = machine.validate(val.to_string());
//         assert_eq!(result, true);
//         assert_eq!(val.len(), offset);
//     }
// }
