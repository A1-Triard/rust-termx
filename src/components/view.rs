use int_vec_2d::{Rect, Point, Vector};
use ooecs::Entity;

pub struct View {
    pub(crate) visual_parent: Option<Entity>,
    pub(crate) real_render_bounds: Rect,
    pub(crate) ty: u16,
}


impl View {
    pub const NONE: u16 = 0;
    pub const DECORATOR: u16 = 1;
    pub const BACKGROUND: u16 = 2;

    pub fn new(ty: u16) -> Self {
        View {
            visual_parent: None,
            real_render_bounds: Rect { tl: Point { x: 0, y: 0 }, size: Vector::null() },
            ty,
        }
    }

    pub fn visual_parent(&self) -> Option<Entity> {
        self.visual_parent
    }

    pub fn ty(&self) -> u16 {
        self.ty
    }
}
