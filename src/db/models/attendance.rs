use diesel;
use db::schema;
use db::pool::DB;
use db::models::*;
use diesel::prelude::*;
use std::collections::HashMap;
use db::schema::attendances::dsl::*;
use db::schema::events::dsl::category;
use server::state_models::AttendanceForm;
use rocket::request::{FromForm, FormItems};
use db::schema::users::dsl::{first_name, last_name};

macro_rules! get_form_val {
    ($e:expr) => (match $e {
        Some(t) => t,
        None => return Err(()),
    })
}

impl Attendance {
    pub fn load(given_attendance_id: i32, conn: &DB) -> Result<Attendance, String> {
        attendances
            .filter(id.eq(given_attendance_id))
            .first::<Attendance>(&**conn)
            .optional()
            .expect("error loading attendance")
            .ok_or(format!("attendance with id {} not found", given_attendance_id))
    }

    pub fn load_for_event(given_event_id: i32, conn: &DB) -> Result<(Event, Vec<(Attendance, User)>), String> {
        let event = Event::load(given_event_id, conn)?;
        let attendance_data = schema::attendances::table
            .inner_join(schema::users::table)
            .order(first_name) // TODO: Which way is this supposed to go (first or last first)?
            .order(last_name)
            .filter(event_id.eq(&given_event_id))
            .load::<(Attendance, User)>(&**conn)
            .expect("error loading attendance");

        Ok((event, attendance_data))
    }

    pub fn load_for_event_separate_by_section(given_event_id: i32, conn: &DB) -> Result<(Event, [Vec<(Attendance, User)>; 4]), String> {
        let (event, pairs) = Attendance::load_for_event(given_event_id, conn)?;
        let mut sorted = [Vec::new(), Vec::new(), Vec::new(), Vec::new()]; // TODO: figure out [T; n] notation here
        for pair in pairs {
            match pair.1.section.to_lowercase().as_str() {
                "tenor 1" => sorted[0].push(pair),
                "tenor 2" => sorted[1].push(pair),
                "baritone" => sorted[2].push(pair),
                "bass" => sorted[3].push(pair),
                _bad_section => panic!("{:?} is not a real section"),
            }
        }

        Ok((event, sorted))
    }

    pub fn load_for_user_at_event(conn: &DB, given_user_id: i32, given_event_id: i32) -> Attendance {
        attendances
            .filter(event_id.eq(&given_event_id))
            .filter(user_id.eq(&given_user_id))
            .first::<Attendance>(&**conn)
            .expect("error loading attendance")
    }

    pub fn load_for_user_at_all_events(given_user_id: i32, conn: &DB) -> Vec<(Attendance, Event)> {
        schema::attendances::table
            .inner_join(schema::events::table)
            .filter(user_id.eq(&given_user_id))
            .load::<(Attendance, Event)>(&**conn)
            .expect("error loading event")
    }

    pub fn load_for_user_at_all_events_of_type(given_user_id: i32, event_type: &str, conn: &DB) -> Vec<(Attendance, Event)> {
        schema::attendances::table
            .inner_join(schema::events::table)
            .filter(user_id.eq(&given_user_id))
            .filter(category.eq(event_type))
            .load::<(Attendance, Event)>(&**conn)
            .expect("error loading event")
    }

    pub fn create_for_new_user(given_user_id: i32, conn: &DB) {
        let new_attendances = Event::load_all(conn)
                                  .iter()
                                  .map(|e| NewAttendance {
                                               user_id: given_user_id,
                                               event_id: e.id,
                                           })
                                  .collect::<Vec<NewAttendance>>();

        diesel::insert(&new_attendances)
            .into(attendances)
            .execute(&**conn)
            .expect("error adding new attendances");
    }

    pub fn create_for_new_event(given_event_id: i32, conn: &DB) {
        let all_users = User::load_all(conn);
        let new_attendances = all_users
                                  .iter()
                                  .map(|u| NewAttendance {
                                               user_id: u.id,
                                               event_id: given_event_id
                                           })
                                  .collect::<Vec<NewAttendance>>();

        diesel::insert(&new_attendances)
            .into(attendances)
            .execute(&**conn)
            .expect("error adding new attendances");
    }

    pub fn update(given_attendance_id: i32, attendance_form: &AttendanceForm, conn: &DB) -> bool {
        let updated = diesel::update(attendances.find(given_attendance_id))
            .set(attendance_form)
            .get_result::<Attendance>(&**conn);
        println!("{:?}", updated);

        updated.is_ok()
    }
}

impl PublicJson for Attendance {}

impl<'f> FromForm<'f> for AttendanceForm {
    type Error = ();

    fn from_form(items: &mut FormItems<'f>, _strict: bool) -> Result<AttendanceForm, ()> {
        let map = items.map(|(k, v)| (k.as_str(), v.url_decode().unwrap())).collect::<HashMap<&str, String>>();
        println!("form sent contained {:?}", map);
        Ok(AttendanceForm {
            should_attend: get_form_val!(map.get("should_attend")).parse().unwrap(),
            did_attend: get_form_val!(map.get("did_attend")).parse().unwrap(),
            minutes_late: get_form_val!(map.get("minutes_late")).parse().unwrap(),
            confirmed: get_form_val!(map.get("confirmed")).parse().unwrap(),
        })
    }
}
