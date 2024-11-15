#[derive(Debug)]
pub struct Section {
    pub title: String,
    pub content: String,
}

impl Section {
    pub fn new(title: String, content: String) -> Self {
        Self { title, content }
    }
}
