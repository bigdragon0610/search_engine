use std::env::args;

use search_engine::{dict::create_dict, File};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 3 {
        panic!("args is not enough: 1 => dict (json), 2 => file name, 3 => content");
    }

    let path = args[1].clone();
    let file = File {
        name: args[2].clone(),
        content: args[3].clone(),
    };

    let dict_file = std::fs::File::open(path.clone()).unwrap();
    let mut dict = serde_json::from_reader(dict_file).unwrap();
    create_dict(&file, &mut dict);
    std::fs::write(path, serde_json::to_string(&dict).unwrap()).unwrap();
}
