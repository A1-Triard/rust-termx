#![feature(macro_metavar_expr_concat)]

pub mod components;
pub mod systems;
pub mod termx;

#[cfg(test)]
mod tests {
    use crate::components::view::View;
    use crate::components::view_layout::ViewLayout;
    use crate::components::decorator::Decorator;
    use crate::components::background::Background;
    use crate::systems::layout::LayoutExt;
    use crate::termx::{Termx, TermxExt};
    use int_vec_2d::Vector;
    use ooecs::Entity;

    #[test]
    fn layout_works() {
        let termx = Termx::new();
        let bg = {
            let mut data = termx.termx().data.borrow_mut();
            let view = data.components.as_ref().unwrap().view;
            let view_layout = data.components.as_ref().unwrap().view_layout;
            let decorator = data.components.as_ref().unwrap().decorator;
            let background = data.components.as_ref().unwrap().background;
            let bg = Entity::new(background, &mut data.world);
            bg.add(view, &mut data.world, View::new());
            bg.add(view_layout, &mut data.world, ViewLayout::new());
            bg.add(decorator, &mut data.world, Decorator::new());
            bg.add(background, &mut data.world, Background::new());
            let layout = data.systems.as_ref().unwrap().layout.clone();
            layout.perform(bg, &mut data.world, Vector { x: 80, y: 25 });
            bg
        };
        termx.set_background_pattern(bg, "x".to_string());
    }
}
