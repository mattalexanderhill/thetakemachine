use diesel::prelude::*;
use diesel::insert_into;
use diesel::dsl::{Eq, Select, Filter};
use diesel::pg::{Pg, PgConnection};
use diesel::deserialize::Queryable;

use crate::db::schema::question_answers;
use crate::db::schema::question_answers::SqlType;

#[derive(Debug, Insertable)]
pub struct QuestionAnswer {
  pub session_id: String,
  pub question_id: i32,
  pub answer_id: i32,
}

type AllColumns = (
    question_answers::id,
    question_answers::session_id,
    question_answers::question_id,
    question_answers::answer_id,
);

impl Queryable<SqlType, Pg> for QuestionAnswer {
    type Row = (i32, String, i32, i32);

    fn build(row: Self::Row) -> Self {
        QuestionAnswer {
            session_id: row.1,
            question_id: row.2 as i32,
            answer_id: row.3 as i32,
        }
    }
}

pub type All = Select<question_answers::table, AllColumns>;
pub type FromSession<'a> = Filter<All, Eq<question_answers::session_id, &'a str>>;

pub fn all() -> All {
    use crate::db::schema::question_answers::dsl::*;

    question_answers.select(question_answers::all_columns())
}

pub fn from_session(session: &str) -> FromSession {
    use crate::db::schema::question_answers::dsl::*;

    all().filter(session_id.eq(session))
}

pub fn load_from_session(conn: &PgConnection, session: &str) -> QueryResult<Vec<QuestionAnswer>> {
    from_session(session).load::<QuestionAnswer>(conn)
}

pub fn store_response(conn: &PgConnection, response: &Vec<QuestionAnswer>) -> QueryResult<usize> {
    insert_into(question_answers::table).values(response).execute(conn)
}
