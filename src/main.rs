use actix_web::{App, HttpServer};
use actix_files as fs;

mod routes;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    bunt::println!("Starting server at {$blue}http://127.0.0.1:8080{/$}!");
    HttpServer::new(|| {
        App::new()
            .service(routes::common::home)
            .service(routes::common::all)
            .service(routes::common::add)
            .service(routes::common::del)
            .service(routes::common::edit)
            .service(routes::common::complete)
            .service(fs::Files::new("/static/", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}