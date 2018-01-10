use diesel;
use db::pool::DB;
use db::models::*;
use chrono::NaiveDate;
use diesel::prelude::*;
use std::collections::HashMap;
use db::schema::events::dsl::*;
use serde_json::{to_value, Value};
use rocket::request::{FromForm, FormItems};

macro_rules! get_form_val {
    ($e:expr) => (match $e {
        Some(t) => t,
        None => return Err(()),
    })
}

impl Event {
    const VALID_TYPES: [&'static str; 6] = ["rehearsal", "sectional", "tutti", "volunteer", "ombuds", "other"];

    pub fn load(given_event_id: i32, conn: &DB) -> Result<Event, String> {
        events
            .filter(id.eq(given_event_id))
            .first::<Event>(&**conn)
            .optional()
            .expect("error loading event")
            .ok_or(format!("event with id {} doesn't exist", given_event_id))
    }

    pub fn valid_type(event_type: &str) -> bool {
        Self::VALID_TYPES.iter().find(|&&t| t == event_type).is_some()
    }

    pub fn load_all(conn: &DB) -> Vec<Event> {
        events
        .order(performance_time)
        .load::<Event>(&**conn)
        .expect("error loading events")
    }

    pub fn load_all_of_type(event_type: &str, conn: &DB) -> Vec<Event> {
        events
        .filter(category.eq(event_type))
        .order(performance_time)
        .load::<Event>(&**conn)
        .expect("error loading events")
    }

    pub fn category_value(cat: &str) -> i32 {
        match cat.to_lowercase().as_str() {
            "tutti" => 35,
            "rehearsal" | "sectional" => 10,
            "volunteer" | "ombuds" | "other" => 5,
            _ => 0,
        }
    }

    pub fn add_event(new_event: NewEvent, conn: &DB) -> i32 {
        diesel::insert(&new_event)
            .into(events)
            .execute(&**conn)
            .expect("error adding event");

        let new_event_id = events
            .filter(title.eq(&new_event.title))
            .filter(start_time.eq(&new_event.start_time))
            .first::<Event>(&**conn)
            .expect("error loading event")
            .id;

        Attendance::create_for_new_event(new_event_id, conn);

        new_event_id
    }
}

impl PublicJson for Event {
    fn public_json(&self) -> Value {
        let mut vals = to_value(self).unwrap();
        vals["value"] = Event::category_value(&self.category).into();
        vals
    }
}

impl<'f> FromForm<'f> for NewEvent {
    // In practice, we'd use a more descriptive error type.
    type Error = ();

    fn from_form(items: &mut FormItems<'f>, _strict: bool) -> Result<NewEvent, ()> {
        let map = items.map(|(k, v)| (k.as_str(), v.url_decode().unwrap())).collect::<HashMap<&str, String>>();
        println!("{:?}", map);
        Ok(NewEvent {
            title: get_form_val!(map.get("title")).to_string(),
            location: get_form_val!(map.get("location")).to_string(),
            category: get_form_val!(map.get("category")).to_string(),
            description: match map.get("description") {
                Some(d) if d.len() > 0 => Some(d.to_string()),
                _ => None,
            },
            start_time: time_from_form_fields(get_form_val!(map.get("start_date")), get_form_val!(map.get("start_time"))), // TODO: fix fragile input
            end_time: time_from_form_fields(get_form_val!(map.get("end_date")), get_form_val!(map.get("end_time"))),
            performance_time: {
                let perf_date = match map.get("performance_date") {
                    Some(d) if d.len() > 0 => Some(d.to_string()),
                    _ => None,
                };
                let perf_time = match map.get("performance_time") {
                    Some(t) if t.len() > 0 => Some(t.to_string()),
                    _ => None,
                };
                if perf_time.is_some() && perf_date.is_some() {
                    Some(time_from_form_fields(&perf_date.unwrap(), &perf_time.unwrap()))
                } else {
                    None
                }
            }
        })
    }
}

// TODO: Clean this up to use chrono's parser
fn time_from_form_fields(ymd: &str, hm: &str) -> i32 {
    let ymd = ymd.split('-').collect::<Vec<&str>>();
    let hm = hm.split(':').collect::<Vec<&str>>();
    println!("{:?}, {:?}", ymd, hm);
    NaiveDate::from_ymd(ymd[0].parse().unwrap(), ymd[1].parse().unwrap(), ymd[2].parse().unwrap())
        .and_hms(hm[0].parse().unwrap(), hm[1].parse().unwrap(), 0).timestamp() as i32
}
