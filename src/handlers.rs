use async_std::fs::OpenOptions;
use async_std::io;
use tide::prelude::json;
use tide::{log::info, Request};
use tide::{Response, StatusCode};
pub async fn upload(req: Request<()>) -> tide::Result {
    let api_key = std::env::var("API_KEY").expect("API_KEY not found in .env");
    let provided_key: String = req
        .header("API-KEY")
        .map(|header_values| header_values.as_str().to_string())
        .unwrap_or_else(|| "na".to_string());
    if provided_key == api_key {
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
        let domain = std::env::var("DOMAIN").expect("No domain env variable set");
        let mut response = Response::new(StatusCode::InternalServerError);
        response.insert_header("Content-Type", "application/json");
        response.insert_header("Access-Control-Allow-Origin", "*");
        response.set_body(json!({
            "file":format!("{}/{}",domain,path),
        }));
        return Ok(response);
    }
    let response_json = json!({
        "message": "Unauthorized. Invalid API key",
    });
    let mut response = Response::new(StatusCode::Unauthorized);
    response.insert_header("Content-Type", "application/json");
    response.insert_header("Access-Control-Allow-Origin", "*");
    response.set_body(response_json);
    Ok(response)
}
