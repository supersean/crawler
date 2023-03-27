// @generated automatically by Diesel CLI.

diesel::table! {
    crawl_results (id) {
        id -> Int4,
        url -> Varchar,
        raw_html -> Varchar,
        created_at -> Timestamp,
        entry_id -> Int4,
    }
}

diesel::table! {
    entries (id) {
        id -> Int4,
        terms -> Varchar,
        created_at -> Timestamp,
        user_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(crawl_results -> entries (entry_id));
diesel::joinable!(entries -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    crawl_results,
    entries,
    users,
);
