use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    HttpServer::new(||
            App::new().service(
                fs::Files::new("/statics", ".").index_file("index.html")
            )
    ).bind(("0.0.0.0", 8080))?
        .run()
        .await
}
