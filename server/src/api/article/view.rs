// use crate::{errors::CustomError, models::article::Article, State};
// use ntex::web::types::{Json, State};
// use std::sync::{Arc, Mutex};
// pub async fn article(state: State<Arc<Mutex<State>>>) -> Result<Json<Vec<Article>>, CustomError> {
//     let sql = &state.lock().unwrap().db_pool;
//     let mut articles = sql
//         .prepare("select id,date,title,content from articles")
//         .unwrap();
//     let articles = articles
//         .query_map([], |row| {
//             Ok(Article {
//                 id: Some(row.get(0).unwrap()),
//                 date: Some(row.get(1).unwrap()),
//                 title: row.get(2).unwrap(),
//                 content: None,
//             })
//         })
//         .unwrap();
//     let articles = articles.map(|x| x.unwrap()).collect();
//     Ok(Json(articles))
// }
use salvo::prelude::*;
use crate::models::article::Article;
use crate::State;

#[handler]
pub async fn view(depot: &mut Depot, res: &mut Response) {
    let db_pool = depot.obtain::<State>().expect("get db_pool fail");
    let conn = db_pool.lock().unwrap().db_pool.clone();
    let articles:Vec<Article> = sqlx::query!("select * from articles")
        .fetch_all(&conn).await.unwrap().iter().map(|row|{
            Article { id: Some(row.id as i32),
                date: Some(row.date),
                title: row.title.clone().unwrap(),
                content: None,
            }
        }).collect();
    res.render(Json(articles));
}
