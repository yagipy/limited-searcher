table! {
    folders (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
        status -> Bool,
    }
}

table! {
    urls (id) {
        id -> Int4,
        folder_id -> Int4,
        url -> Varchar,
        status -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
    }
}

joinable!(folders -> users (user_id));
joinable!(urls -> folders (folder_id));

allow_tables_to_appear_in_same_query!(
    folders,
    urls,
    users,
);
