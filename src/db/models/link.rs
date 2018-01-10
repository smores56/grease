use diesel;
use db::pool::DB;
use db::models::*;
use diesel::prelude::*;
use db::schema::links::dsl::*;

impl Link {
    pub fn load(given_link_id: i32, conn: &DB) -> Result<Link, String> {
        links
            .filter(id.eq(given_link_id))
            .first::<Link>(&**conn)
            .optional()
            .expect("error loading link")
            .ok_or(format!("no link exists with the id {}", given_link_id))
    }

    pub fn load_for_song(given_song_id: i32, conn: &DB) -> Vec<Link> {
        links
            .filter(song_id.eq(given_song_id))
            .order(name)
            .load::<Link>(&**conn)
            .expect("error loading links")
    }

    pub fn load_for_song_sorted(given_song_id: i32, conn: &DB) -> (Vec<Link>, Vec<Link>) {
        let mut performance_links = Vec::new();
        let mut other_links = Vec::new();
        for l in Link::load_for_song(given_song_id, conn) {
            if l.is_performance {
                performance_links.push(l);
            } else {
                other_links.push(l);
            }
        }

        (performance_links, other_links)
    }

    // TODO: figure out what to do with actual link uploading / creation
    pub fn create(new_link: NewLink, conn: &DB) {
        diesel::insert(&new_link)
            .into(links)
            .execute(&**conn)
            .expect("error adding new link");
    }

    pub fn create_multiple(new_links: Vec<NewLink>, conn: &DB) {
        diesel::insert(&new_links)
            .into(links)
            .execute(&**conn)
            .expect("error adding new links");
    }

    pub fn update(given_link_id: i32, updated_link: NewLink, conn: &DB) -> bool {
        diesel::update(links.find(given_link_id))
            .set(&updated_link)
            .get_result::<Link>(&**conn)
            .is_ok()
    }
}

impl PublicJson for Link {}
