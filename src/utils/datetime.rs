use chrono::{DateTime, Utc};

pub fn get_now() -> DateTime<Utc> {
    Utc::now()
}
