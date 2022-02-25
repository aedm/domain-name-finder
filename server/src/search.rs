use crate::Database;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SearchInput {
    pub words: Vec<String>,
}

#[derive(Deserialize)]
pub struct BatchLookupInput {
    pub words: Vec<String>,
}

#[derive(Serialize)]
pub struct SearchResult {
    pub free: Vec<String>,
    pub reserved: Vec<String>,
}

pub fn search(db: &Database, input: &SearchInput) -> SearchResult {
    let words = &input.words;
    let mut free = vec![];
    let mut reserved = vec![];
    for w1 in words {
        for w2 in words {
            if w1 != w2 {
                let term = format!("{}{}", w1, w2);
                if db.contains(&term) {
                    reserved.push(term);
                } else {
                    free.push(term);
                }
            }
        }
    }
    SearchResult { free, reserved }
}

pub fn batch_lookup(db: &Database, input: BatchLookupInput) -> SearchResult {
    let mut free = vec![];
    let mut reserved = vec![];
    for word in input.words {
        if db.contains(&word) {
            reserved.push(word);
        } else {
            free.push(word);
        }
    }
    SearchResult { free, reserved }
}
