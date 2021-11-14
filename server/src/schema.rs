table! {
    command (id) {
        id -> Int4,
        stonker_id -> Int4,
        company_id -> Int4,
        threshold -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
        share -> Int4,
    }
}

table! {
    company (id) {
        id -> Int4,
        name -> Varchar,
        performer_id -> Int4,
    }
}

table! {
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
    stock (id) {
        id -> Int4,
        stonker_id -> Int4,
        company_id -> Int4,
    }
}

table! {
    stonker (id) {
        id -> Int4,
        name -> Varchar,
        balance -> Int4,
    }
}

joinable!(command -> company (company_id));
joinable!(command -> stonker (stonker_id));
joinable!(company -> stonker (performer_id));
joinable!(history -> stock (stock_id));
joinable!(history -> stonker (stonker_id));
joinable!(stock -> company (company_id));
joinable!(stock -> stonker (stonker_id));

allow_tables_to_appear_in_same_query!(
    command,
    company,
    history,
    stock,
    stonker,
);
