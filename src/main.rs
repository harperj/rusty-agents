extern crate cpython;
extern crate futures;
extern crate grpcio;
extern crate tch;
#[macro_use]
extern crate log;
extern crate thiserror;
mod behavior;
mod communicator;
mod communicator_objects;
mod environment;

use environment::Environment;
use environment::UnityEnvironment;
use std::thread::sleep_ms;

fn main() {
    let mut env = UnityEnvironment::default();
    env.init();
    for i in 1..1000 {
        println!("Sending step {}", i);
        env.step().ok();
        println!("Sent step {}", i);
        sleep_ms(1);
    }
}
