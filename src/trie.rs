// src/trie.rs

use std::collections::HashMap;

#[derive(Default)]
pub struct TrieNode {
  children: HashMap<String, TrieNode>,
  is_end_of_word: bool,
}

#[derive(Default)]
pub struct Trie {
  root: TrieNode,
}

impl Trie {
  pub fn new() -> Self {
    Trie {
      root: TrieNode::default(),
    }
  }

  pub fn insert(&mut self, phrase: &str) {
    let mut node = &mut self.root;
    for word in phrase.split_whitespace() {
      node = node.children.entry(word.to_string()).or_insert_with(TrieNode::default);
    }
    node.is_end_of_word = true;
  }

  pub fn search(&self, phrase: &str) -> bool {
    let mut node = &self.root;
    for word in phrase.split_whitespace() {
      if let Some(next_node) = node.children.get(word) {
        node = next_node;
      } else {
        return false;
      }
    }
    node.is_end_of_word
  }

  pub fn count_words(&self) -> usize {
    self.count_words_recursive(&self.root)
  }

  fn count_words_recursive(&self, node: &TrieNode) -> usize {
    let mut count = if node.is_end_of_word { 1 } else { 0 };
    for child in node.children.values() {
      count += self.count_words_recursive(child);
    }
    count
  }
}
