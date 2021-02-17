extern crate reqwest;

use anyhow::Result;
use csv;
use md5;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{LineWriter, Write};
use std::time::Duration;
use tokio::time::delay_for;

/// the Tureng seach api endpoint
const URL: &str = "http://ws.tureng.com/TurengSearchServiceV4.svc/Search";
/// the salt to be appended to the tokenized Tureng query term
const SALT: &str = "46E59BAC-E593-4F4F-A4DB-960857086F9C";

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct QueryRequestBody {
    term: String,
    code: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ResultItem {
    #[serde(rename = "CategoryEN")]
    category_en: Option<String>,
    #[serde(rename = "CategoryTR")]
    category_tr: Option<String>,
    term: String,
    #[serde(rename = "TypeEN")]
    type_en: Option<String>,
    #[serde(rename = "TypeTR")]
    type_tr: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct MobileResult {
    is_found: u32,
    #[serde(rename = "IsTRToEN")]
    is_tr_to_en: u32,
    results: Vec<ResultItem>,
    suggestions: Vec<String>,
    term: String,
    #[serde(rename = "VoiceURLs")]
    voice_urls: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct QueryResponseBody {
    exception_message: Option<String>,
    is_successful: bool,
    mobile_result: MobileResult,
}

fn get_token(query: &str) -> String {
    let a = md5::compute(format!("{}{}", query, SALT));
    format!("{:x}", a)
}

async fn make_query(query: &str) -> std::result::Result<QueryResponseBody, reqwest::Error> {
    let body = QueryRequestBody {
        term: query.to_string(),
        code: get_token(query),
    };
    let client = reqwest::Client::new();
    client
        .post(URL)
        .json(&body)
        .send()
        .await?
        .json::<QueryResponseBody>()
        .await
}

#[tokio::main]
async fn main() -> Result<()> {
    let queries = vec!["ev".to_string(), "-den yana".to_string()];
    let mut failed_queries = Vec::new();
    let csv_path = "./data/seeds/tureng.csv";
    let mut wtr = csv::Writer::from_path(csv_path)?;

    for q in queries {
        let res = make_query(&q).await;
        match res {
            Ok(res) => {
                wtr.serialize(res)?;
                delay_for(Duration::from_secs(5)).await;
            }
            Err(err) => {
                println!("{}", err);
                failed_queries.push(q);
                continue;
            }
        }
    }

    wtr.flush()?;

    Ok(())
}
