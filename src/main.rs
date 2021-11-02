use std::fs::File;
use std::io::BufReader;
use actix_web::Error;
use actix_web::{middleware::Logger, web, App, HttpServer, HttpResponse};
use feeder_types::*;
use std::sync::Arc;
use fetcher::Fetcher;

#[macro_use]
extern crate log;

mod fetcher;

/// Starts the server and blocks until it is shutdown
///
/// # Errors
/// Returns errors on varoius IO errors
pub async fn start_server(fetcher: Arc<Fetcher>) -> Result<(), std::io::Error> {
    info!("starting server!");

    let fetcher = Arc::clone(&fetcher);
    let app = move || {
        App::new()
            .data(Arc::clone(&fetcher))
            .wrap(Logger::default())
            .configure(routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3344")?
        .run().await
}


/// Configures routes
///
/// All modules should register their routes here
/// # Parameter
/// `cfg`: The Actix configuration
fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/updates", web::get().to(updates));
}

async fn updates(fetcher: web::Data<Arc<Fetcher>>) -> Result<HttpResponse, Error> {
    let items = fetcher.get_entries().await;
    Ok(HttpResponse::Created().header("Access-Control-Allow-Origin", "*")
        .json(items))
}

#[actix_web::main]
async fn main() {
    let fetcher = Arc::new(fetcher::Fetcher::new());
    env_logger::init();

    let fetcher_clone = Arc::clone(&fetcher);
    actix_web::rt::spawn(async move {
        fetcher_clone.update_loop().await;
    });
    info!("worker init done");

    start_server(fetcher).await.unwrap();
}
