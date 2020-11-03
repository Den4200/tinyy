table! {
    tiny_urls (code) {
        code -> Varchar,
        url -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    tiny_urls,
    users,
);
