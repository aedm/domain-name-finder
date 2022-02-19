use actix_web::web::Data;
use smol_str::SmolStr;
use std::collections::{BTreeSet, HashSet};

pub const VALID_LETTERS: &str = "abcdefghijklmnopqrstuvwxz0123456789-";
pub const VALID_LETTERS_COUNT: usize = VALID_LETTERS.len();

pub const LETTER_INDEX: [usize; 256] = {
    let mut x = [0usize; 256];
    let mut i = 0usize;
    while i < VALID_LETTERS.len() {
        x[VALID_LETTERS.as_bytes()[i] as usize] = i;
        i += 1;
    }
    x
};

pub type DatabaseWords =
    [HashSet<SmolStr>; VALID_LETTERS_COUNT * VALID_LETTERS_COUNT * VALID_LETTERS_COUNT];

pub struct Database {
    pub words: Vec<HashSet<SmolStr>>,
}

impl Database {
    pub fn contains(&self, word: &str) -> bool {
        if word.len() < 3 {
            return true;
        }
        return false;
    }
}
