use anyhow::Ok;
use article::{delete, edit, get, new, view::{self}};
use ntex::web::{middleware, App, HttpServer, self};
use rusqlite::Connection;
use std::{
    env, sync::{Arc, Mutex}
};

mod article;
mod errors;
mod models;

#[derive(Debug)]
pub struct AppState {
    pub db_pool: Connection,
}

#[ntex::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    env::set_var("RUST_LOG", "ntex=info");
    env_logger::init();
    let db_url = env::var("DATABASE_URL").expect("Please set `DATABASE_URL`");
    println!("db: {db_url}");
    let conn = Connection::open(db_url).expect("open sqlite err");
    let app_state = Arc::new(Mutex::new(AppState {db_pool: conn}));

    HttpServer::new(move || {
        App::new()
            .state(Arc::clone(&app_state))
            .wrap(middleware::Logger::default())
            .configure(route)
    }).bind("127.0.0.1:8000")?.workers(48).run().await?;
    Ok(())
}

fn route(cfg:&mut web::ServiceConfig) {
    cfg.service(web::scope("/article")
        .route("", web::get().to(view::article))
        .route("/new", web::post().to(new::article))
        .route("/delete/{id}", web::put().to(delete::article))
        .route("/edit/{id}", web::put().to(edit::article))
        .route("/get/{id}", web::get().to(get::article))
    );
}