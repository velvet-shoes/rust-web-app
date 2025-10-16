// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Nullable<Integer>,
        title -> Text,
        creator -> Text,
        text -> Text,
        content -> Nullable<Binary>,
        date_of_creation -> Nullable<Timestamp>,
    }
}
