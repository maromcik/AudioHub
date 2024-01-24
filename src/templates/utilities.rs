use chrono::{DateTime, Utc};

pub fn format_date(timestamp: &DateTime<Utc>) -> String {
    timestamp.format("%d.%m.%Y").to_string()
}

pub fn format_position(position: &f64) -> String {
    let seconds = (position % 60f64).round();
    let minutes = ((position / 60f64) % 60f64).floor();
    let hours = ((position / 60f64) / 60f64).floor();
    format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds)
}

pub fn display_optional(value: &Option<String>) -> String {
    value.to_owned().unwrap_or(String::from(""))
}

pub fn as_integer(number: &i16) -> i16 {
    number.to_owned()
}

pub fn get_percentage(part: &f64, whole: &f64) -> i64 {
    let fraction = part / whole;
    (fraction * 100.0).floor() as i64
}