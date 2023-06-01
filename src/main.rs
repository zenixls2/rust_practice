use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Parser)]
#[command(author, version, about, long_about = None)]
struct Image {
    #[arg(short, long)]
    prompt: String,
    #[arg(short, default_value_t = 1)]
    n: i32,
    #[arg(short, long, default_value_t = String::from("512x512"))]
    size: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Data {
    url: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Output {
    created: i64,
    data: Vec<Data>,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth: String;
    if let Ok(val) = std::env::var("AUTH_KEY") {
        auth = val.to_string();
    } else {
        return Err("No AUTH_KEY exists".into());
    }
    let args = Image::parse();
    let client = awc::Client::builder()
        .add_default_header(("Accept-Encoding", "gzip, deflate"))
        .add_default_header((
            "User-Agent",
            "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/113.0",
        ))
        .disable_timeout()
        .finish();
    let result = client
        .post("http://chat1.manongzyg.one/api/openai/v1/images/generations")
        .bearer_auth(auth)
        .insert_header(("Content-Type", "application/json"))
        .send_json(&args)
        .await?
        .json::<Output>()
        .await?;

    println!("Response: {} {:?}", result.created, result.data);
    let image: Vec<u8> = client
        .get(&result.data[0].url)
        .insert_header(("Content-Type", "image/png"))
        .send()
        .await?
        .body()
        .await?
        .to_vec();
    let mut f = File::create("out.png")?;
    f.write_all(&image)?;
    f.sync_all()?;
    Ok(())
}
