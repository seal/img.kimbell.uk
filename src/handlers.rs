use async_std::fs::OpenOptions;
use async_std::io;
use tide::prelude::json;

use tide::{log::info, Request};
use tide::{Response, StatusCode};
/*
pub async fn all(_req: Request<()>) -> tide::Result {
    let paths = fs::read_dir("./images/").unwrap();
    let file_names: Vec<String> = paths
        .filter_map(|entry| entry.ok())
        .flat_map(|entry| entry.file_name().into_string().ok())
        .collect();
    let mut response = Response::new(StatusCode::InternalServerError);
    response.insert_header("Content-Type", "application/json");
    response.insert_header("Access-Control-Allow-Origin", "*");
    response.set_body(json!({
        "files":file_names,
    }));
    Ok(response)
}
*/
pub async fn upload(req: Request<()>) -> tide::Result {
    let path = req.param("file")?.to_string().clone();
    let fs_path = format!("./images/{}", path);
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&fs_path)
        .await?;

    let bytes_written = io::copy(req, file).await?;

    info!("file written", {
        bytes: bytes_written,
        path: fs_path
    });

    let mut response = Response::new(StatusCode::InternalServerError);
    response.insert_header("Content-Type", "application/json");
    response.insert_header("Access-Control-Allow-Origin", "*");
    response.set_body(json!({
        "file":format!("https://img.kimbell.uk/{}",path),
    }));
    Ok(response)
}
