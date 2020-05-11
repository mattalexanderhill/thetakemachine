use diesel::prelude::*;
use diesel::insert_into;
use diesel::pg::PgConnection;

use crate::db::schema::responses;

#[derive(Debug, Insertable)]
pub struct Response {
  pub session_id: String,
  pub question_id: i32,
  pub answer_id: i32,
}

pub fn store_responses(conn: &PgConnection, resps: &Vec<Response>) -> QueryResult<usize> {
    insert_into(responses::table).values(resps).execute(conn)
}
