use rand::{distributions::Alphanumeric, thread_rng, Rng};

use rocket::request::{Form, FromForm, FormItems};
use rocket::response::Redirect;
use rocket::http::{ContentType, Status};
use rocket::response::content::Content;

use rocket_contrib::templates::Template;

use crate::models::questions::load_all;
use crate::models::answers::Answer;
use crate::models::question_answers::{QuestionAnswer, store_response};
use crate::db::Conn;


const SESSION_ID_LEN: u8 = 6;

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
    id: i32,
    text: String,
}

#[derive(Serialize, Debug)]
struct QuestionsContextQuestion {
    id: i32,
    text: String,
    number: i32,
}

#[derive(Serialize, Debug)]
struct QuestionsContext {
    questions: Vec<QuestionsContextQuestion>,
    answers: Vec<QuestionsContextAnswer>,
    parent: &'static str,
}

#[get("/quiz/questions")]
pub fn get_questions(conn: Conn) -> Result<Template, Status> {
    let questions = load_all(&conn, Some(3)).unwrap_or(vec![]);

    if questions.is_empty() {
        return Err(Status::NotFound);
    }

    let context = QuestionsContext {
        questions: questions
            .iter()
            .map(|q| QuestionsContextQuestion {
                id: q.id,
                text: q.text.to_owned(),
                number: q.number
            })
            .collect(),
        answers: Answer::iter()
            .map(|a| QuestionsContextAnswer {
                id: a as i32,
                text: a.to_string(), 
            })
            .collect(),
        parent: "layout"
    };

    Ok(Template::render("quiz/questions", &context))
}

#[derive(Debug)]
pub struct QuestionResponse {
    question_id: i32,
    answer: Answer,
}

#[derive(Debug)]
pub struct QuestionsResponse {
    responses: Vec<QuestionResponse>,
}

impl<'a> FromForm<'a> for QuestionsResponse {
    type Error = &'static str;

    fn from_form(items: &mut FormItems<'a>, _strict: bool) -> Result<QuestionsResponse, &'static str> {
        let mut responses = Vec::new();

        for item in items {
            responses.push(QuestionResponse {
                question_id: item.key
                    .as_str()
                    .trim_start_matches("q_")
                    .parse::<i32>()
                    .map_err(|_| "invalid form response")?,
                answer: Answer::from_str(item.value.as_str())
                    .map_err(|_| "invalid form response")?,
            });
        }

        Ok(QuestionsResponse { responses })
    }
}

fn gen_session_id(len: u8) -> String{
    let mut rng = thread_rng();
    (0..len).map(|_| rng.sample(Alphanumeric)).collect()
}

#[post("/quiz/questions", data="<answers>")]
pub fn post_questions(
    conn: Conn,
    answers: Form<QuestionsResponse>
) -> Redirect {
    let session_id = gen_session_id(SESSION_ID_LEN);

    let question_answers: Vec<QuestionAnswer> = answers
        .responses
        .iter()
        .map(|qr| QuestionAnswer {
            session_id: session_id.to_owned(),
            question_id: qr.question_id,
            answer_id: qr.answer as i32,
        })
        .collect();

    store_response(&conn, &question_answers);

    let econ = "50";
    let soc = "n50";

    Redirect::to(format!("/quiz/results?econ={}&soc={}", econ, soc))
}

#[derive(Debug, Serialize)]
struct ResultsContext {
    econ: String,
    soc: String,
    parent: &'static str,
}

#[get("/quiz/results?<econ>&<soc>")]
pub fn get_results(econ: String, soc: String) -> Template {
    let context = ResultsContext {
        econ: econ,
        soc: soc,
        parent: "layout"
    };

    Template::render("quiz/results", &context)
}

#[derive(Debug, Serialize)]
struct ChartContext {
    x: i32,
    y: i32
}

#[get("/quiz/chart/<econ>/<soc>")]
pub fn get_chart(econ: String, soc: String) -> Content<Template> {
    let mut econ_n: i32;
    let mut soc_n: i32;

    if econ.chars().nth(0) == Some('n') {
        econ_n = econ
            .trim_start_matches("n")
            .parse::<i32>()
            .unwrap_or(0);

        if econ_n > 100 { econ_n = 100; }
        econ_n = -econ_n;
    } else {
        econ_n = econ
            .parse::<i32>()
            .unwrap_or(0);
    }

    if soc.chars().nth(0) == Some('n') {
        soc_n = soc
            .trim_start_matches("n")
            .parse::<i32>()
            .unwrap_or(0);

        if soc_n > 100 { soc_n = 100; }
    } else {
        soc_n = soc
            .parse::<i32>()
            .unwrap_or(0);
        soc_n = -soc_n;
    }

    let context = ChartContext {
        x: econ_n,
        y: soc_n
    };

    Content(
        ContentType::SVG,
        Template::render("quiz/chart", &context)
    )
}
