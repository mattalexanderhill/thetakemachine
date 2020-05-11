use diesel::prelude::*;
use diesel::dsl::{Eq, Select, Filter};
use diesel::pg::{Pg, PgConnection};
use diesel::deserialize::Queryable;

use crate::models::answers::Answer;

use crate::db::schema_manual::response_scores;
use crate::db::schema_manual::response_scores::SqlType;

#[derive(Debug)]
pub struct ResponseScore {
  pub session_id: String,
  pub question_id: i32,
  pub answer: Answer,
  pub x: Option<i32>,
  pub y: Option<i32>,
}

type AllColumns = (
    response_scores::id,
    response_scores::session_id,
    response_scores::question_id,
    response_scores::answer_id,
    response_scores::x,
    response_scores::y,
);

impl Queryable<SqlType, Pg> for ResponseScore {
    type Row = (i64, String, i32, i32, Option<i32>, Option<i32>);

    fn build(row: Self::Row) -> Self {
        ResponseScore {
            session_id: row.1,
            question_id: row.2,
            answer: Answer::from_id(row.3),
            x: row.4,
            y: row.5,
        }
    }
}

pub type All = Select<response_scores::table, AllColumns>;
pub type FromSession<'a> = Filter<All, Eq<response_scores::session_id, &'a str>>;

pub fn all() -> All {
    use crate::db::schema_manual::response_scores::dsl::*;

    response_scores.select(response_scores::all_columns())
}

pub fn from_session(session: &str) -> FromSession {
    use crate::db::schema_manual::response_scores::dsl::*;

    all().filter(session_id.eq(session))
}

pub fn load_from_session(conn: &PgConnection, session: &str) -> QueryResult<Vec<ResponseScore>> {
    from_session(session).load::<ResponseScore>(conn)
}
