#![feature(macro_metavar_expr_concat)]

pub mod base;
pub mod components;
pub mod systems;
pub mod termx;
pub mod render_port;

#[cfg(test)]
mod tests {
    use crate::systems::layout::LayoutExt;
    use crate::termx::{Termx, TermxExt};
    use int_vec_2d::Vector;

    #[test]
    fn create_view_perform_layout_change_property() {
        let termx = Termx::new();
        let bg = termx.new_background();
        {
            let mut data = termx.termx().data.borrow_mut();
            let layout = data.systems.as_ref().unwrap().layout.clone();
            layout.perform(bg, &mut data.world, Vector { x: 80, y: 25 });
        }
        termx.set_background_pattern(bg, "x".to_string());
    }
}
