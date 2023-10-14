#[derive(Debug, Clone)]
pub struct Participant {
    pub name: String,
    pub score: u64,
}

impl Participant {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            score: 0,
        }
    }
}
