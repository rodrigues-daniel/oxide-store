use actix_web::{HttpResponse, web};
use serde::Serialize;

#[derive(Serialize)]
struct MessageTest {
    message: String,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/", web::get().to(get_page))
            .route("/new", web::post().to(new_post))
            .route("/{id}", web::put().to(page_update)),
    );
}

async fn get_page() -> HttpResponse {
    let res = MessageTest {
        message: "Index Page".to_string(),
    };
    HttpResponse::Ok().json(res)
}

async fn new_post() -> HttpResponse {
    let res = MessageTest {
        message: "New Page Created".to_string(),
    };
    HttpResponse::Accepted().json(res)
}

async fn page_update() -> HttpResponse {
    let res = MessageTest {
        message: "New Page Updated!".to_string(),
    };
    HttpResponse::Accepted().json(res)
}