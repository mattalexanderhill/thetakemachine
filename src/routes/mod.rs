use rocket::response::Redirect;

pub mod catch;
pub mod quiz;

#[derive(Serialize)]
struct TemplateContext {
    items: Vec<&'static str>,
    parent: &'static str,
}

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to("/quiz")
}

#[get("/favicon.ico")]
pub fn favicon() -> Redirect {
    Redirect::to("/assets/imgs/favicon.ico")
}
