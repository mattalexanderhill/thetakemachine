table! {
    answers (id) {
        id -> Int4,
        text -> Varchar,
        question -> Int4,
        score_x -> Int4,
        score_y -> Int4,
    }
}

table! {
    questions (id) {
        id -> Int4,
        number -> Int4,
        text -> Varchar,
        page -> Int4,
    }
}

joinable!(answers -> questions (question));

allow_tables_to_appear_in_same_query!(
    answers,
    questions,
);
