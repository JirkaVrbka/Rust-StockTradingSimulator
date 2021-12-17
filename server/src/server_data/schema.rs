table! {
    use diesel::sql_types::*;
    use crate::models::command::*;

    command (id) {
        id -> Int4,
        stonker_id -> Int4,
        company_id -> Int4,
        threshold -> Int4,
        share -> Int4,
        kind -> Commandtypes,
        created_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::command::*;

    company (id) {
        id -> Int4,
        name -> Varchar,
        performer_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::command::*;

    history (id) {
        id -> Int4,
        stonker_id -> Int4,
        stock_id -> Int4,
        bought_for -> Nullable<Int4>,
        created_at -> Timestamp,
        sold_for -> Nullable<Int4>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::command::*;

    news (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        author -> Varchar,
        created_at -> Timestamp,
        kind -> Effect,
        company_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::command::*;

    stock (id) {
        id -> Int4,
        stonker_id -> Int4,
        company_id -> Int4,
        share -> Int4,
        bought_for -> Int4,
        sold_for -> Nullable<Int4>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::command::*;

    stonker (id) {
        id -> Int4,
        name -> Varchar,
        balance -> Int4,
        blocked_balance -> Int4,
        invested_balance -> Int4,
    }
}

joinable!(command -> company (company_id));
joinable!(command -> stonker (stonker_id));
joinable!(company -> stonker (performer_id));
joinable!(history -> stock (stock_id));
joinable!(history -> stonker (stonker_id));
joinable!(news -> company (company_id));
joinable!(stock -> company (company_id));
joinable!(stock -> stonker (stonker_id));

allow_tables_to_appear_in_same_query!(
    command,
    company,
    history,
    news,
    stock,
    stonker,
);
