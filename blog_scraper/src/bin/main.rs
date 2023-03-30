use reqwest::blocking::{get, Response};
use select::document::Document;
use select::predicate::Class;

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let page = 1;
    let response = fetch_url(page)?;
    let document = Document::from_read(response)?;

    for post_box in document.find(Class("post-box")) {
        if let Some(post_title) = post_box.find(Class("post-tit")).next() {
            println!("Title: {}", post_title.text());
        }
        if let Some(post_date) = post_box.find(Class("post-date")).next() {
            println!("Date: {}", post_date.text());
        }
        if let Some(url) = post_box.find(Class("more-link")).next() {
            println!("URL: {}", url.attr("href").unwrap());
        }
        println!("---");
    }

    Ok(())
}

fn fetch_url(page: i32) -> Result<Response, Box<dyn std::error::Error>> {
    let url = format!("https://anagrams.jp/blog/page/{page}");
    Ok(get(url)?)
}
