use rocket::config::{Config, Environment, Value};
use std::collections::HashMap;
use std::env;

pub fn from_env() -> Config {
    let environment = Environment::active().expect("No environment found");

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT invalid in .env");

    let mut db_config = HashMap::new();
    let mut databases = HashMap::new();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL missing from .env");

    db_config.insert("url", Value::from(database_url));
    databases.insert("diesel_postgres_pool", Value::from(db_config));

    Config::build(environment)
        .environment(environment)
        .port(port)
        .extra("databases", databases)
        .finalize()
        .unwrap()
}
