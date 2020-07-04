use rand::{distributions::Alphanumeric, thread_rng, Rng};

use rocket::request::{Form, FromForm, FormItems};
use rocket::response::Redirect;
use rocket::http::{ContentType, Status};
use rocket::response::content::Content;

use rocket_contrib::templates::Template;

use crate::models::questions::load_all;
use crate::models::answers::Answer;
use crate::models::responses::{Response, store_responses};
use crate::models::response_scores::load_from_session;
use crate::models::demographics::{self, Gender, AgeGroup,
    Politics, Ethics, Demographic, store_demographic};
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

#[derive(Serialize)]
struct AboutContext {
    parent: &'static str,
}

#[get("/quiz/about")]
pub fn get_about() -> Template {
    let context = AboutContext { parent: "layout" };
    Template::render("quiz/about", &context)
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
    session_id: String,
    questions: Vec<QuestionsContextQuestion>,
    answers: Vec<QuestionsContextAnswer>,
    parent: &'static str,
}

fn gen_session_id(len: u8) -> String{
    let mut rng = thread_rng();
    (0..len).map(|_| rng.sample(Alphanumeric)).collect()
}

#[get("/quiz/questions")]
pub fn get_questions(conn: Conn) -> Result<Template, Status> {
    let questions = load_all(&conn, None).unwrap_or(vec![]);

    if questions.is_empty() {
        return Err(Status::NotFound);
    }

    let context = QuestionsContext {
        session_id: gen_session_id(SESSION_ID_LEN),
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
pub struct QuestionAnswer {
    question_id: i32,
    answer: Answer,
}

#[derive(Debug)]
pub struct QuestionsForm {
    session_id: String,
    question_answers: Vec<QuestionAnswer>,
}

impl<'a> FromForm<'a> for QuestionsForm {
    type Error = &'static str;

    fn from_form(items: &mut FormItems<'a>, _strict: bool) -> Result<QuestionsForm, &'static str> {
        let mut question_answers = Vec::new();
        let mut session_id = String::with_capacity(SESSION_ID_LEN.into());

        for item in items {
            let (key, value) = item.key_value_decoded();

            if key == "session" {
                session_id.push_str(&value.trim());
            } else {
                question_answers.push(QuestionAnswer {
                    question_id: key
                        .trim_start_matches("q_")
                        .parse::<i32>()
                        .map_err(|_| "invalid form response")?,
                    answer: Answer::from_str(&value)
                        .map_err(|_| "invalid form response")?,
                });
            }
        }

        Ok(QuestionsForm { session_id, question_answers })
    }
}


#[post("/quiz/questions", data="<form>")]
pub fn post_questions(
    conn: Conn,
    form: Form<QuestionsForm>
) -> Result<Redirect, &'static str> {
    let responses: Vec<Response> = form
        .question_answers
        .iter()
        .map(|qa| Response {
            session_id: form.session_id.to_owned(),
            question_id: qa.question_id,
            answer_id: qa.answer as i32,
        })
        .collect();

    store_responses(&conn, &responses).expect("storage error");

    let redirect_uri = if form.session_id.is_empty() {
        "/quiz/_/demographics".to_owned()
    } else {
        format!("/quiz/{}/demographics", form.session_id)
    };

    Ok(Redirect::to(redirect_uri))
}


#[derive(Debug, Serialize)]
struct QuadrantMessageContext {
    show_blue: bool,
    show_green: bool,
    show_red: bool,
    show_yellow: bool,
}

impl Default for QuadrantMessageContext {
    fn default() -> QuadrantMessageContext {
        QuadrantMessageContext {
            show_blue: false,
            show_green: false,
            show_red: false,
            show_yellow: false,
        }
    }
}


#[derive(Debug, Serialize)]
struct ResultsContext {
    x_fmt: String,
    y_fmt: String,
    x: String,
    y: String,
    parent: &'static str,
    message: QuadrantMessageContext,
}

#[get("/quiz/<session>/results")]
pub fn get_results(conn: Conn, session: String) -> Result<Template, Status> {
    let mut result: (i64, i64) = (0, 0);

    if session != "_" {
        let scores = load_from_session(&conn, &session)
            .expect("db error");
        let mut x_scores: Vec<i32> = Vec::with_capacity(scores.len());
        let mut y_scores: Vec<i32> = Vec::with_capacity(scores.len());

        for rs in scores.iter() {
            match rs.answer {
                Answer::Agree | Answer::Disagree => {
                    if let Some(x) = rs.x {
                        x_scores.push(x);
                    }
                    if let Some(y) = rs.y {
                        y_scores.push(y);
                    }
                },
                Answer::DontCare | Answer::DontKnow => {}
            }
        }

        if !x_scores.is_empty() {
            let sum: i32 = x_scores.iter().sum();
            result.0 = (f64::from(sum) / (x_scores.len() as f64)) as i64
        }

        if !y_scores.is_empty() {
            let sum: i32 = y_scores.iter().sum();
            result.1 = (f64::from(sum) / (y_scores.len() as f64)) as i64
        }
    }

    let (x, y) = result;

    let x_str = if x < 0 {
        format!("n{}", x.abs())
    } else {
        x.to_string()
    };

    let y_str = if y < 0 {
        format!("n{}", y.abs())
    } else {
        y.to_string()
    };

    let mut message = QuadrantMessageContext::default();

    match (x < 0, y < 0) {
        (false, false) => message.show_blue = true,
        (false, true)  => message.show_yellow = true,
        (true, true)   => message.show_green = true,
        (true, false)  => message.show_red = true,
    };

    let context = ResultsContext {
        x_fmt: format!("{:.2}", x as f32 / 100.0),
        y_fmt: format!("{:.2}", y as f32 / 100.0),
        x: x_str,
        y: y_str,
        parent: "layout",
        message,
    };

    Ok(Template::render("quiz/results", &context))
}

#[derive(Debug, Serialize)]
struct ChartContext {
    x: i32,
    y: i32
}

#[get("/quiz/chart/<x_str>/<y_str>")]
pub fn get_chart(x_str: String, y_str: String) -> Content<Template> {
    let mut x: i32;
    let mut y: i32;

    if x_str.chars().nth(0) == Some('n') {
        x = x_str
            .trim_start_matches("n")
            .parse::<i32>()
            .unwrap_or(0);

        if x > 100 { x = 100; }
        x = -x;
    } else {
        x = x_str.parse::<i32>().unwrap_or(0);
    }

    if y_str.chars().nth(0) == Some('n') {
        y = y_str
            .trim_start_matches("n")
            .parse::<i32>()
            .unwrap_or(0);

        if y > 100 { y = 100; }
    } else {
        y = y_str.parse::<i32>().unwrap_or(0);
        y = -y;
    }

    let context = ChartContext { x, y };

    Content(
        ContentType::SVG,
        Template::render("quiz/chart", &context)
    )
}

#[derive(Debug, Serialize)]
struct DemographicsOption {
    id: i32,
    text: String,
}

#[derive(Debug, Serialize)]
struct DemographicsContext {
    session_id: String,
    genders: Vec<DemographicsOption>,
    age_groups: Vec<DemographicsOption>,
    politics: Vec<DemographicsOption>,
    ethics: Vec<DemographicsOption>,
    parent: &'static str,
}

#[get("/quiz/<session_id>/demographics")]
pub fn get_demographics(session_id: String) -> Result<Template, Status> {
    let context = DemographicsContext {
        session_id: session_id,
        genders: demographics::GENDER_OPTIONS.iter().map(|o| DemographicsOption{id: *o as i32, text: format!("{}", o) }).collect(),
        age_groups: demographics::AGE_GROUP_OPTIONS.iter().map(|o| DemographicsOption{id: *o as i32, text: format!("{}", o) }).collect(),
        politics: demographics::POLITICS_OPTIONS.iter().map(|o| DemographicsOption{id: *o as i32, text: format!("{}", o) }).collect(),
        ethics: demographics::ETHICS_OPTIONS.iter().map(|o| DemographicsOption{id: *o as i32, text: format!("{}", o) }).collect(),
        parent: "layout",
    };

    Ok(Template::render("quiz/demographics", &context))
}

#[derive(Debug)]
pub struct DemographicsForm {
    session_id: String,
    gender: Option<Gender>,
    age_group: Option<AgeGroup>,
    politics: Option<Politics>,
    ethics: Option<Ethics>,
}

impl<'a> FromForm<'a> for DemographicsForm {
    type Error = &'static str;

    fn from_form(items: &mut FormItems<'a>, _strict: bool) -> Result<DemographicsForm, &'static str> {
        let mut form = DemographicsForm {
            session_id: String::with_capacity(SESSION_ID_LEN.into()),
            gender: None,
            age_group: None,
            politics: None,
            ethics: None,
        };

        for item in items {
            let (key, value) = item.key_value_decoded();

            match key.as_str() {
                "session" => {
                    form.session_id.push_str(&value.trim());
                },
                "gender" => {
                    form.gender = Gender::from_str(&value).ok();
                },
                "age" => {
                    form.age_group = AgeGroup::from_str(&value).ok();
                },
                "politics" => {
                    form.politics = Politics::from_str(&value).ok();
                },
                "ethics" => {
                    form.ethics = Ethics::from_str(&value).ok();
                },
                _ => {}
            }
        }

        Ok(form)
    }
}

#[post("/quiz/demographics", data="<form>")]
pub fn post_demographics(
    conn: Conn,
    form: Form<DemographicsForm>
) -> Result<Redirect, &'static str> {

    let dem = Demographic {
        session_id: form.session_id.clone(),
        gender: form.gender,
        age_group: form.age_group,
        politics: form.politics,
        ethics: form.ethics,
    };

    store_demographic(&conn, &dem).expect("storage error");

    let redirect_uri = if form.session_id.is_empty() {
        "/quiz/_/results".to_owned()
    } else {
        format!("/quiz/{}/results", form.session_id)
    };

    Ok(Redirect::to(redirect_uri))
}
