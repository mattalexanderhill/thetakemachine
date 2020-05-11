table! {
    answers (id) {
        id -> Int4,
        text -> Varchar,
    }
}

table! {
    questions (id) {
        id -> Int4,
        number -> Int4,
        text -> Varchar,
    }
}

table! {
    responses (id) {
        id -> Int4,
        session_id -> Varchar,
        question_id -> Int4,
        answer_id -> Int4,
        at -> Timestamp,
    }
}

table! {
    scores (id) {
        id -> Int4,
        question_id -> Int4,
        answer_id -> Int4,
        x -> Int4,
        y -> Int4,
    }
}

joinable!(responses -> answers (answer_id));
joinable!(responses -> questions (question_id));
joinable!(scores -> answers (answer_id));
joinable!(scores -> questions (question_id));

allow_tables_to_appear_in_same_query!(
    answers,
    questions,
    responses,
    scores,
);
