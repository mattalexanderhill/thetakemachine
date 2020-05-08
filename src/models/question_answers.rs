use diesel::prelude::*;
use diesel::dsl::Select;
use diesel::pg::{Pg, PgConnection};
use diesel::deserialize::Queryable;

use crate::db::schema_manual::question_answers::SqlType;
use crate::db::schema_manual::question_answers::dsl::*;

pub struct QuestionAnswer {
    pub question_id: u32,
    pub question_number: u32,
    pub question_text: String,
    pub answer_id: u32,
    pub answer_text: String,
    pub answer_score_x: Option<u32>,
    pub answer_score_y: Option<u32>,
}

impl Queryable<SqlType, Pg> for QuestionAnswer {
    type Row = (i32, i32, String, i32, String, Option<i32>, Option<i32>);

    fn build(row: Self::Row) -> Self {
        QuestionAnswer {
            question_id: row.0 as u32,
            question_number: row.1 as u32,
            question_text: row.2,
            answer_id: row.3 as u32,
            answer_text: row.4,
            answer_score_x: row.5.map_or(None, |x| Some(x as u32)),
            answer_score_y: row.6.map_or(None, |x| Some(x as u32))
        }
    }
}

type AllColumns = (
    question_id,
    question_number,
    question_text,
    answer_id,
    answer_text,
    answer_score_x,
    answer_score_y,
);

pub type All = Select<question_answers, AllColumns>;

pub fn all() -> All {
    question_answers.select(question_answers::all_columns())
}

pub fn load_all(conn: &PgConnection) -> QueryResult<Vec<QuestionAnswer>> {
    all().load::<QuestionAnswer>(conn)
}
