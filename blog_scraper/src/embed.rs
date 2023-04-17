use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};

use crate::types::{RequestBody, ResponseBody};

pub fn fetch_embeddings(s: &str) -> Result<ResponseBody, Box<dyn std::error::Error>> {
    let api_key = env!("OPENAI_API_KEY");
    let url = "https://api.openai.com/v1/embeddings";

    let request_body = RequestBody {
        input: s.to_string(),
        model: "text-embedding-ada-002".to_string(),
    };

    let client = reqwest::blocking::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", api_key).parse()?);
    headers.insert(CONTENT_TYPE, "application/json".parse()?);

    let response = client
        .post(url)
        .headers(headers)
        .json(&request_body)
        .send()?
        .json::<ResponseBody>()?;

    Ok(response)
}
