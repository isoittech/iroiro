extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod otameshi;
use crate::otameshi::event_listener;
use crate::otameshi::json_serialize;
use crate::otameshi::key_input_listener;
use crate::otameshi::key_input_listener2;
use crate::otameshi::logger;
use crate::otameshi::message_box;
use crate::otameshi::thread;
use crate::otameshi::thread2_join;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::logaaa();

    // println!("----------------------------");
    // json_serialize::siriarudaaaaa()?;

    // println!("----------------------------");
    // otameshi_message_box::msg_box_a()?;

    // println!("----------------------------");
    // thread::sureddo();

    // println!("----------------------------");
    // thread2_join::jyoiiiin();

    // println!("----------------------------");
    // event_listener::ibento_risunaaa();

    // println!("----------------------------");
    // key_input_listener::kii_risunaa()?;

    println!("----------------------------");
    key_input_listener2::kii_risunaa2()?;

    Ok(())
}
