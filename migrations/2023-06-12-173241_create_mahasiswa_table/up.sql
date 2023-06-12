-- Your SQL goes here
CREATE TABLE mahasiswas (
    id VARCHAR(255) PRIMARY KEY,
    nim VARCHAR(255),
    nama VARCHAR(255) NOT NULL,
    jurusan VARCHAR(255) NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
)