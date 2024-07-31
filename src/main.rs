// src/main.rs 

use actix_web::{web, App, HttpServer, Responder};
use std::sync::{Arc, RwLock};
use mongodb::Client;
use serde::Deserialize;
use crate::trie::Trie;
use crate::db::{load_bad_words, get_mongo_client};
use crate::nlp_processing::process_text;
use dotenv::dotenv;
use std::env;

mod trie;
mod db;
mod nlp_processing;

struct AppState {
  trie: Arc<RwLock<Trie>>,
}

#[derive(Deserialize)]
struct Message {
  content: String,
}

async fn check_content(data: web::Data<AppState>, msg: web::Json<Message>) -> impl Responder {
  let processed_words = process_text(&msg.content);
  let trie = data.trie.read().unwrap();
  let violation = processed_words.iter().any(|word| trie.search(word));
  if violation {
    "not ok"
  } else {
    "ok"
  }
}

async fn reload_trie(data: web::Data<AppState>, client: web::Data<Client>, db_name: String) -> impl Responder {
  let bad_words = load_bad_words(&client, &db_name).await;
  let mut new_trie = Trie::new();
  for word in bad_words {
    println!("readding word: {}", word);
    new_trie.insert(&word);
  }
  *data.trie.write().unwrap() = new_trie;
  "Trie reloaded"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set in .env file or as an environment variable");
  let db_name = env::var("MONGO_DBNAME").expect("MONGO_DBNAME must be set in .env file or as an environment variable");
    
  let client = get_mongo_client(&mongo_uri).await;
  let bad_words = load_bad_words(&client, &db_name).await;
  let mut initial_trie = Trie::new();
  for word in bad_words {
    println!("adding word: {}", word);
    initial_trie.insert(&word);
  }

  let app_state = web::Data::new(AppState {
    trie: Arc::new(RwLock::new(initial_trie)),
  });

  let db_name_clone = db_name.clone();
  HttpServer::new(move || {
    let db_name_clone = db_name_clone.clone();
    App::new()
      .app_data(app_state.clone())
      .app_data(web::Data::new(client.clone()))
      .route("/check", web::post().to(check_content))
      .route("/reload", web::post().to(move |data, client| {
        reload_trie(data, client, db_name_clone.clone())
      }))
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}

