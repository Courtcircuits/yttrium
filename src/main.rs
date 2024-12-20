use tracing::info;
use yaml::document::document_state_machine;
pub mod grammar;
pub mod yaml;

fn main() {
    tracing_subscriber::fmt::init();

    let val = "---
zob:test
salut:
 zob:
  zizi:
   -test

   zob:
 test:test
salut:
 test:test
list:
 -zob
 -zizi
---";

    let machine = document_state_machine(0);

    let (result, offset) = machine.validate_from(val.to_string(), 0, 0);
    info!("offset is : {}, {}", offset, val.len());
    if result {
        println!("valid");
    } else {
        println!("non valid");
    }
}
