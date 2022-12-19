// @generated automatically by Diesel CLI.

diesel::table! {
    pages (uuid) {
        uuid -> Uuid,
        page_title -> Nullable<Varchar>,
        slug -> Nullable<Varchar>,
    }
}
