use diesel::prelude::*;
use diesel::dsl::{Filter, Select, Eq};
use diesel::pg::{Pg, PgConnection};
use diesel::deserialize::Queryable;

use crate::db::schema_manual::question_answers::SqlType;
use crate::db::schema_manual::question_answers::dsl::*;

pub struct QuestionAnswer {
    pub question_id: u32,
    pub question_number: u32,
    pub question_text: String,
    pub question_page: u32,
    pub answer_id: u32,
    pub answer_text: String,
    pub answer_score_x: u32,
    pub answer_score_y: u32,
}

impl Queryable<SqlType, Pg> for QuestionAnswer {
    type Row = (i32, i32, String, i32, i32, String, i32, i32);

    fn build(row: Self::Row) -> Self {
        QuestionAnswer {
            question_id: row.0 as u32,
            question_number: row.1 as u32,
            question_text: row.2,
            question_page: row.3 as u32,
            answer_id: row.4 as u32,
            answer_text: row.5,
            answer_score_x: row.6 as u32,
            answer_score_y: row.7 as u32,
        }
    }
}

type AllColumns = (
    question_id,
    question_number,
    question_text,
    question_page,
    answer_id,
    answer_text,
    answer_score_x,
    answer_score_y,
);

pub type All = Select<question_answers, AllColumns>;

pub fn all() -> All {
    question_answers.select(question_answers::all_columns())
}

pub fn page_of_questions(n: u32) -> Filter<All, Eq<question_page, i32>> {
    all().filter(question_page.eq(n as i32))
}

pub fn load_page(conn: &PgConnection, n: u32) -> QueryResult<Vec<QuestionAnswer>> {
    page_of_questions(n).load::<QuestionAnswer>(conn)
}

pub fn load_all(conn: &PgConnection) -> QueryResult<Vec<QuestionAnswer>> {
    all().load::<QuestionAnswer>(conn)
}
