use std::rc::Rc;

use crate::grammar::{
    state::create_state,
    state_machine::{StateMachine, StateMachineBuilder},
    transition::{create_word_transition, CharTransition, IndentationOperation},
};

use super::{kv::kv_transition, scalar::scalar_transition, value::value_transition};

pub fn document_state_machine(indentation: i32) -> StateMachine {
    let begin_doc = Rc::new(create_state(false, "begin_doc"));
    let header = Rc::new(create_state(false, "header"));
    let header_end = Rc::new(create_state(false, "header_end"));
    let body = Rc::new(create_state(false, "body"));
    let back = Rc::new(create_state(false, "back"));
    let end = Rc::new(create_state(true, "end"));

    let header_ts = create_word_transition(
        begin_doc.clone(),
        header.clone(),
        "---".to_string(),
        IndentationOperation::BYPASS,
        indentation,
    );

    let header_end_ts = Rc::new(CharTransition::new(
        header.clone(),
        header_end.clone(),
        "\n".to_string(),
        IndentationOperation::RESET,
    ));

    let body_ts = Rc::new(kv_transition(
        header_end.clone(),
        body.clone(),
        indentation,
        IndentationOperation::BYPASS,
    ));

    let back_ts = Rc::new(CharTransition::new(
        body.clone(),
        header_end.clone(),
        "\n".to_string(),
        IndentationOperation::RESET,
    ));

    let end_ts = create_word_transition(
        header_end.clone(),
        end.clone(),
        "---".to_string(),
        IndentationOperation::BYPASS,
        indentation,
    );

    StateMachineBuilder::new(begin_doc.clone(), " ", indentation)
        .add_states(vec![header, header_end, body, back, end])
        .add_transitions(vec![header_ts, header_end_ts, body_ts, back_ts, end_ts])
        .build()
}

#[cfg(test)]
mod tests {
    use super::document_state_machine;

    #[test]
    fn test_document_state_machine_recognize_kv_doc() {
        let val = "---
test:test
---";
        let machine = document_state_machine(0);
        let (result, offset) = machine.validate(val.to_string());
        assert_eq!(result, true);
        assert_eq!(val.len(), offset);
    }

    #[test]
    fn test_document_state_machine_recognize_kv_complex_doc() {
        let val = "---
zob:test
test:
 zob:
  -test
---";
        let machine = document_state_machine(0);
        let (result, offset) = machine.validate(val.to_string());
        assert_eq!(result, true);
        assert_eq!(val.len(), offset);
    }
}
