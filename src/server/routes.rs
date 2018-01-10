use tera::Tera;
use std::fs::File;
use serde_json::Value;
use std::path::{PathBuf, Path};
// use io::config::Config;

use db;
use db::models::*;
use super::state_models::*;
use io::pages::init_page_handler;

use rocket::{State};
use rocket::request::Form;
use rocket::request::FlashMessage;
use rocket::http::{Cookie, Cookies, Status};
use rocket::response::content::{Html, Json};
use rocket::response::{Flash, Redirect, Failure, NamedFile};

#[get("/")]
fn blank() -> Redirect {
    Redirect::permanent("/index")
}

#[get("/index", rank = 2)]
fn not_logged_in() -> Redirect {
    Redirect::to("/login")
}

#[get("/index")]
fn home(user: LoggedInUser, handler: State<Tera>) -> Html<String> {
    Html(handler.render("index.html", &json!({"email": user.email})).unwrap())
}

#[get("/favicon.ico")]
fn icon() -> File {
    File::open("/static/favicon.ico").unwrap()
}

#[get("/login")]
fn logged_in(_user: LoggedInUser) -> Redirect {
    Redirect::to("/index")
}

#[get("/login", rank = 2)]
fn login(flash: Option<FlashMessage>, handler: State<Tera>) -> Html<String> {
    let vals = match flash {
        Some(msg) => json!({msg.name(): msg.msg()}),
        None => json!({}),
    };
    Html(handler.render("login.html", &vals).unwrap())
}

#[post("/login", data = "<login_data>")]
fn submit_login(login_data: Form<LoginData>, mut cookies: Cookies, conn: db::DB) -> Flash<Redirect> {
    let login_data = login_data.into_inner();
    match User::load(&login_data.email, &conn) {
        Ok(User { ref pass_hash, .. } ) if &login_data.pass_hash == pass_hash => {
            cookies.add(Cookie::new("email", login_data.email));
            Flash::success(Redirect::to("/index"), "Correct credentials.")
        }
        _ => Flash::error(
                 Redirect::to("/login"),
                 "The email/password combination did not match any users in our database.",
             ),
    }
}

#[get("/register", rank = 2)]
fn register(flash: Option<FlashMessage>, handler: State<Tera>) -> Html<String> {
    let vals = match flash {
        Some(msg) => json!({msg.name(): msg.msg()}),
        None => json!({}),
    };
    Html(handler.render("login.html", &vals).unwrap())
}

#[post("/register", data = "<new_user_data>")]
fn submit_register(new_user_data: Form<db::NewUser>, conn: db::DB) -> Flash<Redirect> {
    match User::create(new_user_data.into_inner(), &conn) {
         Ok(()) => Flash::success(
                       Redirect::to("/login"),
                       "Your account was successfully created!",
                   ),
         Err(_) => Flash::error(
                       Redirect::to("/register"),
                       "An account with the given email already exists in our database.",
                   ),
    }
}

#[get("/events")]
fn events_unspecified(_user: LoggedInUser) -> Redirect {
    Redirect::permanent("/events/all")
}

#[get("/events/<event_type>")]
fn events(event_type: String, user: LoggedInUser, handler: State<Tera>, conn: db::DB) -> Result<Html<String>, Failure> {
    let user_id = user.get_model(&conn).id;
    let events_of_type: Vec<(Attendance, Event)> = match event_type.to_lowercase().as_str() {
        "all" => Attendance::load_for_user_at_all_events(user_id, &conn),
        t if Event::valid_type(t) => Attendance::load_for_user_at_all_events_of_type(user_id, t, &conn),
        _ => return Err(Failure(Status::NotFound)),
    };

    let vals = json!({
        "email": user.email,
        "event_type": event_type,
        "attendance_event_pairs": events_of_type
                                      .iter()
                                      .map(|pair| json!({
                                          "event": pair.1.public_json(),
                                          "attendance": pair.0.public_json(),
                                      }))
                                      .collect::<Vec<Value>>(),
    });
    println!("{:?}", vals);
    Ok(Html(handler.render("events.html", &vals).unwrap()))
}

#[get("/event/<event_id>")]
fn single_event(event_id: i32, _user: LoggedInUser, handler: State<Tera>, conn: db::DB) -> Result<Html<String>, Failure> {
    if let Ok(event) = Event::load(event_id, &conn) {
        Ok(Html(handler.render("single_event.html", &json!({"event": event})).unwrap()))
    } else {
        Err(Failure(Status::NotFound))
    }
}

