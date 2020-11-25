table! {
    collections (id) {
        id -> Uuid,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    items (id) {
        id -> Uuid,
        collection_id -> Uuid,
        name -> Text,
        visible -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(items -> collections (collection_id));

allow_tables_to_appear_in_same_query!(
    collections,
    items,
);
