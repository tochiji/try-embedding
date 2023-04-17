use blog_scraper::{embed::fetch_embeddings, types::Item};

fn main() {
    // 第一引数をStringとして取得
    let args: Vec<String> = std::env::args().collect();
    let keyword = &args[1];

    // keywordが空ならば終了
    if keyword.is_empty() {
        eprintln!("Please input keyword");
        std::process::exit(1);
    }

    match run(keyword) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}

fn run(word: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 同じフォルダの result.jsonを読み込む
    let file = std::fs::File::open("result2.json")?;
    let reader = std::io::BufReader::new(file);
    let items: Vec<Item> = serde_json::from_reader(reader)?;

    let embed = fetch_embeddings(word)?;
    let base_embed = embed.data[0].embedding.clone();

    let mut ranks = vec![];

    for item in items.iter() {
        let title = item.title.to_string();
        let url = item.url.to_string();

        let embed = item.embedding.clone().unwrap();
        let cos_sim = cos_sim(&base_embed, &embed);
        let rank = (title, url, cos_sim);
        ranks.push(rank);
    }

    // cos_simの値でソート
    ranks.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    // 上位10件を表示
    for rank in ranks.iter().take(10) {
        println!("Title: {}", rank.0);
        println!("URL: {}", rank.1);
        println!("Cosine Similarity: {}", rank.2);
        println!("");
    }

    Ok(())
}

// ベクトル同士のコサイン類似度を計算する関数
fn cos_sim(v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
    let mut sum = 0.0;
    let mut v1_sum = 0.0;
    let mut v2_sum = 0.0;
    for i in 0..v1.len() {
        sum += v1[i] * v2[i];
        v1_sum += v1[i] * v1[i];
        v2_sum += v2[i] * v2[i];
    }
    sum / (v1_sum.sqrt() * v2_sum.sqrt())
}
