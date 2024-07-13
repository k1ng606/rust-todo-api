use actix_web::{App, HttpServer};

use crate::routes::{done_item, get_all, post_item};
use crate::persistence::todo_persistence::todo_persistence::init_db;

mod dto;
mod routes;
mod persistence;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match init_db() {
        Ok(_) => {}
        Err(e) => {println!("{}", e)}
    }

    HttpServer::new(|| {
        App::new()
            .service(post_item)
            .service(done_item)
            .service(get_all)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

