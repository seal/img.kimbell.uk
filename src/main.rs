use crate::handlers::upload;
use dotenv::dotenv;
use std::io::ErrorKind;
use tide::log::error;
use tide::log::info;
use tide::log::LevelFilter;
use tide::utils::After;
mod handlers;
use env_logger;
use tide::Response;
use tide::StatusCode;

#[async_std::main]
async fn main() -> tide::Result<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    info!("Initialising...");
    if let Err(e) = async_std::fs::create_dir_all("./images").await {
        eprintln!("Error creating 'images' directory: {}", e);
        return Err(tide::Error::from_str(
            StatusCode::InternalServerError,
            "Internal Server Error",
        ));
    }
    dotenv().ok();
    info!("Env vars OK");
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());
    info!("DB Connection OK");
    app.with(After(|mut res: Response| async {
        if let Some(err) = res.downcast_error::<async_std::io::Error>() {
            match err.kind() {
                ErrorKind::NotFound => {
                    error!("{:?}", err);
                    let msg = format!("Error: {:?}", err);
                    res.set_status(StatusCode::NotFound);
                    res.set_body(msg);
                }
                _ => {
                    error!("{:?}", err);
                    let msg = format!("Internal Server Error: {:?}", err);
                    res.set_status(StatusCode::InternalServerError);
                    res.set_body(msg);
                }
            }
        }
        Ok(res)
    }));

    app.at("/new/:file").put(upload);
    app.at("/").serve_dir("images/")?;
    info!("Created Routes");
    app.listen("0.0.0.0:3000").await?;
    Ok(())
}
