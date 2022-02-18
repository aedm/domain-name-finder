use crate::database_reader::DbEntry;
use crate::Database;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SearchInput {
    pub words: Vec<String>,
}

#[derive(Serialize)]
pub struct SearchResult {
    pub free: Vec<String>,
    pub reserved: Vec<String>,
}

pub fn search(input: &SearchInput, db: &Database) -> SearchResult {
    let words = &input.words;
    let mut free = vec![];
    let mut reserved = vec![];
    for w1 in words {
        for w2 in words {
            if w1 != w2 {
                let term = format!("{}{}", w1, w2);
                let db_entry = DbEntry::from(term.clone());
                if db.contains(&db_entry) {
                    reserved.push(term);
                } else {
                    free.push(term);
                }
            }
        }
    }
    SearchResult { free, reserved }
}
