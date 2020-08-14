extern crate cpython;
extern crate tch;
extern crate grpcio;
extern crate futures;
#[macro_use] extern crate log;
#[macro_use] extern crate thiserror;
mod environment;
mod communicator_objects;
mod communicator;

use environment::UnityEnvironment;
use environment::Environment;
use std::thread::sleep_ms;

fn main() {
    let env = UnityEnvironment::default();
    env.init();
    for i in 1..10 {
        println!("Sending step {}", i);
        env.step();
        println!("Sent step {}", i);
        sleep_ms(10000);
    }
}
