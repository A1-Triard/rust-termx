use ooecs::Entity;

pub struct Decorator {
    pub(crate) child: Option<Entity>,
}

impl Decorator {
    pub fn new() -> Self {
        Decorator { child: None }
    }

    pub fn child(&self) -> Option<Entity> {
        self.child
    }
}
