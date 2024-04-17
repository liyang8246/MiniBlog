use api::article::view::view;
use salvo::prelude::*;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::{env, sync::{Arc, Mutex}};
use dotenv::dotenv;

mod api;
mod models;

type State = Arc<Mutex<AppState>>;
pub struct AppState {
    pub db_pool: SqlitePool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap();
    let db_pool: State = Arc::new(Mutex::new(AppState {
        db_pool: SqlitePoolOptions::new()
            .max_connections(16)
            .connect(&db_url)
            .await.unwrap()
    }));

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    let router = Router::new().hoop(affix::inject(db_pool))
        .push(Router::with_path("api").get(view))
        .push(Router::with_path("<**path>").get(
            StaticDir::new(["../client",])
            .defaults("index.html")
            .auto_list(true)
        ));

    Server::new(acceptor).serve(router).await;
}
