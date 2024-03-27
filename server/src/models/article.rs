use serde::{Serialize, Deserialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Article {
    pub id:Option<i32>,
    pub date:Option<chrono::NaiveDateTime>,
    pub title:String,
    pub content:Option<String>,
}