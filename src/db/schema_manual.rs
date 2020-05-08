table! {
    question_answers(question_id, answer_id) {
        question_id -> Int4,
        question_number -> Int4,
        question_text -> Varchar,
        answer_id -> Int4,
        answer_text -> Varchar,
        answer_score_x -> Nullable<Int4>,
        answer_score_y -> Nullable<Int4>,
    }
}
