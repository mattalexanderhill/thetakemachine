#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate rand;
extern crate dotenv;

use rocket::Rocket;
use rocket_contrib::templates::Template;    
use rocket_contrib::serve::StaticFiles;

mod db;
mod models;
mod routes;
mod config;

pub fn wgafoy() -> Rocket {
    dotenv::dotenv().ok();
    rocket::custom(config::from_env())
        .mount("/", routes![
            routes::get_index,
            routes::get_favicon,
            routes::quiz::get_index,
            routes::quiz::get_questions,
            routes::quiz::get_chart,
            routes::quiz::get_results,
            routes::quiz::post_questions,
        ])
        .register(catchers![
            routes::catch::not_found,
            routes::catch::server_error,
        ])
        .mount("/assets", StaticFiles::from("static"))
        .attach(db::Conn::fairing())
        .attach(Template::fairing())
}

#[cfg(test)]
mod tests;
