use yaml::{kv::kv_state_machine, sequence::sequence_state_machine, value::value_state_machine};

pub mod grammar;
pub mod yaml;

fn main() {
    let val = "test:
 -test
 -ewq
 -adssca";
    let mut machine = kv_state_machine(0);

    let (result, offset) = machine.validate(val.to_string());
    if result {
        println!("valid");
    } else {
        println!("non valid");
    }
}
