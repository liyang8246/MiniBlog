use std::sync::{Arc, Mutex};
use ntex::web::types::{State, Json};
use crate::{AppState, models::article::Article, errors::CustomError};

pub async fn article(
    state: State<Arc<Mutex<AppState>>>,
) -> Result<Json<Vec<Article>>, CustomError> {
    let sql = &state.lock().unwrap().db_pool;
    let mut articles = sql.prepare("select id,date,title,content from articles").unwrap();
    let articles = articles.query_map([], |row|{
        Ok(Article{
            id:Some(row.get(0).unwrap()),
            date:Some(row.get(1).unwrap()),
            title:row.get(2).unwrap(),
            content:None,
        })
    }).unwrap();
    let articles = articles.map(|x| x.unwrap()).collect();
    Ok(Json(articles))
}