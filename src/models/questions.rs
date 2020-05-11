use diesel::prelude::*;
use diesel::dsl::Select;
use diesel::pg::PgConnection;

use crate::db::schema::questions::dsl::*;

#[derive(Debug, Queryable)]
pub struct Question {
  pub id: i32,
  pub number: i32,
  pub text: String,
}

type AllColumns = (
    id,
    number,
    text,
);

pub type All = Select<questions, AllColumns>;

pub fn all() -> All {
    questions.select(questions::all_columns())
}

pub fn load_all(conn: &PgConnection, limit: Option<i64>) -> QueryResult<Vec<Question>> {
    if let Some(lim) = limit {
        all().limit(lim).load::<Question>(conn)
    } else{
        all().load::<Question>(conn)
    }
}
