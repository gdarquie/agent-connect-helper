#[macro_use]
extern crate dotenv_codegen;

use std::error::Error;
use std::fs;
use serde::{Serialize, Deserialize};
use dotenv::dotenv;

#[derive(Serialize, Deserialize, Debug)]
struct Provider {
    uid: String,
    name: String,
    fqdn: String,
}

// fn create_header() -> str {
//     let mut bearer_header: String = Default::default();
//     bearer_header.push_str("Bearer ");
//     bearer_header.push_str(dotenv!("GRIST_BEARER"));

//     let mut headers = reqwest::header::HeaderMap::new();
//     headers.insert("Content-Type", "application/json".parse()?);
//     headers.insert("Authorization", bearer_header.parse()?);

//     headers
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let mut bearer_header: String = Default::default();
    bearer_header.push_str("Bearer ");
    bearer_header.push_str(dotenv!("GRIST_BEARER"));

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);
    headers.insert("Authorization", bearer_header.parse()?);

    // get data from a file
    let mut path: String = Default::default();
    path.push_str(dotenv!("SOURCE_PATH"));

    let file_contents = fs::read_to_string(path).expect("Unable to read file");
    let json_file_contents: Vec<Provider> = serde_json::from_str(&file_contents)?;

    let mut records: String = Default::default();
    for (index, fqdn) in json_file_contents.iter().enumerate() {
        records.push_str(r#"{"id":"#);
        records.push_str(&index.to_string());
        records.push_str(r#", "fields":"#);

        records.push_str(&serde_json::to_string(&fqdn)?);
        records.push_str(r#"}"#);

        if index != json_file_contents.len()-1 {
            records.push_str(", ");
        }
    }

    let prefix = r#"{ "records":["#;
    let suffix = r#"]}"#;
    
    let concatenation = format!("{prefix}{records}{suffix}");
    let json: serde_json::Value = serde_json::from_str(&concatenation)?;

    let client = reqwest::Client::builder().build()?;

    let mut table_url: String = Default::default();
    table_url.push_str(dotenv!("GRIST_URL"));
    table_url.push_str(dotenv!("GRIST_WS"));
    table_url.push_str("/tables/");
    table_url.push_str(dotenv!("GRIST_TABLE"));
    table_url.push_str("/records");

    let request = client.request(reqwest::Method::POST, table_url)
        .headers(headers)
        .json(&json);

    request.send().await?;

    println!("Table has been updated.");

    Ok(())
}

