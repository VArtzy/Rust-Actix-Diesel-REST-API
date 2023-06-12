use actix_web::web;
use actix_web::{web::{Data, Json}, post, get, put, delete, HttpResponse};
use crate::{models::mahasiswa::Mahasiswa, repository::database::Database};

#[post("/mahasiswa")]
async fn create_mahasiswa(db: Data<Database>, new_mahasiswa: Json<Mahasiswa>) -> HttpResponse {
    let mahasiswa = db.create_mahasiswa(new_mahasiswa.into_inner());
    match mahasiswa {
        Ok(mahasiswa) => HttpResponse::Ok().json(mahasiswa),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/mahasiswa")]
async fn get_mahasiswa(db: web::Data<Database>) -> HttpResponse {
    let mahasiswas = db.get_mahasiswa();
    HttpResponse::Ok().json(mahasiswas)
}

#[get("/mahasiswa/{id}")]
async fn get_mahasiswa_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let mahasiswa = db.get_mahasiswa_by_id(&id);
    match mahasiswa {
        Some(mahasiswa) => HttpResponse::Ok().json(mahasiswa),
        None => HttpResponse::NotFound().body("Mahasiswa not found"),
    }
}

#[put("/mahasiswa/{id}")]
async fn update_mahasiswa_by_id(db: web::Data<Database>, id: web::Path<String>, updated_mahasiswa: web::Json<Mahasiswa>) -> HttpResponse {
    let mahasiswa = db.update_mahasiswa_by_id(&id, updated_mahasiswa.into_inner());
    match mahasiswa {
        Some(mahasiswa) => HttpResponse::Ok().json(mahasiswa),
        None => HttpResponse::NotFound().body("Mahasiswa not found"),
    }
}

#[delete("/mahasiswa/{id}")]
async fn delete_mahasisiswa_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let mahasiswa = db.delete_mahasiswa_by_id(&id);
    match mahasiswa {
        Some(mahasiswa) => HttpResponse::Ok().json(mahasiswa),
        None => HttpResponse::NotFound().body("Mahasiswa not found"),
    }
}

pub fn config (cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_mahasiswa)
            .service(get_mahasiswa)
            .service(get_mahasiswa_by_id)
            .service(update_mahasiswa_by_id)
            .service(delete_mahasisiswa_by_id)
    );
}