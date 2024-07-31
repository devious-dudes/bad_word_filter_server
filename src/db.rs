// src/db.rs

use futures_util::stream::StreamExt;
use mongodb::{Client, options::ClientOptions};
use serde::Deserialize;
use crate::nlp_processing::process_single_word;

#[derive(Deserialize)]
struct BadWord {
  word: String,
}

pub async fn load_bad_words(client: &Client, db_name: &str) -> Vec<String> {
  let db = client.database(db_name);
  let collection = db.collection::<BadWord>("badwords");
  let mut cursor = collection.find(None, None).await.unwrap();
  let mut bad_words = Vec::new();
  while let Some(result) = cursor.next().await {
    match result {
      Ok(doc) => {
        if let Some(processed_word) = process_single_word(&doc.word) {
          bad_words.push(processed_word);
        }
      }
      Err(e) => eprintln!("Error fetching document: {:?}", e),
    }
  }
  bad_words
}

pub async fn get_mongo_client(uri: &str) -> Client {
  let client_options = ClientOptions::parse(uri).await.unwrap();
  Client::with_options(client_options).unwrap()
}

