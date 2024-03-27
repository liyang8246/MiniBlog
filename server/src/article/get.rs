use std::sync::{Arc, Mutex};

use ntex::web::types::{Path, State, Json};
use crate::{AppState, models::article::Article, errors::CustomError};

pub async fn article(
    id: Path<i32>,
    state: State<Arc<Mutex<AppState>>>,
) -> Result<Json<Option<Article>>, CustomError> {
    let sql = &state.lock().unwrap().db_pool;
    let mut articles = sql.prepare(&format!("select id,date,title,content from articles where id={}",id.clone())).unwrap();
    let articles = articles.query_map([], |row|{
        Ok(Article{
            id:Some(row.get(0).unwrap()),
            date:Some(row.get(1).unwrap()),
            title:row.get(2).unwrap(),
            content:Some(row.get(3).unwrap()),
        })
    }).unwrap();
    let article = articles.map(|x| x.unwrap()).next();
    println!("{}", format!("select id,date,title,content from articles where id={}",id.into_inner()));
    Ok(Json(article))
}