use super::schema::*;
use serde::Serialize;
use serde_json::{to_value, Value};

pub mod attendance;
pub mod carpool;
pub mod event;
pub mod file;
pub mod link;
pub mod song;
pub mod user;

#[derive(Debug, Queryable, Identifiable, Associations, Serialize)]
#[table_name = "users"]
pub struct User {
    #[primary_key] pub id: i32,
    pub email: String,
    pub pass_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub section: String,
    pub phone_number: String,
}

#[derive(Insertable, Deserialize, FromForm)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub pass_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
}

#[derive(Debug, Queryable, Identifiable, Associations, Serialize)]
#[table_name = "events"]
pub struct Event {
    #[primary_key] pub id: i32,
    pub title: String,
    pub location: String,
    pub category: String,
    pub description: Option<String>,
    pub start_time: i32,
    pub end_time: i32,
    pub performance_time: Option<i32>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "events"]
pub struct NewEvent {
    pub title: String,
    pub location: String,
    pub category: String,
    pub description: Option<String>,
    pub start_time: i32,
    pub end_time: i32,
    pub performance_time: Option<i32>,
}

#[derive(Debug, Queryable, Identifiable, Associations, Serialize)]
#[belongs_to(Event)]
#[belongs_to(User)]
#[table_name = "attendances"]
pub struct Attendance {
    #[primary_key] pub id: i32,
    pub event_id: i32,
    pub user_id: i32,
    pub should_attend: bool,
    pub did_attend: bool,
    pub minutes_late: i32,
    pub confirmed: bool,
}

#[derive(Insertable, Deserialize)]
#[table_name = "attendances"]
pub struct NewAttendance {
    pub event_id: i32,
    pub user_id: i32,
}

#[derive(Debug, Queryable, Identifiable, Associations, Serialize)]
#[belongs_to(Event)]
#[belongs_to(User)]
#[table_name = "carpools"]
pub struct Carpool {
    #[primary_key] pub id: i32,
    pub event_id: i32,
    pub user_id: i32,
    pub is_driver: bool,
    pub driver_id: Option<i32>,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "carpools"]
pub struct NewCarpool {
    pub event_id: i32,
    pub user_id: i32,
    pub is_driver: bool,
    pub driver_id: Option<i32>,
}

#[derive(Debug, Queryable, Identifiable, Associations, Serialize)]
#[table_name = "songs"]
pub struct Song {
    #[primary_key] pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub key: String,
    pub starting_pitch: String,
    pub this_semester: bool,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "songs"]
pub struct NewSong {
    pub name: String,
    pub description: Option<String>,
    pub key: String,
    pub starting_pitch: String,
    pub this_semester: bool,
}

#[derive(Debug, Serialize)]
pub struct SongData {
    pub song: Song,
    pub sheets: Vec<File>,
    pub midis: Vec<File>,
    pub performance_links: Vec<Link>,
    pub other_links: Vec<Link>,
}

#[derive(Debug, Queryable, Identifiable, Associations, Serialize)]
#[belongs_to(Song)]
#[table_name = "files"]
pub struct File {
    #[primary_key] pub id: i32,
    pub song_id: i32,
    pub path: String,
    pub name: String,
    pub is_sheet: bool,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "files"]
pub struct NewFile {
    pub song_id: i32,
    pub path: String,
    pub name: String,
    pub is_sheet: bool,
}

#[derive(Debug, Queryable, Identifiable, Associations, Serialize)]
#[belongs_to(Song)]
#[table_name = "links"]
pub struct Link {
    #[primary_key] pub id: i32,
    pub song_id: i32,
    pub link: String,
    pub name: String,
    pub is_performance: bool,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "links"]
pub struct NewLink {
    pub song_id: i32,
    pub link: String,
    pub name: String,
    pub is_performance: bool,
}

pub trait PublicJson {
    fn public_json(&self) -> Value where Self: Serialize {
        to_value(self).unwrap()
    }
}
