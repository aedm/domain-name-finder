use crate::Database;
use anyhow::{anyhow, Result};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};

#[derive(Deserialize, Debug)]
pub struct SearchInput {
    pub words: Vec<String>,
    pub prefixes: Vec<String>,
    pub postfixes: Vec<String>,
    #[serde(rename = "minWordCount")]
    pub min_word_count: usize,
    #[serde(rename = "maxWordCount")]
    pub max_word_count: usize,
}

#[derive(Serialize, Debug)]
pub struct SearchResult {
    pub free: Vec<String>,
    pub reserved: Vec<String>,
}

// pub enum HighlightReason {
//     SYNONYM(String),
//     PLURAL,
// }
//
// pub struct Highlight {
//     start_pos: usize,
//     end_pos: usize,
//     reason: HighlightReason,
// }
//
// pub struct ResultItem {
//     word: String,
//     is_free: bool,
//     highlights: Vec<Highlight>,
// }

#[derive(Serialize, Deserialize)]
pub struct BatchLookupInput {
    pub words: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BatchLookupResponse {
    pub is_free: HashMap<String, bool>,
}

pub async fn search(db: Option<&Database>, input: &SearchInput) -> Result<SearchResult> {
    let mut search_terms = HashSet::<String>::new();

    // Single word
    if input.min_word_count <= 1 && input.max_word_count >= 1 {
        search_terms.extend(input.prefixes.iter().cloned());
        search_terms.extend(input.words.iter().cloned());
        search_terms.extend(input.postfixes.iter().cloned());
    }

    if input.max_word_count >= 2 {
        let prefixes =
            HashSet::<&String>::from_iter(input.prefixes.iter().chain(input.words.iter()));
        let postfixes =
            HashSet::<&String>::from_iter(input.postfixes.iter().chain(input.words.iter()));
        // let words = HashSet::<&String>::from_iter(input.words.iter());

        // Two words
        if input.min_word_count <= 2 {
            for w1 in &prefixes {
                for w2 in &postfixes {
                    if w1 != w2 {
                        search_terms.insert(format!("{w1}{w2}"));
                    }
                }
            }
        }

        // Three words
        if input.min_word_count <= 3 && input.max_word_count >= 3 {
            for pre in &prefixes {
                for word in &input.words {
                    if pre != &word {
                        for post in &postfixes {
                            if pre != post && &word != post {
                                search_terms.insert(format!("{pre}{word}{post}"));
                            }
                        }
                    }
                }
            }
        }
    }

    let mut terms = search_terms.into_iter().collect_vec();
    println!("Word count: {}", terms.len());

    let lookup_result = lookup(db, terms).await?;

    let mut free = vec![];
    let mut reserved = vec![];
    for (word, is_free) in lookup_result {
        if is_free {
            free.push(word);
        } else {
            reserved.push(word);
        }
    }
    Ok(SearchResult { free, reserved })
}

pub fn batch_lookup(db: &Database, input: BatchLookupInput) -> BatchLookupResponse {
    BatchLookupResponse {
        is_free: lookup_local(db, input.words),
    }
}

async fn lookup(db: Option<&Database>, terms: Vec<String>) -> Result<HashMap<String, bool>> {
    if let Some(database) = db {
        Ok(lookup_local(database, terms))
    } else {
        lookup_proxy(terms).await
    }
}

fn lookup_local(db: &Database, words: Vec<String>) -> HashMap<String, bool> {
    words
        .into_iter()
        .map(|word| {
            let is_free = !db.contains(&word);
            (word, is_free)
        })
        .collect()
}

async fn lookup_proxy(words: Vec<String>) -> Result<HashMap<String, bool>> {
    let input = BatchLookupInput { words };
    let mut client = awc::Client::default();
    let mut request = client
        .post("http://localhost:9000/api/batch-lookup")
        .send_json(&input)
        .await
        .map_err(|err| anyhow!("Can't send request: {err:?}"))?;
    let response = request
        .json::<BatchLookupResponse>()
        .await
        .map_err(|err| anyhow!("Batch request error: {err:?}"))?;
    Ok(response.is_free)
}
