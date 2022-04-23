use seq_macro::seq;
use std::collections::HashSet;

#[macro_export]
macro_rules! for_each_domain_length {
    ($l:tt) => {
        seq! {N in 0..65 $l}
    };
}

for_each_domain_length!({
    pub struct Database {
        #(
            pub words_~N: HashSet<[u8; N]>,
        )*
    }
});

impl Database {
    pub fn new() -> Database {
        for_each_domain_length!({
            Database {
                #(
                    words_~N: HashSet::new(),
                )*
            }
        })
    }

    pub fn contains(&self, word: &str) -> bool {
        for_each_domain_length!({
            match word.len() {
                #(
                    N => self.words_~N.contains::<[u8; N]>(&word.as_bytes()[0..N].try_into().unwrap()),
                )*
                _ => false,
            }
        })
    }
}
