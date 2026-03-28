use int_vec_2d::{Rect, Point, Vector};
use ooecs::Entity;

pub struct View {
    pub(crate) visual_parent: Option<Entity>,
    pub(crate) real_render_bounds: Rect,
    pub(crate) tree: u16,
    pub(crate) render: u16,
}

pub const TREE_NONE: u16 = 0;
pub const TREE_DECORATOR: u16 = 1;

pub const RENDER_NONE: u16 = 0;
pub const RENDER_BACKGROUND: u16 = 1;

impl View {
    pub fn new(tree: u16, render: u16) -> Self {
        View {
            visual_parent: None,
            real_render_bounds: Rect { tl: Point { x: 0, y: 0 }, size: Vector::null() },
            tree,
            render,
        }
    }

    pub fn visual_parent(&self) -> Option<Entity> {
        self.visual_parent
    }

    pub fn tree(&self) -> u16 {
        self.tree
    }

    pub fn render(&self) -> u16 {
        self.render
    }
}
