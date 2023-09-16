// @generated automatically by Diesel CLI.

diesel::table! {
    category (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        active -> Bool,
    }
}
