use chrono::{Local, Utc};
use hl7_parser::timestamps::TimeStamp;

pub fn generate_timestamp(utc: bool) -> String {
    if utc {
        TimeStamp::from(Utc::now()).to_string()
    } else {
        TimeStamp::from(Local::now()).to_string()
    }
}

pub fn generate_control_id(_: ()) -> String {
    use rand::distributions::{Alphanumeric, DistString};
    Alphanumeric.sample_string(&mut rand::thread_rng(), 20)
}

