use r2d2;
use diesel;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

use dotenv::dotenv;
use std::env;
use std::ops::Deref;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

pub type Pool = r2d2::Pool<ConnectionManager<diesel::pg::PgConnection>>;

lazy_static! {
    static ref DB_URL: String = {
        dotenv().ok();
        env::var("DATABASE_URL").unwrap()
    };
}

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(DB_URL.as_str());
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub struct DB(r2d2::PooledConnection<ConnectionManager<diesel::pg::PgConnection>>);

impl Deref for DB {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DB, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DB(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}
