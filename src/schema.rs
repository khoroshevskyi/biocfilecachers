// @generated automatically by Diesel CLI.

diesel::table! {
    resource (id) {
        id -> Integer,
        rid -> Text,
        rname -> Text,
        create_time -> Nullable<Timestamp>,
        access_time -> Nullable<Timestamp>,
        rpath -> Text,
        rtype -> Nullable<Text>,
        fpath -> Nullable<Text>,
        last_modified_time -> Nullable<Timestamp>,
        etag -> Nullable<Text>,
        expires -> Nullable<Timestamp>,
    }
}
