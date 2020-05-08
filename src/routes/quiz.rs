use itertools::Itertools;
use rocket::request::{Form, FromForm, FormItems};
use rocket::response::Redirect;
use rocket::http::Status;
use rocket_contrib::templates::Template;
use crate::models::question_answers::{QuestionAnswer, load_all};
use crate::db::Conn;

#[derive(Serialize)]
struct IndexContext {
    parent: &'static str,
}

#[get("/quiz")]
pub fn get_index() -> Template {
    let context = IndexContext { parent: "layout" };
    Template::render("quiz/index", &context)
}

#[derive(Serialize, Debug)]
struct QuestionsContextAnswer {
    id: u32,
    text: String,
}

#[derive(Serialize, Debug)]
struct QuestionsContextQuestion {
    id: u32,
    text: String,
    number: u32,
    answers: Vec<QuestionsContextAnswer>,
}

#[derive(Serialize, Debug)]
struct QuestionsContext {
    questions: Vec<QuestionsContextQuestion>,
    parent: &'static str,
}

#[get("/quiz/questions")]
pub fn get_questions(conn: Conn) -> Result<Template, Status> {
    let questions = load_all(&conn).unwrap_or(vec![]);

    if questions.is_empty() {
        return Err(Status::NotFound);
    }

    let mut grouped_questions = Vec::new();

    for (q_id, group) in &questions.into_iter().group_by(|q| q.question_id) {
        let group: Vec<QuestionAnswer> = group.collect();

        grouped_questions.push(QuestionsContextQuestion {
            id: q_id,
            text: group[0].question_text.clone(),
            number: group[0].question_number,
            answers: group
                .into_iter()
                .map(|a| QuestionsContextAnswer {
                    id: a.answer_id,
                    text: a.answer_text,
                })
                .collect(),
        })
    }

    let context = QuestionsContext {
        questions: grouped_questions,
        parent: "layout"
    };

    Ok(Template::render("quiz/questions", &context))
}

#[derive(Debug)]
pub struct QuestionResponse {
    question_id: u32,
    answer_id: u32,
}

#[derive(Debug)]
pub struct QuestionsResponse {
    responses: Vec<QuestionResponse>,
}

impl<'a> FromForm<'a> for QuestionsResponse {
    type Error = &'static str;

    fn from_form(items: &mut FormItems<'a>, strict: bool) -> Result<QuestionsResponse, &'static str> {
        let mut responses = Vec::new();

        for item in items {
            responses.push(QuestionResponse {
                question_id: item.key
                    .as_str()
                    .trim_start_matches("q_")
                    .parse::<u32>()
                    .map_err(|_| "invalid form response")?,
                answer_id: item.value
                    .as_str()
                    .parse::<u32>()
                    .map_err(|_| "invalid form response")?,
            });
        }

        Ok(QuestionsResponse { responses })
    }
}

#[post("/quiz/questions", data="<answers>")]
pub fn post_questions(
    conn: Conn,
    answers: Form<QuestionsResponse>
) -> Redirect {
    println!("{:?}", answers);

    Redirect::to("/quiz")
}
