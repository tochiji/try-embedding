use reqwest::blocking::{get, Response};
use select::document::Document;
use select::predicate::{Class, Name};
use serde::Serialize;

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut v = vec![];

    for page in 1..=20 {
        eprintln!("page: {}", page);

        let response = fetch_page(page)?;
        let document = Document::from_read(response)?;

        for post_box in document.find(Class("post-box")) {
            let mut title: String = "".to_string();
            let mut date: String = "".to_string();
            let mut url: String = "".to_string();

            if let Some(post_title) = post_box.find(Class("post-tit")).next() {
                title = post_title.text();
            }
            if let Some(post_date) = post_box.find(Class("post-date")).next() {
                date = post_date.text();
            }
            if let Some(post_url) = post_box.find(Class("more-link")).next() {
                url = post_url.attr("href").unwrap().to_string();
            }

            eprintln!("title: {}", title);
            let contents = fetch_contents(&url).unwrap();

            let item = Item::new(title, date, url, contents);
            v.push(item);
        }
    }

    let json = serde_json::to_string(&v).unwrap();
    std::fs::write("result.json", json)?;

    Ok(())
}

#[derive(Debug, Serialize)]
struct Item {
    title: String,
    post_date: String,
    url: String,
    contents: String,
}

impl Item {
    fn new(title: String, post_date: String, url: String, contents: String) -> Self {
        Self {
            title,
            post_date,
            url,
            contents,
        }
    }
}

fn fetch_page(page: i32) -> Result<Response, Box<dyn std::error::Error>> {
    let url = format!("https://anagrams.jp/blog/page/{page}");
    Ok(get(url)?)
}

fn fetch_contents(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = get(url)?;
    let document = Document::from_read(response)?;

    let mut result_contents = String::new();

    if let Some(contents) = document.find(Class("toc_list")).next() {
        for li in contents.find(Name("li")) {
            let text = li.text().replace('\n', "");
            if text == *"まとめ" {
                continue;
            }
            result_contents += &text;
            result_contents += "\n";
        }
    }

    Ok(result_contents)
}
