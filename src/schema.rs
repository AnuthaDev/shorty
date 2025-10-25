// @generated automatically by Diesel CLI.

diesel::table! {
    urls (id) {
        id -> Int4,
        original_url -> Text,
        #[max_length = 10]
        short_code -> Varchar,
        created_at -> Timestamp,
    }
}
