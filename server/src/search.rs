use crate::Database;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;

#[derive(Deserialize)]
pub struct SearchInput {
    pub words: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BatchLookupInput {
    pub words: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    pub free: Vec<String>,
    pub reserved: Vec<String>,
}

async fn look_up_terms(db: Option<&Database>, terms: Vec<String>) -> Result<SearchResult> {
    if let Some(database) = db {
        let mut free = vec![];
        let mut reserved = vec![];
        for term in terms {
            if database.contains(&term) {
                reserved.push(term);
            } else {
                free.push(term);
            }
        }
        Ok(SearchResult { free, reserved })
    } else {
        let input = BatchLookupInput { words: terms };
        let mut client = awc::Client::default();
        let mut response = client
            .post("http://localhost:9000/api/batch-lookup")
            .send_json(&input)
            .await
            .map_err(|err| anyhow!("send"))?;
        response
            .json::<SearchResult>()
            .await
            .map_err(|err| anyhow!("Batch request error: {err:?}"))
    }
}

pub async fn search(db: Option<&Database>, input: &SearchInput) -> Result<SearchResult> {
    let words = &input.words;
    let mut terms = vec![];
    for w1 in words {
        for w2 in words {
            if w1 != w2 {
                terms.push(format!("{w1}{w2}"));
            }
        }
    }
    look_up_terms(db, terms).await
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
