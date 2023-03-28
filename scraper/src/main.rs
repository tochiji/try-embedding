use reqwest::blocking::get;
use select::document::Document;
use select::predicate::Class;

fn main() {
    let url = "XXXX"; // ここにブログ記事のURLを入れてください
    let response = get(url).expect("Failed to fetch the HTML");

    let document = Document::from_read(response).expect("Failed to parse the HTML");

    let title = document
        .find(Class("post-ttl"))
        .next()
        .expect("Failed to find the title")
        .text();

    let content = document
        .find(Class("post-body")) // ここに本文を格納している要素のクラス名を入れてください
        .next()
        .expect("Failed to find the content element")
        .text();

    println!("Title: {}", title);
    println!("Content: {}", content);
}
