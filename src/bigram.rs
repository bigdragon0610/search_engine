type Bigram = Vec<String>;

pub fn create_bigram(text: &str) -> Option<Bigram> {
    if text.len() < 2 {
        return None;
    }

    let mut bigram = Bigram::new();

    let chars = text.chars().collect::<Vec<char>>();
    for i in 0..chars.len() - 1 {
        if chars[i].is_ascii_whitespace() || chars[i + 1].is_ascii_whitespace() {
            continue;
        }

        bigram.push(format!("{}{}", chars[i], chars[i + 1]));
    }

    if bigram.is_empty() {
        return None;
    }

    Some(bigram)
}
