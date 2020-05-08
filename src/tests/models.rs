use super::utils::*;
use crate::models::questions;
use crate::models::question_answers;

#[test]
fn test_questions_count() {
    let conn = test_connection();

    let count = questions::count(&conn).unwrap();
    assert_eq!(count, 54);
}

#[test]
fn test_all() {
    let conn = test_connection();

    let questions = question_answers::load_all(&conn).unwrap();

    assert_eq!(questions.len(), 216);
}
