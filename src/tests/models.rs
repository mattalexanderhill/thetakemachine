use super::utils::*;

use diesel::Connection;

use crate::models::questions;
use crate::models::answers;
use crate::models::responses;
use crate::models::response_scores;

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
fn test_questions_all() {
    let conn = test_connection();

    let questions = questions::load_all(&conn, None).unwrap();

    assert_eq!(questions.len(), 57);
}

#[test]
fn test_store_and_load_responses() {
    let conn = test_connection();
    conn.begin_test_transaction().unwrap();

    let session = "test_transaction_123";

    let responses = vec![
        responses::Response {
            session_id: session.into(),
            question_id: 1,
            answer_id: 2,
        },
        responses::Response {
            session_id: session.into(),
            question_id: 2,
            answer_id: 3,
        }
    ];

     let stored = responses::store_responses(&conn, &responses)
        .unwrap();

    assert_eq!(stored, 2);

    let scores = response_scores::load_from_session(&conn, session)
        .unwrap();

    assert_eq!(scores.len(), 2);
}
