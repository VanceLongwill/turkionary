extern crate chrono;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use sqlx::PgPool;

mod intl;
/// mod definition;
mod term;
pub mod conf;

#[actix_rt::main]
async fn main() -> Result<()> {
    let conf = conf::Conf::new();
    let db_pool = PgPool::new(&conf.database_url).await?;

    let mut server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone()) // pass database pool to application so we can access it inside handlers
            .configure(term::init) // init todo routes
    });

    server = server.bind(format!("{}:{}", conf.host, conf.port))?;

    println!("Starting server");
    server.run().await?;

    Ok(())
}
