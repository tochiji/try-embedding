use blog_scraper::{embed::fetch_embeddings, types};

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // 同じフォルダの result.jsonを読み込む
    let file = std::fs::File::open("result.json")?;
    let reader = std::io::BufReader::new(file);
    let items: Vec<types::Item> = serde_json::from_reader(reader)?;

    let mut outputs = vec![];

    for item in items.iter() {
        let word = format!("Title:{} Contents:{}", item.title, item.contents);
        let embed = fetch_embeddings(&word)?;
        let new_item = types::Item {
            title: item.title.to_string(),
            post_date: item.post_date.to_string(),
            url: item.url.to_string(),
            contents: item.contents.to_string(),
            embedding: Some(embed.data[0].embedding.clone()),
            model: Some(embed.model),
        };
        outputs.push(new_item);
    }

    // outputをJSONにして"result2.json"として書き出す
    let json = serde_json::to_string(&outputs).unwrap();
    std::fs::write("result2.json", json)?;

    Ok(())
}
