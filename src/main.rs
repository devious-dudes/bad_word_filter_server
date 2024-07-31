// src/main.rs 

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use clap::{Parser};
use daemonize::Daemonize;
use std::fs::File;
use std::sync::{Arc, RwLock};
use mongodb::Client;
use serde::Deserialize;
use serde_json::json;
use sysinfo::{System, SystemExt};
use crate::trie::Trie;
use crate::db::{load_bad_words, get_mongo_client};
use crate::nlp_processing::process_text;
use dotenv::dotenv;
use std::env;
use crate::middleware::AuthMiddleware;

mod trie;
mod db;
mod nlp_processing;
mod middleware;

struct AppState {
  trie: Arc<RwLock<Trie>>,
}

#[derive(Deserialize)]
struct Message {
  content: String,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  /// Host to bind to
  #[clap(long, default_value = "localhost")]
  host: String,

  /// Port to bind to
  #[clap(long, default_value_t = 8080)]
  port: u16,

  /// Run as a daemon
  #[clap(short, long)]
  daemon: bool,
}

async fn check_content(data: web::Data<AppState>, msg: web::Json<Message>) -> impl Responder {
  let processed_phrases = process_text(&msg.content);
  let trie = data.trie.read().unwrap();
  let violation = processed_phrases.iter().any(|phrase| trie.search(phrase));
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
    new_trie.insert(&word);
  }
  *data.trie.write().unwrap() = new_trie;
  "Trie reloaded"
}

async fn health(data: web::Data<AppState>) -> impl Responder {
  let trie = data.trie.read().unwrap();
  let word_count = trie.count_words();

  let mut sys = System::new_all();
  sys.refresh_all();
  let memory_used = sys.used_memory();
  let total_memory = sys.total_memory();

  HttpResponse::Ok().json(json!({
    "status": "ok",
    "word_count": word_count,
    "memory_used_kb": memory_used,
    "total_memory_kb": total_memory,
  }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  let args = Args::parse();

  if args.daemon {
    let stdout = File::create("/tmp/daemon.out").unwrap();
    let stderr = File::create("/tmp/daemon.err").unwrap();

    let daemonize = Daemonize::new()
      .stdout(stdout)
      .stderr(stderr);

    match daemonize.start() {
      Ok(_) => println!("Daemonized successfully"),
      Err(e) => eprintln!("Error, {}", e),
    }
  }

  let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set in .env file or as an environment variable");
  let db_name = env::var("MONGO_DBNAME").expect("MONGO_DBNAME must be set in .env file or as an environment variable");
  let bearer_token = env::var("BEARER_TOKEN").ok();

  let client = get_mongo_client(&mongo_uri).await;
  let bad_words = load_bad_words(&client, &db_name).await;
  let mut initial_trie = Trie::new();
  for word in bad_words {
      initial_trie.insert(&word);
  }

  let app_state = web::Data::new(AppState {
      trie: Arc::new(RwLock::new(initial_trie)),
  });

  let db_name_clone = db_name.clone();
  HttpServer::new(move || {
    let db_name_clone = db_name_clone.clone();
    App::new()
      .wrap(AuthMiddleware::new(bearer_token.clone()))
      .app_data(app_state.clone())
      .app_data(web::Data::new(client.clone()))
      .route("/check", web::post().to(check_content))
      .route("/reload", web::post().to(move |data, client| {
        reload_trie(data, client, db_name_clone.clone())
      }))
      .route("/health", web::get().to(health))
  })
  .bind((args.host.as_str(), args.port))?
  .run()
  .await
}
