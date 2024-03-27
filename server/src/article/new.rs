use std::sync::{Arc, Mutex};
use ntex::web::types::{State, Json};
use crate::{AppState, models::article::Article, errors::CustomError};

pub async fn article(
    json:Json<Article>,
    state: State<Arc<Mutex<AppState>>>,
) -> Result<String, CustomError> {
    if json.content.is_none() {
        return Err(CustomError::InternalError("未传入文章标题".into()));
    }
    let sql = &state.lock().unwrap().db_pool;
    sql.execute(
        "insert into articles (title,content) values (?1,?2)",
        (&json.title,&json.content.as_ref().unwrap()),
    ).unwrap();
    Ok("Successfully added".to_string())
}
