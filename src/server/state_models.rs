use db;
use db::schema::attendances;

use rocket::http::Status;
use rocket::{Data, Outcome};
use rocket::request::{FromRequest, Request};

#[derive(FromForm)]
pub struct LoginData {
    pub email: String,
    pub pass_hash: String,
}

#[derive(FromForm)]
pub struct NewFileData {
    pub song_id: i32,
    pub path: String,
    pub name: String,
    pub is_sheet: bool,
    pub file: Data,
}

#[derive(AsChangeset)]
#[table_name = "attendances"]
pub struct AttendanceForm {
    pub should_attend: bool,
    pub did_attend: bool,
    pub minutes_late: i32,
    pub confirmed: bool,
}

pub struct LoggedInUser {
    pub email: String,
}

impl LoggedInUser {
    pub fn get_model(&self, conn: &db::DB) -> db::User {
        db::User::load(&self.email, conn).unwrap()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for LoggedInUser {
    type Error = String;
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, (Status, String), ()> {
        let cookies = request.cookies();
        if let Some(email) = cookies.get("email") {
            Outcome::Success(LoggedInUser {
                email: email.value().to_string(),
            })
        } else {
            Outcome::Forward(())
        }
    }
}
