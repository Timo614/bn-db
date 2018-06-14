table! {
    users (id) {
        id -> Int4,
        email -> Text,
        hashed_pw -> Text,
        active -> Bool,
    }
}
