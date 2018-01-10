#![recursion_limit = "128"]
#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen, diesel_codegen)]
extern crate chrono;
extern crate crypto;
#[macro_use]
extern crate diesel;
// extern crate diesel_derives;
extern crate diesel_infer_schema;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate open;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate tera;
extern crate toml;
extern crate rocket;

pub mod io;
pub mod db;
pub mod server;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
