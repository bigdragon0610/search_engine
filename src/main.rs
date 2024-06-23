use std::collections::HashMap;

#[derive(Clone)]
struct File {
    name: String,
    content: String,
}

type Dict = HashMap<String, Vec<String>>;

fn bigram(files: &Vec<File>) -> Dict {
    let mut dict = Dict::new();

    for file in files {
        let chars = file.content.chars().collect::<Vec<char>>();
        for i in 0..chars.len() - 1 {
            if chars[i].is_ascii_whitespace() || chars[i + 1].is_ascii_whitespace() {
                continue;
            }

            dict.entry(format!("{}{}", chars[i], chars[i + 1]))
                .and_modify(|e| e.push(file.name.clone()))
                .or_insert([file.name.clone()].to_vec());
        }
    }

    dict
}

fn search(search_word: &str, dict: &Dict, files: &Vec<File>) {
    if search_word.is_empty() {
        return;
    }

    let mut cnt_files = files
        .iter()
        .map(|file| (file.name.clone(), 0))
        .collect::<HashMap<String, usize>>();

    let chars = search_word.chars().collect::<Vec<char>>();
    for i in 0..chars.len() - 1 {
        if chars[i].is_ascii_whitespace() || chars[i + 1].is_ascii_whitespace() {
            continue;
        }

        let text = &format!("{}{}", chars[i], chars[i + 1]);
        if let Some(file_names) = dict.get(text) {
            for file_name in file_names {
                cnt_files.entry(file_name.clone()).and_modify(|e| *e += 1);
            }
        }
    }

    let mut cnt_files = cnt_files
        .iter()
        .filter(|e| *e.1 > 0)
        .collect::<Vec<(&String, &usize)>>();
    cnt_files.sort_by(|a, b| b.1.cmp(a.1));

    println!("{:?}", cnt_files);
}

fn main() {
    let files = [
        File {
            name: "test1".to_string(),
            content: "完全に理解した".to_string(),
        },
        File {
            name: "test2".to_string(),
            content: "完全な球体".to_string(),
        },
        File {
            name: "test3".to_string(),
            content: "理解できない".to_string(),
        },
    ]
    .to_vec();

    let dict = bigram(&files);

    search("できた", &dict, &files)
}
