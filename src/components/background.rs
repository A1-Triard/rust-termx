pub struct Background {
    pub(crate) pattern: String,
}

impl Background {
    pub fn new() -> Self {
        Background {
            pattern: "░".to_string()
        }
    }

    pub fn pattern(&self) -> &str {
        &self.pattern
    }
}
