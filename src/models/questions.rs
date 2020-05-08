use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::db::schema::questions::dsl::*;

#[derive(Debug, Queryable)]
pub struct Question {
  pub id: u32,
  pub number: u32,
  pub text: String,
}

pub fn count(conn: &PgConnection) -> QueryResult<i64> {
    questions.count().get_result(conn)
}
