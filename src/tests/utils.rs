extern crate dotenv;

use rocket::local::Client;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use once_cell::sync::OnceCell;
use std::env;
use crate::wgafoy;


pub fn test_client() -> &'static Client {
    static INSTANCE: OnceCell<Client> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let wgafoy = wgafoy();
        Client::new(wgafoy).expect("valid rocket instance")
    })
}

pub fn test_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
