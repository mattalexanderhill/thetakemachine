table! {
    response_scores (id) {
        id -> Int4,
        session_id -> Varchar,
        question_id -> Int4,
        answer_id -> Int4,
        x -> Nullable<Int4>,
        y -> Nullable<Int4>,
    }
}
