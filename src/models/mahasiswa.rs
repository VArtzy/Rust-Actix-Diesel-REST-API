use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::mahasiswas)]
pub struct Mahasiswa {
    #[serde(default)]
    pub id: String,
    pub nim: Option<String>,
    pub nama: String,
    pub jurusan: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}