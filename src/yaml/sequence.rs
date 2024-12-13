use std::rc::Rc;

use crate::grammar::{
    state::{create_state, State},
    state_machine::{StateMachine, StateMachineBuilder},
    transition::{CharTransition, EpsilonTransition, GroupTransition, IndentationOperation},
};

use super::scalar::scalar_transition;

pub fn sequence_state_machine(indentation: i32) -> StateMachine {
    let begin = Rc::new(create_state(false, "start"));
    let tick = Rc::new(create_state(false, "tick"));
    let val = Rc::new(create_state(true, "val"));
    let next = Rc::new(create_state(false, "next"));

    let b_t = CharTransition::new(
        begin.clone(),
        tick.clone(),
        "-".to_string(),
        IndentationOperation::BYPASS,
    );
    let t_v = scalar_transition(
        tick.clone(),
        val.clone(),
        indentation,
        IndentationOperation::BYPASS,
    );
    let v_n = CharTransition::new(
        val.clone(),
        next.clone(),
        "\n".to_string(),
        IndentationOperation::CONSERVE,
    );
    let n_b = EpsilonTransition::new(next.clone(), begin.clone());
    let automaton = StateMachineBuilder::new(begin, " ", indentation)
        .add_states(vec![tick, val, next])
        .add_transitions(vec![Rc::new(b_t), Rc::new(t_v), Rc::new(v_n), Rc::new(n_b)])
        .build();
    automaton
}

pub fn sequence_transition(
    from: Rc<State>,
    to: Rc<State>,
    indentation: i32,
    operation: IndentationOperation,
) -> GroupTransition {
    GroupTransition::new(from, to, sequence_state_machine(indentation), operation)
}

#[cfg(test)]
mod tests {
    use crate::yaml::sequence::sequence_state_machine;

    #[test]
    fn test_kv_state_machine_recognize_kv() {
        let kv = "-val
-val
-val";
        let machine = sequence_state_machine(0);
        let (result, offset) = machine.validate(kv.to_string());
        assert_eq!(result, true);
        assert_eq!(kv.len(), offset);
    }
}
