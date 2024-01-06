#[macro_use]
extern crate dotenv_codegen;

use std::error::Error;
use std::fs;
use serde::{Serialize, Deserialize};
use dotenv::dotenv;
use reqwest::header::HeaderMap;

#[derive(Serialize, Deserialize, Debug)]
struct Provider {
    uid: String,
    name: String,
    fqdn: String,
}

fn create_headers() -> HeaderMap {
    let mut bearer_header: String = Default::default();
    bearer_header.push_str("Bearer ");
    bearer_header.push_str(dotenv!("GRIST_BEARER"));

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Authorization", bearer_header.parse().unwrap());

    headers
}

async fn create_table(headers: HeaderMap, path: String, doc: String) {
    let file_contents = fs::read_to_string(path.clone()).expect("Unable to read file");
    let json_file_contents: Vec<Provider> = serde_json::from_str(&file_contents).unwrap();

    // create table
    let mut table_url: String = Default::default();
    table_url.push_str(dotenv!("GRIST_URL"));
    table_url.push_str(&doc);
    table_url.push_str("/tables");

    // add the table structure
    let mut table_structure_text: String = Default::default();
    table_structure_text.push_str(r#"{"tables":[{"id":""#);
    table_structure_text.push_str(&path);
    table_structure_text.push_str(r#"","columns":[{"id":"uid","fields":{"label":"uid"}},{"id":"name","fields":{"label":"name"}},{"id":"fqdn","fields":{"label":"fqdn"}}]}]}"#);
    let table_structure_json: serde_json::Value = serde_json::from_str(&table_structure_text).unwrap();

    let client = reqwest::Client::builder().build().unwrap();

    let table_request = client.request(reqwest::Method::POST, table_url)
        .headers(headers.clone())
        .json(&table_structure_json);

    let tables_id_text = table_request
        .send()
        .await
        .expect("failed to get response")
        .text()
        .await
        .expect("failed to get payload");

    let tables_id_json: serde_json::Value = serde_json::from_str(&tables_id_text).expect("Failed to parse JSON");
    let mut table_id: String = Default::default();

    if let Some(table) = tables_id_json["tables"].get(0) {
        if let Some(id) = table["id"].as_str() {
            table_id.push_str(id);
        } else {
            println!("No 'id' found or it's not a string");
        }
    } else {
        println!("No 'tables' array found or it's empty");
    }
    
    // add content to the table
    let mut records: String = Default::default();
    for (index, fqdn) in json_file_contents.iter().enumerate() {
        records.push_str(r#"{"id":"#);
        records.push_str(&index.to_string());
        records.push_str(r#", "fields":"#);

        records.push_str(&serde_json::to_string(&fqdn).unwrap());
        records.push_str(r#"}"#);

        if index != json_file_contents.len()-1 {
            records.push_str(", ");
        }
    }

    let prefix = r#"{ "records":["#;
    let suffix = r#"]}"#;
    
    let concatenation = format!("{prefix}{records}{suffix}");
    let json: serde_json::Value = serde_json::from_str(&concatenation).unwrap();

    let mut table_url: String = Default::default();
    table_url.push_str(dotenv!("GRIST_URL"));
    table_url.push_str(dotenv!("GRIST_DOC"));
    table_url.push_str("/tables/");
    table_url.push_str(&table_id);
    table_url.push_str("/records");

    println!("{}", &table_url);

    let content_request = client.request(reqwest::Method::POST, table_url)
        .headers(headers)
        .json(&json);

    content_request.send().await.unwrap();

    println!("Table has been updated.");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let headers: HeaderMap = create_headers();

    // for each file in folder, create table
    let paths = fs::read_dir(dotenv!("SOURCE_PATH")).unwrap();

    for path in paths {
        create_table(
            headers.clone(), 
            path.unwrap().path().display().to_string(), 
            dotenv!("GRIST_DOC").to_string()
        ).await;
    }

    Ok(())
}

