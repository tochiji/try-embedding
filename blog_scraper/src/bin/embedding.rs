use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct RequestBody {
    input: String,
    model: String,
}

#[derive(Deserialize)]
struct ResponseBody {
    data: Vec<Embedding>,
    model: String,
    usage: Usage,
}

#[derive(Deserialize)]
struct Embedding {
    embedding: Vec<f32>,
    index: usize,
}

#[derive(Deserialize)]
struct Usage {
    prompt_tokens: usize,
    total_tokens: usize,
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let embed = fetch_embeddings("Hello, world!")?;
    println!("{:?}", embed);
    Ok(())
}

fn fetch_embeddings(s: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
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

    Ok(response.data[0].embedding.clone())
}
