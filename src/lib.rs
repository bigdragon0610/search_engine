pub mod bigram;
pub mod dict;

#[derive(Clone)]
pub struct File {
    pub name: String,
    pub content: String,
}
