use alloc::rc::Rc;
use alloc::string::String;
use crate::components::layout_view::*;
use crate::components::panel::Panel;
use crate::components::focus_scope::FocusScope;
use crate::components::view::*;
use crate::template::{Template, NameResolver};
use crate::termx::{Termx, IsTermx};
use ooecs::{Entity, World};

pub struct AdornersPanel { }

impl AdornersPanel {
    pub fn new() -> Self {
        AdornersPanel { }
    }

    pub fn new_entity(world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        let c = termx.termx().components();
        let p = Entity::new(c.adorners_panel, world);
        p.add(c.view, world, View::new(TREE_PANEL, RENDER_NONE));
        p.add(c.layout_view, world, LayoutView::new(LAYOUT_ADORNERS_PANEL));
        p.add(c.focus_scope, world, FocusScope::new());
        p.add(c.panel, world, Panel::new());
        p.add(c.adorners_panel, world, AdornersPanel::new());
        p
    }
}

#[macro_export]
macro_rules! adorners_panel_template {
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
macro_rules! adorners_panel_apply_template {
    ($this:ident, $entity:ident, $world:expr, $termx:expr, $names:ident) => {
        $crate::panel_apply_template! { $this, $entity, $world, $termx, $names }
    };
}

adorners_panel_template! {
    #[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
    #[serde(rename="AdornersPanel@Children")]
    pub struct AdornersPanelTemplate in template { }
}

#[typetag::serde(name="AdornersPanel")]
impl Template for AdornersPanelTemplate {
    fn name(&self) -> Option<&String> {
        Some(&self.name)
    }

    fn create_entity(&self, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        AdornersPanel::new_entity(world, termx)
    }

    fn apply(
        &self,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        names: &mut NameResolver,
    ) {
        let this = self;
        adorners_panel_apply_template! { this, entity, world, termx, names }
    }
}
