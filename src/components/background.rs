use alloc::string::{String, ToString};
use alloc::rc::Rc;
use crate::components::decorator::Decorator;
use crate::components::view::*;
use crate::components::view_layout::*;
use crate::property;
use crate::termx::IsTermx;
use ooecs::Entity;
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

    pub fn new_entity(termx: &Rc<dyn IsTermx>) -> Entity {
        let termx = termx.termx();
        let view = termx.components().view;
        let view_layout = termx.components().view_layout;
        let decorator = termx.components().decorator;
        let background = termx.components().background;
        let mut world = termx.world.borrow_mut();
        let bg = Entity::new(background, &mut world);
        bg.add(view, &mut world, View::new(TREE_DECORATOR, RENDER_BACKGROUND));
        bg.add(view_layout, &mut world, ViewLayout::new(LAYOUT_BACKGROUND));
        bg.add(decorator, &mut world, Decorator::new());
        bg.add(background, &mut world, Background::new());
        bg
    }

    property!(Termx, background, pattern, ref String as &str, @render);
    property!(Termx, background, color, (Fg, Bg), @render);
}
