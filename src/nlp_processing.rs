// src/nlp_processing.rs

use stopwords::{Stopwords, Language, Spark};
use rust_stemmers::{Algorithm, Stemmer};

pub fn process_text(text: &str) -> Vec<String> {
  let normalized_text = text.to_lowercase();
  let words: Vec<&str> = normalized_text.split_whitespace().collect();

  let stopwords = Spark::stopwords(Language::English).unwrap();
  let filtered_words: Vec<&str> = words.into_iter()
    .filter(|word| !stopwords.contains(word))
    .collect();

  let en_stemmer = Stemmer::create(Algorithm::English);
  let stemmed_words: Vec<String> = filtered_words.into_iter()
    .map(|word| en_stemmer.stem(word).to_string())
    .collect();

  // Generate bigrams and trigrams
  let mut phrases = vec![];
  for i in 0..stemmed_words.len() {
    phrases.push(stemmed_words[i].clone());
    if i + 1 < stemmed_words.len() {
      phrases.push(format!("{} {}", stemmed_words[i], stemmed_words[i + 1]));
    }
    if i + 2 < stemmed_words.len() {
      phrases.push(format!("{} {} {}", stemmed_words[i], stemmed_words[i + 1], stemmed_words[i + 2]));
    }
  }
  phrases
}

pub fn process_single_word(word: &str) -> Option<String> {
  let processed_words = process_text(word);
  processed_words.get(0).cloned()
}
