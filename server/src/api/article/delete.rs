use crate::{errors::CustomError, State};
use ntex::web::types::{Path, State};
use std::sync::{Arc, Mutex};

pub async fn article(
    id: Path<i32>,
    state: State<Arc<Mutex<State>>>,
) -> Result<String, CustomError> {
    let sql = &state.lock().unwrap().db_pool;
    if let Err(err_code) = sql.execute("delete from articles where id=?1", [id.clone()]) {
        return Err(CustomError::InternalError(
            format!("数据库错误: {err_code}").into(),
        ));
    }
    Ok("Successfully edited".to_string())
}
