use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct RequestBody {
    pub input: String,
    pub model: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseBody {
    pub data: Vec<Embedding>,
    pub model: String,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Embedding {
    pub embedding: Vec<f32>,
    pub index: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub total_tokens: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub title: String,
    pub post_date: String,
    pub url: String,
    pub contents: String,
    pub embedding: Option<Vec<f32>>,
    pub model: Option<String>,
}
