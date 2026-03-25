use termx_screen_base::{Bg, Fg};

pub struct Background {
    pub(crate) pattern: String,
    pub(crate) color: (Fg, Bg),
}

impl Background {
    pub fn new() -> Self {
        Background {
            pattern: "░".to_string(),
            color: (Fg::LightGray, Bg::Black),
        }
    }

    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    pub fn color(&self) -> (Fg, Bg) {
        self.color
    }
}
