use alloc::rc::Rc;
use alloc::string::String;
use crate::components::layout_view::*;
use crate::components::panel::Panel;
use crate::components::focus_scope::FocusScope;
use crate::components::view::*;
use crate::property;
use crate::template::{Template, NameResolver};
use crate::termx::{Termx, IsTermx};
use ooecs::{Entity, World};

pub struct StackPanel {
    vertical: bool,
}

impl StackPanel {
    pub fn new() -> Self {
        StackPanel { vertical: true }
    }

    pub fn new_entity(world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        let termx = termx.termx();
        let view = termx.components().view;
        let layout_view = termx.components().layout_view;
        let focus_scope = termx.components().focus_scope;
        let panel = termx.components().panel;
        let stack_panel = termx.components().stack_panel;
        let p = Entity::new(stack_panel, world);
        p.add(view, world, View::new(TREE_PANEL, RENDER_NONE));
        p.add(layout_view, world, LayoutView::new(LAYOUT_STACK_PANEL));
        p.add(focus_scope, world, FocusScope::new());
        p.add(panel, world, Panel::new());
        p.add(stack_panel, world, StackPanel::new());
        p
    }

    property!(Termx, stack_panel, vertical, bool, @measure);
}

#[macro_export]
macro_rules! stack_panel_template {
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

                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub vertical: Option<bool>,
                $($(
                    $(#[$field_attr])*
                    pub $field_name : $field_ty
                ),+)?
            }
        }
    };
}

#[macro_export]
macro_rules! stack_panel_apply_template {
    ($this:ident, $entity:ident, $world:expr, $termx:expr, $names:ident) => {
        $crate::panel_apply_template! { $this, $entity, $world, $termx, $names }
        $this.vertical.map(|x|
            $crate::components::stack_panel::StackPanel::set_vertical($entity, $world, $termx, x)
        );
    };
}

stack_panel_template! {
    #[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
    #[serde(rename="StackPanel@Children")]
    pub struct StackPanelTemplate in template { }
}

#[typetag::serde(name="StackPanel")]
impl Template for StackPanelTemplate {
    fn name(&self) -> Option<&String> {
        Some(&self.name)
    }

    fn create_entity(&self, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        StackPanel::new_entity(world, termx)
    }

    fn apply_resources<'a>(
        &self,
        entity: Entity<Termx>,
        world: &'a mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
    ) -> Option<&'a Rc<dyn Template>> {
        View::apply_resources(
            self.resources.clone(), entity, world, termx, &self.style_key, "IMPLICIT_StackPanel"
        )
    }

    fn apply(
        &self,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        names: &mut NameResolver
    ) {
        let this = self;
        stack_panel_apply_template! { this, entity, world, termx, names }
    }
}
