table! {
    pages (Id) {
        Id -> Integer,
        Name -> Text,
    }
}

table! {
    products (id) {
        id -> Integer,
        name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    pages,
    products,
);
