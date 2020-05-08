use rocket::Request;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct NotFoundContext {
    title: &'static str,
    parent: &'static str,
}

#[catch(404)]
pub fn not_found(_req: &Request<'_>) -> Template {
    let context = NotFoundContext {
        title: "404",
        parent: "layout",
    };

    Template::render("errors/404", &context)
}

#[derive(Serialize)]
struct ServerErrorContext {
    title: &'static str,
    parent: &'static str,
}

#[catch(500)]
pub fn server_error(_req: &Request<'_>) -> Template {
    let context = ServerErrorContext {
        title: "500",
        parent: "layout"
    };

    Template::render("error/500", &context)
}
