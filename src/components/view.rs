use int_vec_2d::{Rect, Point, Vector};
use ooecs::Entity;

pub struct View {
    pub(crate) visual_parent: Option<Entity>,
    pub(crate) real_render_bounds: Rect,
}

impl View {
    pub fn new() -> Self {
        View {
            visual_parent: None,
            real_render_bounds: Rect { tl: Point { x: 0, y: 0 }, size: Vector::null() },
        }
    }

    pub fn visual_parent(&self) -> Option<Entity> {
        self.visual_parent
    }
}
