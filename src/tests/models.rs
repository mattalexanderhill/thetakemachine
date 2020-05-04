use super::utils::*;
use crate::models::questions;
use crate::models::question_answers;

#[test]
fn test_questions_count() {
    let conn = test_connection();

    let count = questions::count(&conn).unwrap();
    assert_eq!(count, 3);
}

#[test]
fn test_load_page() {
    let conn = test_connection();

    let questions = question_answers::load_page(&conn, 1).unwrap();

    assert_eq!(questions.len(), 6);
}
