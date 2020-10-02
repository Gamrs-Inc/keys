use keyboard_query::{DeviceQuery, DeviceState};
use std::io::prelude::*;
use std::io::stdout;

fn main() {
    let device_state = DeviceState::new();
    let mut prev_keys = vec![];

    loop {
        let keys = device_state.get_keys();
        if keys != prev_keys {
            stdout()
                .write(format!("{:?}", keys).replace(" ", "").as_bytes())
                .unwrap();
            stdout().flush().unwrap();
            prev_keys = keys;
        }
    }
}
