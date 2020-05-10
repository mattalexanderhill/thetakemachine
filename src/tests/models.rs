use super::utils::*;
use crate::models::questions;
use crate::models::answers;

#[test]
fn test_answers_iter() {
    let answers: Vec<String> = answers::Answer::iter()
        .map(|a| a.to_string())
        .collect();

    assert_eq!(answers[0], "Agree");
    assert_eq!(answers[1], "Disagree");
    assert_eq!(answers[2], "Don't Know");
    assert_eq!(answers[3], "Don't Care");
}

#[test]
fn test_questions_count() {
    let conn = test_connection();

    let count = questions::count(&conn).unwrap();
    assert_eq!(count, 54);
}

#[test]
fn test_questions_all() {
    let conn = test_connection();

    let questions = questions::load_all(&conn, None).unwrap();

    assert_eq!(questions.len(), 54);
}
