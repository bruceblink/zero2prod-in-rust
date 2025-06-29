use std::net::TcpListener;
use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
use crate::email_client::EmailClient;

use crate::routes::{health_check, subscribe};

pub fn run(listener: TcpListener,
           db_pool: PgPool,
           email_client: EmailClient
) -> Result<Server, std::io::Error>{
    // 智能指针包装一个连接
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // 获取连接的副本绑定到应用程序
            .app_data(db_pool.clone())
            .app_data(email_client.clone())   //传递email_client的clone
        })
        .listen(listener)?
        .run();
    Ok(server)
}