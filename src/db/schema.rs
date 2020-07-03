table! {
    age_group (id) {
        id -> Int4,
        text -> Varchar,
    }
}

table! {
    answers (id) {
        id -> Int4,
        text -> Varchar,
    }
}

table! {
    demographics (id) {
        id -> Int4,
        session_id -> Varchar,
        gender -> Nullable<Int4>,
        age_group -> Nullable<Int4>,
        politics -> Nullable<Int4>,
        ethics -> Nullable<Int4>,
    }
}

table! {
    ethics (id) {
        id -> Int4,
        text -> Varchar,
    }
}

table! {
    gender (id) {
        id -> Int4,
        text -> Varchar,
    }
}

table! {
    politics (id) {
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

joinable!(demographics -> age_group (age_group));
joinable!(demographics -> ethics (ethics));
joinable!(demographics -> gender (gender));
joinable!(demographics -> politics (politics));
joinable!(responses -> answers (answer_id));
joinable!(responses -> questions (question_id));
joinable!(scores -> answers (answer_id));
joinable!(scores -> questions (question_id));

allow_tables_to_appear_in_same_query!(
    age_group,
    answers,
    demographics,
    ethics,
    gender,
    politics,
    questions,
    responses,
    scores,
);
