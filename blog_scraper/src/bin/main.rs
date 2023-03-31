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
    let mut v = vec![];

    for page in 1..=10 {
        let response = fetch_url(page)?;
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

            let item = Item::new(title, date, url);
            v.push(item);
        }
    }

    println!("{:#?}", v);
    Ok(())
}

#[derive(Debug)]
struct Item {
    title: String,
    post_date: String,
    url: String,
}

impl Item {
    fn new(title: String, post_date: String, url: String) -> Self {
        Self {
            title,
            post_date,
            url,
        }
    }
}

fn fetch_url(page: i32) -> Result<Response, Box<dyn std::error::Error>> {
    let url = format!("https://anagrams.jp/blog/page/{page}");
    Ok(get(url)?)
}
