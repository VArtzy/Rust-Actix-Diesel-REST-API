// @generated automatically by Diesel CLI.

diesel::table! {
    mahasiswas (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        nim -> Nullable<Varchar>,
        #[max_length = 255]
        nama -> Varchar,
        #[max_length = 255]
        jurusan -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
