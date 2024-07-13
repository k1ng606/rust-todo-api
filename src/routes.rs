use actix_web::{HttpResponse, post, Responder, web};
use serde::{Deserialize, Serialize};
use crate::dto::item::item::Item;
use crate::persistence::todo_persistence::todo_persistence::{complete_item, create_item};

#[post("/item")]
async fn post_item(req_body: web::Json<Item>) -> impl Responder {
    let item = req_body.into_inner();

    match create_item(item) {
        Ok(created_item_id) => HttpResponse::Created().body(created_item_id.to_string()),
        Err(e) => {
            eprintln!("Failed to create item: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create item")
        }
    }
}

#[post("/item/complete")]
async fn done_item(req_body: web::Json<CompleteItemRequest>) -> impl Responder {
    let item_to_complete_id: i64 = match req_body.item_id.parse::<i64>() {
        Ok(parsed) => { parsed }
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid input");
        }
    };
    match complete_item(item_to_complete_id) {
        Ok(_) => HttpResponse::Ok().body("Completed"),
        Err(e) => {
            println!("{}",e);
            HttpResponse::InternalServerError().body("Failed to complete item")
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CompleteItemRequest {
    item_id: String
}