table! {
    answers (id) {
        id -> Int4,
        text -> Varchar,
    }
}

table! {
    question_answers (id) {
        id -> Int4,
        session_id -> Varchar,
        question_id -> Int4,
        answer_id -> Int4,
    }
}

table! {
    questions (id) {
        id -> Int4,
        number -> Int4,
        text -> Varchar,
    }
}

joinable!(question_answers -> answers (answer_id));
joinable!(question_answers -> questions (question_id));

allow_tables_to_appear_in_same_query!(
    answers,
    question_answers,
    questions,
);
