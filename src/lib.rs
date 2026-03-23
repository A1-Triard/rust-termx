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
    use crate::termx::{Termx, IsTermx, TermxExt};
    use int_vec_2d::Vector;
    use ooecs::Entity;

    #[test]
    fn layout_works() {
        let termx = Termx::new();
        let mut data = termx.termx().data.borrow_mut();
        let view = data.components.as_ref().unwrap().view;
        let view_layout = data.components.as_ref().unwrap().view_layout;
        let decorator = data.components.as_ref().unwrap().decorator;
        let background = data.components.as_ref().unwrap().background;
        let bg = Entity::new(&mut data.world);
        bg.add_component(&mut data.world, view, View::new());
        bg.add_component(&mut data.world, view_layout, ViewLayout::new());
        bg.add_component(&mut data.world, decorator, Decorator::new());
        bg.add_component(&mut data.world, background, Background::new());
        let layout = data.systems.as_ref().unwrap().layout.clone();
        layout.perform(bg, &mut data.world, Vector { x: 80, y: 25 });
    }
}
