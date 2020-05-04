table! {
    question_answers(question_id, answer_id) {
        question_id -> Int4,
        question_number -> Int4,
        question_text -> Varchar,
        question_page -> Int4,
        answer_id -> Int4,
        answer_text -> Varchar,
        answer_score_x -> Int4,
        answer_score_y -> Int4,
    }
}
