use rocket::Request;
use rocket_contrib::templates::Template;

#[catch(404)]
pub fn not_found(_req: &Request<'_>) -> Template {
    Template::render("errors/404", &())
}

// #[catch(500)]
// fn client_error(req: &Request<'_>) -> Template {
//     Template::render("error/500", &())
// }
