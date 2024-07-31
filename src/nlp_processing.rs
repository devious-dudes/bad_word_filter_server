// src/nlp_processing.rs

use stopwords::{Stopwords, Language, Spark};
use rust_stemmers::{Algorithm, Stemmer};

pub fn process_text(text: &str) -> Vec<String> {
    // Normalize text
    let normalized_text = text.to_lowercase();
    
    // Split text into words
    let words: Vec<&str> = normalized_text.split_whitespace().collect();
    
    // Remove stop words
    let stopwords = Spark::stopwords(Language::English).unwrap();
    let filtered_words: Vec<&str> = words.into_iter()
        .filter(|word| !stopwords.contains(word))
        .collect();
    
    // Stem words
    let en_stemmer = Stemmer::create(Algorithm::English);
    let stemmed_words: Vec<String> = filtered_words.into_iter()
        .map(|word| en_stemmer.stem(word).to_string())
        .collect();

    stemmed_words
}

pub fn process_single_word(word: &str) -> Option<String> {
  let processed_words = process_text(word);
  processed_words.get(0).cloned()
}

