use core::fmt::Error;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::models::mahasiswa::Mahasiswa;
use crate::repository::schema::mahasiswas::dsl::*;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn create_mahasiswa(&self, mahasiswa: Mahasiswa) -> Result<Mahasiswa, Error> {
        let mahasiswa = Mahasiswa {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            ..mahasiswa
        };
        diesel::insert_into(mahasiswas)
            .values(&mahasiswa)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating new mahasiswa");
        Ok(mahasiswa)
    }

    pub fn get_mahasiswa(&self) -> Vec<Mahasiswa> {
        mahasiswas
            .load::<Mahasiswa>(&mut self.pool.get().unwrap())
            .expect("Error loading all mahasiswa")
    }

    pub fn get_mahasiswa_by_id(&self, mahasiswa_id: &str) -> Option<Mahasiswa> {
        let mahasiswa = mahasiswas
            .find(mahasiswa_id)
            .get_result::<Mahasiswa>(&mut self.pool.get().unwrap())
            .expect("Error loading mahasiswa by id");
        Some(mahasiswa)
    }

    pub fn update_mahasiswa_by_id(&self, mahasiswa_id: &str, mut mahasiswa: Mahasiswa) -> Option<Mahasiswa> {
        mahasiswa.updated_at = Some(Utc::now().naive_utc());
        let mahasiswa = diesel::update(mahasiswas.find(mahasiswa_id))
            .set(&mahasiswa)
            .get_result::<Mahasiswa>(&mut self.pool.get().unwrap())
            .expect("Error updating mahasiswa by id");
        Some(mahasiswa)
    }

    pub fn delete_mahasiswa_by_id(&self, mahasiswa_id: &str) -> Option<usize> {
        let count = diesel::delete(mahasiswas.find(mahasiswa_id))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting mahasiswa by id");
        Some(count)
    }
}