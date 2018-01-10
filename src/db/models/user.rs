use diesel;
use db::pool::DB;
use serde_json::Value;
use diesel::prelude::*;
use db::schema::users::dsl::*;
use db::models::{User, NewUser, PublicJson, Attendance};

impl User {
    pub fn load(given_email: &str, conn: &DB) -> Result<User, String> {
        users
            .filter(email.eq(given_email))
            .first(&**conn)
            .optional()
            .expect("error loading user")
            .ok_or(format!("no user exists with the email {}", given_email))
    }

    pub fn load_all(conn: &DB) -> Vec<User> {
        users
            .order(first_name)
            .order(last_name)
            .load::<User>(&**conn)
            .expect("error loading all users")
    }

    pub fn create(new_user: NewUser, conn: &DB) -> Result<(), String> {
        match Self::load(&new_user.email, conn) {
            Ok(user) => Err(format!("A user with the email {} already exists.", user.email)),
            Err(_) => {
                diesel::insert(&new_user)
                    .into(users)
                    .execute(&**conn)
                    .expect("error adding user");

                let new_user_id = User::load(&new_user.email, conn).unwrap().id;
                Attendance::create_for_new_user(new_user_id, conn);

                Ok(())
            }
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

impl PublicJson for User {
    fn public_json(&self) -> Value {
        json!({
            "id": self.id,
            "email": self.email,
            "name": self.full_name(),
            "section": self.section,
            "phone_number": self.phone_number,
        })
    }
}
