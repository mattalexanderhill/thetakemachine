use rocket::response::Redirect;

pub mod catch;
pub mod quiz;

#[get("/")]
pub fn get_index() -> Redirect {
    Redirect::to("/quiz")
}

#[get("/favicon.ico")]
pub fn get_favicon() -> Redirect {
    Redirect::to("/assets/imgs/favicon.ico")
}
