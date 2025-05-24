pub mod configuration;
pub mod routes;
pub mod startup;

use std::net::TcpListener;
use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::dev::Server;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// 定义表单数据结构体
#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

// 总是返回200
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
