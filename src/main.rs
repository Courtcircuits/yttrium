use tracing::{debug, info};
use yaml::kv::kv_state_machine;

pub mod grammar;
pub mod yaml;

fn main() {
    tracing_subscriber::fmt::init();
    debug!("starting parsing");

    let val = "main:
 te:aez
 zib:zae";

    let machine = kv_state_machine(0);

    let (result, offset) = machine.validate_from(val.to_string(), 0, 0);
    info!("offset is : {}, {}", offset, val.len());
    if result {
        println!("valid");
    } else {
        println!("non valid");
    }
}
