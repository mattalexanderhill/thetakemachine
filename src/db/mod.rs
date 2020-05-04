use rocket_contrib::databases::diesel;

#[database("diesel_postgres_pool")]
pub struct Conn(diesel::PgConnection);

pub mod schema;
pub mod schema_manual;
