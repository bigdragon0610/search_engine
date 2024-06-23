use std::collections::{HashMap, HashSet};

use crate::{bigram, File};

pub type Dict = HashMap<String, HashSet<String>>;

pub fn create_dict(file: &File, dict: &mut Dict) {
    if let Some(bigram) = bigram::create_bigram(&file.content) {
        for word in bigram {
            dict.entry(word)
                .and_modify(|e| {
                    e.insert(file.name.clone());
                })
                .or_insert({
                    let mut set = HashSet::new();
                    set.insert(file.name.clone());
                    set
                });
        }
    }
}
