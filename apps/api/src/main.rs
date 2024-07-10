mod database;
mod entity;

use std::{env, io};
use actix_web::{HttpServer, App, HttpRequest, HttpResponse, get, post};
use serde::Serialize;
use sea_orm::{EntityTrait, ActiveModelTrait, ActiveValue::Set};

use database::{init_session, DATABASE_SESSION, DATABASE_READ_SESSION};
use entity::sample::{Entity as Sample, ActiveModel as SampleModel};

#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
struct SampleJson {
    id: i32,
    name: String,
}

#[get("/")]
async fn index(req: HttpRequest) -> HttpResponse {
    if let Some(remote_user) = req.headers().get("Remote-User") {
        return HttpResponse::Ok().body(format!("Hello, {}!", remote_user.to_str().unwrap()));
    }
    HttpResponse::Ok().body("Hello, random person!")
}

#[get("/ok")]
async fn ok() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

#[get("/api/v1/samples")]
async fn get_samples() -> HttpResponse {
    let db = DATABASE_READ_SESSION.get().expect("Database session not initialized");
    let samples = Sample::find().all(&db.connection).await.unwrap()
        .into_iter().map(|s| SampleJson { id: s.id, name: s.name }).collect::<Vec<SampleJson>>();
    let json = serde_json::to_string(&samples).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(json)
}

#[post("/api/v1/samples")]
async fn post_samples() -> HttpResponse {
    let db = DATABASE_SESSION.get().expect("Database session not initialized");
    let name = "test sample".to_string();
    let sample = SampleModel {
        name: Set(name.clone()),
        ..Default::default()
    };
    match sample.save(&db.connection).await {
        Ok(s) => {
            let id = s.id.clone().take().unwrap();
            let name = s.name.clone().take().unwrap();
            let json = serde_json::to_string(&SampleJson { id: id, name: name }).unwrap();
            HttpResponse::Ok()
                .content_type("application/json")
                .body(json)
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to save sample: {:?}", err))
        }
    }
}

#[actix_web::main]
async fn main() -> Result<(), io::Error> {
    let ip_address = env::var("BOUND_IP_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
    let p = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port = u16::from_str_radix(&p, 10).unwrap();
    init_session().await.unwrap();

    println!("Starting server on port {}", port);

    HttpServer::new(|| {
        App::new()
            .service(ok)
            .service(index)
            .service(get_samples)
            .service(post_samples)
    })
    .bind((ip_address, port))?
    .run()
    .await
}