#[post("/events", data = "<new_event_data>")]
fn submit_event(_user: LoggedInUser, new_event_data: Form<db::NewEvent>, conn: db::DB) -> Flash<Redirect> {
    let id = Event::add_event(new_event_data.into_inner(), &conn);
    Flash::success(Redirect::to("/events/all"), id.to_string())
}

#[get("/attendance/<event_id>")]
fn attendance(event_id: i32, _user: LoggedInUser, handler: State<Tera>, conn: db::DB) -> Result<Html<String>, Failure> {
    if let Ok((event, attendance_user_pairs)) = Attendance::load_for_event(event_id, &conn) {
        let vals = json!({
            "event": event,
            "pairs": attendance_user_pairs.iter().map(|&(ref a, ref u)| json!({
                "attendance": json!({
                    "id": a.id,
                    "should_attend": if a.should_attend { Some(1) } else { None },
                    "did_attend": if a.did_attend { Some(1) } else { None },
                    "minutes_late": a.minutes_late,
                    "confirmed": if a.confirmed { Some(1) } else { None },
                }),
                "user": u.public_json(),
            })).collect::<Vec<Value>>()
        });
        println!("How about: {}", vals);
        Ok(Html(handler.render("attendance.html", &vals).unwrap()))
    } else {
        Err(Failure(Status::NotFound))
    }
}

#[post("/attendance/<attendance_id>", data = "<attendance>")]
fn update_attendance(attendance_id: i32, attendance: Form<AttendanceForm>, _user: LoggedInUser, conn: db::DB) -> Json<String> {
    Json(format!("{}", json!({"success": Attendance::update(attendance_id, &attendance.into_inner(), &conn)})))
}

#[get("/repertoire")]
fn repertoire(user: LoggedInUser, handler: State<Tera>, conn: db::DB) -> Html<String> {
    let (current_songs, other_songs) = Song::load_all_separate_this_semester(&conn);
    Html(handler.render("repertoire.html", &json!({
        "email": user.email,
        "current_songs": current_songs,
        "other_songs": other_songs,
    })).unwrap())
}

#[get("/repertoire/<song_id>")]
fn single_song(song_id: i32, _user: LoggedInUser, handler: State<Tera>, conn: db::DB) -> Result<Html<String>, Failure> {
    if let Ok(song_data) = Song::load_with_data(song_id, &conn) {
        Ok(Html(handler.render("single_song.html", &song_data).unwrap()))
    } else {
        Err(Failure(Status::NotFound))
    }
}

#[post("/upload_file", data = "<data>")]
fn upload_file(mut data: Form<NewFileData>, conn: db::DB) -> Json<String> {
    let new_file_data = data.into_inner();
    Json((match new_file_data.file.stream_to_file(&format!("/static/uploaded/{}", new_file_data.path)) {
        Ok(_) => {
            if let Ok(_existing_file) = db::File::load_for_path(&new_file_data.path, &conn) {
                json!({"error": format!("A file already exists with the filename \"{}\".", new_file_data.path)})
            } else {
                db::File::create(NewFile {
                    song_id: new_file_data.song_id,
                    path: new_file_data.path,
                    name: new_file_data.name,
                    is_sheet: new_file_data.is_sheet,
                }, &conn);
                json!({"success": "The file was successfully added."})
            }
        },
        Err(msg) => json!({"error": format!("The following error occurred while uploading the file: {}", msg)})
    }).to_string())
}

#[post("/logout")]
fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove(Cookie::named("email"));
    Redirect::to("/login")
}

#[get("/styles/<file..>")]
fn styles(file: PathBuf) -> Result<NamedFile, Failure> {
    NamedFile::open(Path::new("static/styles/").join(file)).map_err(|_| Failure(Status::NotFound))
}

#[get("/uploaded/<file..>")]
fn uploaded(file: PathBuf) -> Result<NamedFile, Failure> {
    NamedFile::open(Path::new("static/uploaded/").join(file)).map_err(|_| Failure(Status::NotFound))
}

#[get("/scripts/<file..>")]
fn scripts(file: PathBuf) -> Result<NamedFile, Failure> {
    NamedFile::open(Path::new("scripts/").join(file)).map_err(|_| Failure(Status::NotFound))
}

pub fn start() {
    ::rocket::ignite()
        .manage(db::init_pool())
        // .manage(Config::load())
        .manage(init_page_handler())
        .mount(
            "/",
            routes![
                blank, home, icon,
                login, submit_login, logged_in, not_logged_in, logout,
                register, submit_register,
                events, events_unspecified, single_event, submit_event,
                repertoire, single_song,
                styles, uploaded, scripts,
                attendance, update_attendance,
            ],
        )
        .launch();
}
