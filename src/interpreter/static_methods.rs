use crate::object;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn clock() -> object::Object {
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64;
    obj_number!(t)
}
