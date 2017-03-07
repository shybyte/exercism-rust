extern crate chrono;
use chrono::*;

pub fn after(birthday: DateTime<UTC>) -> DateTime<UTC> {
    birthday + Duration::seconds(1_000_000_000)
}
