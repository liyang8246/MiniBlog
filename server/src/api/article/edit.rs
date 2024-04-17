use crate::{errors::CustomError, models::article::Article, State};
use ntex::web::types::{Json, State};
use std::sync::{Arc, Mutex};

pub async fn article(
    json: Json<Article>,
    state: State<Arc<Mutex<State>>>,
) -> Result<String, CustomError> {
    if json.id.is_none() {
        return Err(CustomError::InternalError("未传入文章id".into()));
    }
    if json.content.is_none() {
        return Err(CustomError::InternalError("未传入文章标题".into()));
    }
    let sql = &state.lock().unwrap().db_pool;
    if let Err(err_code) = sql.execute(
        "update articles title=?1,content=?2",
        (&json.title, &json.content.as_ref().unwrap()),
    ) {
        return Err(CustomError::InternalError(
            format!("数据库错误: {err_code}").into(),
        ));
    }
    Ok("Successfully edited".to_string())
}
