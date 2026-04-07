use alloc::string::String;
use alloc::rc::Rc;
use crate::components::button::Button;
use crate::components::view::*;
use crate::components::layout_view::*;
use crate::components::focus_scope::FocusScope;
use crate::components::input_element::*;
use crate::systems::init::InitExt;
use crate::template::{Template, NameResolver};
use crate::termx::{IsTermx, Termx};
use ooecs::{Entity, World};

pub struct TButton { }

impl TButton {
    pub fn new() -> Self {
        TButton { }
    }

    pub fn new_entity(world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        let c = termx.termx().components();
        let b = Entity::new(c.t_button, world);
        b.add(c.view, world, View::new(TREE_NONE, RENDER_T_BUTTON));
        b.add(c.layout_view, world, LayoutView::new(LAYOUT_T_BUTTON));
        b.add(c.focus_scope, world, FocusScope::new());
        b.add(c.input_element, world, InputElement::new(INPUT_BUTTON));
        b.add(c.button, world, Button::new());
        b.add(c.t_button, world, TButton::new());
        termx.termx().systems().init.init_t_button(b, world);
        b
    }
}

#[macro_export]
macro_rules! t_button_template {
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
        $crate::button_template! {
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
macro_rules! t_button_apply_template {
    ($this:ident, $entity:ident, $world:expr, $termx:expr, $names:ident) => {
        $crate::button_apply_template! { $this, $entity, $world, $termx, $names }
    };
}

t_button_template! {
    #[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
    #[serde(rename="TButton@Text")]
    pub struct TButtonTemplate in template { }
}

#[typetag::serde(name="TButton")]
impl Template for TButtonTemplate {
    fn name(&self) -> Option<&String> {
        Some(&self.name)
    }

    fn create_entity(&self, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        TButton::new_entity(world, termx)
    }

    fn apply(
        &self,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        names: &mut NameResolver,
    ) {
        let this = self;
        t_button_apply_template! { this, entity, world, termx, names }
    }
}
