use db::models::Song;
use chrono::NaiveDateTime;
use tera::{self, Tera, Value};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn init_page_handler() -> Tera {
    let mut tera = compile_templates!("templates/*.html");
    tera.register_filter("date", date_from_epoch);
    tera.register_filter("calendar_date", calendar_date_from_epoch);
    tera.register_filter("time", time_from_epoch);
    tera.register_global_function("current_time", Box::new(current_time));
    tera.register_global_function("keys", Box::new(keys));

    tera
}

pub fn date_from_epoch(v: Value, _map: HashMap<String, Value>) -> tera::Result<Value> {
    let date = NaiveDateTime::from_timestamp(v.as_i64().unwrap(), 0).date();
    Ok(Value::String(date.format("%m-%d-%Y").to_string()))
}

pub fn calendar_date_from_epoch(v: Value, _map: HashMap<String, Value>) -> tera::Result<Value> {
    let date = NaiveDateTime::from_timestamp(v.as_i64().unwrap(), 0).date();
    Ok(Value::String(date.format("%A, %-d %B, %C%y").to_string()))
}

pub fn time_from_epoch(v: Value, _map: HashMap<String, Value>) -> tera::Result<Value> {
    let time = NaiveDateTime::from_timestamp(v.as_i64().unwrap(), 0).time();
    Ok(Value::String(time.format("%-I:%M %p").to_string()))
}

fn current_time(_: HashMap<String, Value>) -> tera::Result<Value> {
    Ok(Value::Number(SystemTime::now()
                         .duration_since(UNIX_EPOCH)
                         .expect("time went backwards")
                         .as_secs()
                         .into()))
}

fn keys(_: HashMap<String, Value>) -> tera::Result<Value> {
    Ok(Song::KEYS[..].into())
}
