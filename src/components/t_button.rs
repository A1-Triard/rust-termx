use alloc::string::String;
use alloc::rc::Rc;
use crate::property;
use crate::base::{Fg, Bg};
use crate::components::button::Button;
use crate::components::view::*;
use crate::components::layout_view::*;
use crate::components::focus_scope::FocusScope;
use crate::components::input_element::*;
use crate::resources::Resources;
use crate::systems::init::InitExt;
use crate::template::{Template, NameResolver};
use crate::termx::{IsTermx, Termx};
use ooecs::{Entity, World};

pub struct TButton {
    color: (Fg, Bg),
    color_hotkey: (Fg, Bg),
    color_focused: (Fg, Bg),
    color_focused_hotkey: (Fg, Bg),
    color_disabled: (Fg, Bg),
}

impl TButton {
    pub fn new() -> Self {
        TButton {
            color: (Fg::Black, Bg::Green),
            color_hotkey: (Fg::Yellow, Bg::Green),
            color_focused: (Fg::White, Bg::Green),
            color_focused_hotkey: (Fg::Yellow, Bg::Green),
            color_disabled: (Fg::DarkGray, Bg::Green),
        }
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

    property!(Termx, t_button, color, (Fg, Bg), @render);
    property!(Termx, t_button, color_hotkey, (Fg, Bg), @render);
    property!(Termx, t_button, color_focused, (Fg, Bg), @render);
    property!(Termx, t_button, color_focused_hotkey, (Fg, Bg), @render);
    property!(Termx, t_button, color_disabled, (Fg, Bg), @render);
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
                use $crate::base::serialize_color as components_t_button_serialize_color;
                use $crate::base::deserialize_color as components_t_button_deserialize_color;
                $(use $path as $import;)*

                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_t_button_serialize_color")]
                #[serde(deserialize_with="components_t_button_deserialize_color")]
                pub color: Option<($crate::base::Fg, $crate::base::Bg)>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_t_button_serialize_color")]
                #[serde(deserialize_with="components_t_button_deserialize_color")]
                pub color_hotkey: Option<($crate::base::Fg, $crate::base::Bg)>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_t_button_serialize_color")]
                #[serde(deserialize_with="components_t_button_deserialize_color")]
                pub color_focused: Option<($crate::base::Fg, $crate::base::Bg)>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_t_button_serialize_color")]
                #[serde(deserialize_with="components_t_button_deserialize_color")]
                pub color_focused_hotkey: Option<($crate::base::Fg, $crate::base::Bg)>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_t_button_serialize_color")]
                #[serde(deserialize_with="components_t_button_deserialize_color")]
                pub color_disabled: Option<($crate::base::Fg, $crate::base::Bg)>,
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
        $this.color.map(|x| $crate::components::t_button::TButton::set_color($entity, $world, $termx, x));
        $this.color_hotkey.map(|x|
            $crate::components::t_button::TButton::set_color_hotkey($entity, $world, $termx, x)
        );
        $this.color_focused.map(
            |x| $crate::components::t_button::TButton::set_color_focused($entity, $world, $termx, x)
        );
        $this.color_focused_hotkey.map(
            |x| $crate::components::t_button::TButton::set_color_focused_hotkey($entity, $world, $termx, x)
        );
        $this.color_disabled.map(
            |x| $crate::components::t_button::TButton::set_color_disabled($entity, $world, $termx, x)
        );
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

    fn style_key(&self) -> Option<&str> {
        Some(if self.style_key.is_empty() { "IMPLICIT_TButton" } else { &self.style_key })
    }

    fn create_entity(&self, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        TButton::new_entity(world, termx)
    }

    fn apply_resources(
        &self,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        base_resources: Option<Rc<Resources>>,
    ) -> Option<Rc<Resources>> {
        View::apply_resources(&self.resources, entity, world, termx, base_resources)
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
