use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use crate::components::decorator::Decorator;
use crate::components::focus_scope::FocusScope;
use crate::components::view::*;
use crate::components::layout_view::*;
use crate::property;
use crate::template::{Template, NameResolver};
use crate::termx::{IsTermx, Termx};
use ooecs::{Entity, World};
use termx_screen_base::{Bg, Fg};

pub struct Border {
    double: bool,
    color: (Fg, Bg),
}

impl Border {
    pub fn new() -> Self {
        Border {
            double: false,
            color: (Fg::LightGray, Bg::Black),
        }
    }

    pub fn new_entity(world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        let c = termx.termx().components();
        let b = Entity::new(c.border, world);
        b.add(c.view, world, View::new(TREE_DECORATOR, RENDER_BORDER));
        b.add(c.layout_view, world, LayoutView::new(LAYOUT_BORDER));
        b.add(c.focus_scope, world, FocusScope::new());
        b.add(c.decorator, world, Decorator::new());
        b.add(c.border, world, Border::new());
        b
    }

    property!(Termx, border, double, bool, @render);
    property!(Termx, border, color, (Fg, Bg), @render);
}

#[macro_export]
macro_rules! border_template {
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
        $crate::decorator_template! {
            $(#[$attr])*
            $vis struct $name in $mod {
                use $crate::base::serialize_color as components_border_serialize_color;
                use $crate::base::deserialize_color as components_border_deserialize_color;
                $(use $path as $import;)*

                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub double: Option<bool>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_border_serialize_color")]
                #[serde(deserialize_with="components_border_deserialize_color")]
                pub color: Option<($crate::base::Fg, $crate::base::Bg)>,
                $($(
                    $(#[$field_attr])*
                    pub $field_name : $field_ty
                ),+)?
            }
        }
    };
}

#[macro_export]
macro_rules! border_apply_template {
    ($this:ident, $entity:ident, $world:expr, $termx:expr, $names:ident) => {
        $crate::decorator_apply_template! { $this, $entity, $world, $termx, $names }
        $this.double.map(|x| $crate::components::border::Border::set_double($entity, $world, $termx, x));
        $this.color.map(|x| $crate::components::border::Border::set_color($entity, $world, $termx, x));
    };
}

border_template! {
    #[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
    #[serde(rename="Border@Child")]
    pub struct BorderTemplate in template { }
}

#[typetag::serde(name="Border")]
impl Template for BorderTemplate {
    fn name(&self) -> Option<&String> {
        Some(&self.name)
    }

    fn create_entity(&self, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        Border::new_entity(world, termx)
    }

    fn apply_resources<'a>(
        &self,
        entity: Entity<Termx>,
        world: &'a mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
    ) -> Option<&'a Box<dyn Template>> {
        View::apply_resources(&self.resources, entity, world, termx, &self.style_key, "IMPLICIT_Border")
    }

    fn apply(
        &self,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        names: &mut NameResolver,
    ) {
        let this = self;
        border_apply_template! { this, entity, world, termx, names }
    }
}
