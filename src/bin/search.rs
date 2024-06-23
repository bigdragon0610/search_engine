use std::{collections::HashMap, env::args};

use search_engine::{bigram, dict::Dict};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        panic!("args is not enough: 1 => dict (json), 2 => search word");
    }

    let path = args[1].clone();
    let dict_file = std::fs::File::open(path.clone()).unwrap();
    let dict = serde_json::from_reader(dict_file).unwrap();

    let search_word = args[2].clone();

    if let Some(result) = search(&search_word, &dict) {
        println!("{:?}", result);
    }
}

fn search(search_word: &str, dict: &Dict) -> Option<Vec<(String, usize)>> {
    if search_word.is_empty() {
        return None;
    }

    let mut cnt_files = HashMap::new();

    if let Some(bigram) = bigram::create_bigram(search_word) {
        for word in bigram {
            if let Some(file_names) = dict.get(&word) {
                for file_name in file_names {
                    cnt_files
                        .entry(file_name.clone())
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            }
        }

        let mut cnt_files = cnt_files
            .iter()
            .filter(|e| *e.1 > 0)
            .map(|e| (e.0.clone(), *e.1))
            .collect::<Vec<(String, usize)>>();
        cnt_files.sort_by(|a, b| b.1.cmp(&a.1));
        return Some(cnt_files);
    }

    None
}
