use alloc::rc::Rc;
use alloc::string::String;
use crate::components::layout_view::*;
use crate::components::panel::Panel;
use crate::components::focus_scope::FocusScope;
use crate::components::view::*;
use crate::template::{Template, NameResolver};
use crate::termx::{Termx, IsTermx};
use ooecs::Entity;

pub struct Canvas { }

impl Canvas {
    pub fn new() -> Self {
        Canvas { }
    }

    pub fn new_entity(termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        let termx = termx.termx();
        let view = termx.components().view;
        let layout_view = termx.components().layout_view;
        let focus_scope = termx.components().focus_scope;
        let panel = termx.components().panel;
        let canvas = termx.components().canvas;
        let mut world = termx.world.borrow_mut();
        let p = Entity::new(canvas, &mut world);
        p.add(view, &mut world, View::new(TREE_PANEL, RENDER_NONE));
        p.add(layout_view, &mut world, LayoutView::new(LAYOUT_CANVAS));
        p.add(focus_scope, &mut world, FocusScope::new());
        p.add(panel, &mut world, Panel::new());
        p.add(canvas, &mut world, Canvas::new());
        p
    }
}

#[macro_export]
macro_rules! canvas_template {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident in $mod:ident {
            $(use $path:path as $import:ident;)*
            $($(
                $(#[$field_attr:meta])*
                pub $field_name:ident : $field_ty:ty
            ),+ $(,)?)?
        }
    ) => {
        $crate::panel_template! {
            $(#[$attr])*
            $vis struct $name in $mod {
                $(use $path as $import;)*

                $($(
                    $(#[$field_attr])*
                    pub $field_name : $field_ty
                ),+)?
            }
        }
    };
}

#[macro_export]
macro_rules! canvas_apply_template {
    ($this:ident, $entity:ident, $termx:expr, $names:ident) => {
        $crate::panel_apply_template! { $this, $entity, $termx, $names }
    };
}

canvas_template! {
    #[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
    #[serde(rename="Canvas@Children")]
    pub struct CanvasTemplate in template { }
}

#[typetag::serde(name="Canvas")]
impl Template for CanvasTemplate {
    fn name(&self) -> Option<&String> {
        Some(&self.name)
    }

    fn create_entity(&self, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        Canvas::new_entity(termx)
    }

    fn apply(&self, entity: Entity<Termx>, termx: &Rc<dyn IsTermx>, names: &mut NameResolver) {
        let this = self;
        canvas_apply_template! { this, entity, termx, names }
    }
}
