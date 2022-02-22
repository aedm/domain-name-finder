use actix_web::web::Data;
use seq_macro::seq;
use smol_str::SmolStr;
use std::collections::{BTreeSet, HashSet};

seq!(N in 1..64 {
    pub struct Database {
        #(
            pub words_~N: HashSet<[u8; N]>,
        )*
    }
});

impl Database {
    pub fn new() -> Database {
        seq!(N in 1..64 {
            Database {
                #(
                    words_~N: HashSet::new(),
                )*
            }
        })
    }

    pub fn contains(&self, word: &str) -> bool {
        seq!(N in 1..64 {
            match word.len() {
                #(
                    N => self.words_~N.contains::<[u8; N]>(&word.as_bytes()[0..N].try_into().unwrap()),
                )*
                _ => false,
            }
        })
    }
}
