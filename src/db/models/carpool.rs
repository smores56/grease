use diesel;
use db::schema;
use db::pool::DB;
use db::models::*;
use diesel::prelude::*;
use db::schema::carpools::dsl::*;
use db::schema::users::dsl::{last_name, first_name};

impl Carpool {
    pub fn load(given_carpool_id: i32, conn: &DB) -> Result<Carpool, String> {
        carpools
            .filter(id.eq(given_carpool_id))
            .first::<Carpool>(&**conn)
            .optional()
            .expect("error loading carpool")
            .ok_or(format!("carpool with id {} doesn't exist", given_carpool_id))
    }

    pub fn load_for_event(given_event_id: i32, conn: &DB) -> Result<(Event, Vec<(Carpool, User)>), String> {
        let event = Event::load(given_event_id, conn)?;
        let carpool_user_pairs = schema::carpools::table
            .inner_join(schema::users::table)
            .filter(event_id.eq(given_event_id))
            .order(first_name) // TODO: pick order here
            .order(last_name)
            .load::<(Carpool, User)>(&**conn)
            .expect("error loading carpools");

        Ok((event, carpool_user_pairs))
    }

    // TODO: figure out what to do with actual carpool uploading / creation
    pub fn create(new_carpool: NewCarpool, conn: &DB) {
        diesel::insert(&new_carpool)
            .into(carpools)
            .execute(&**conn)
            .expect("error adding new carpool");
    }

    pub fn create_multiple(new_carpools: Vec<NewCarpool>, conn: &DB) {
        diesel::insert(&new_carpools)
            .into(carpools)
            .execute(&**conn)
            .expect("error adding new carpools");
    }

    pub fn update(given_carpool_id: i32, updated_carpool: NewCarpool, conn: &DB) -> bool {
        diesel::update(carpools.find(given_carpool_id))
            .set(&updated_carpool)
            .get_result::<Carpool>(&**conn)
            .is_ok()
    }
}

impl PublicJson for Carpool {}
