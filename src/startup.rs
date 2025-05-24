use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use sqlx::PgConnection;
use std::net::TcpListener;

use crate::{health_check, subscribe};

pub fn run(listener: TcpListener,
           connection: PgConnection) -> Result<Server, std::io::Error>{
    // 智能指针包装一个连接
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
          .route("/health_check", web::get().to(health_check))
          .route("/subscriptions", web::post().to(subscribe))
          // 获取连接的副本绑定到应用程序
          .app_data(connection.clone())
        })
        .listen(listener)?
        .run();
    Ok(server)
}