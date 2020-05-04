use super::utils::*;
use rocket::http::Status;

#[test]
fn test_home() {
    let client = test_client();

    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::SeeOther);
}

#[test]
fn test_index() {
    let client = test_client();

    let response = client.get("/quiz").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_questions() {
    let client = test_client();

    let response = client.get("/quiz/questions").dispatch();
    assert_eq!(response.status(), Status::Ok);
}
