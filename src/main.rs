mod api;
mod error;
mod models;
mod test;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use error::GraniumError;
use std::env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let server_port = match env::var("SERVER_PORT") {
        Ok(var) => var,
        Err(err) => {
            eprintln!("{:#?}", GraniumError::ReadEnvVariable { source: err });
            return Ok(());
        }
    };

    HttpServer::new(|| App::new().route("/api/v1/ltp", web::get().to(api::v1::get_ltp)))
        .bind(format!("0.0.0.0:{}", server_port))?
        .run()
        .await
}
