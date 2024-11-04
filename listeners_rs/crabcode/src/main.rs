use axum::{
    extract::{Path, Query},
    response::{IntoResponse, Json, StreamBody},
    routing::{get, post},
    Router,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{self, BufReader, Read},
    path::PathBuf,
};
use tokio_util::io::ReaderStream;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

const CODE_DIR: &str = "/path/to/code";

#[tokio::main]
async fn main() {
    // Build our application with some routes
    let app = Router::new()
        .route("/save-code", post(save_code))
        .route("/list-files", get(list_files))
        .route("/show-file/:filename", get(show_file));

    // Run it with hyper on localhost:8000
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Define the request body structure
#[derive(Deserialize)]
struct SaveCodeRequest {
    filename: String,
    content: String,
}

// Save code to a file in the CODE_DIR
async fn save_code(Json(payload): Json<SaveCodeRequest>) -> impl IntoResponse {
    let file_path = format!("{}/{}", CODE_DIR, payload.filename);

    match save_to_file(file_path, payload.content).await {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"message": "Code saved successfully"}))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"message": "Error saving file"}))),
    }
}

async fn save_to_file(file_path: String, content: String) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)
        .await?;
    file.write_all(content.as_bytes()).await?;
    Ok(())
}

// List available files in CODE_DIR
async fn list_files() -> impl IntoResponse {
    let files = match fs::read_dir(CODE_DIR) {
        Ok(entries) => {
            let mut file_names = vec![];
            for entry in entries.flatten() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    file_names.push(file_name);
                }
            }
            file_names
        }
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"message": "Error listing files"}))),
    };

    (StatusCode::OK, Json(serde_json::json!({ "files": files })))
}

// Show file content via streaming response
async fn show_file(Path(filename): Path<String>) -> impl IntoResponse {
    let file_path = format!("{}/{}", CODE_DIR, filename);

    match File::open(&file_path) {
        Ok(file) => {
            let stream = ReaderStream::new(BufReader::new(file));
            let body = StreamBody::new(stream);
            Ok::<_, StatusCode>((StatusCode::OK, body))
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}
