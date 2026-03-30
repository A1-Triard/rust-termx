use alloc::string::String;
use alloc::rc::Rc;
use crate::base::{Fg, Bg};
use crate::components::view::*;
use crate::components::layout_view::*;
use crate::property;
use crate::template::{Template, NameResolver};
use crate::termx::{IsTermx, Termx};
use ooecs::Entity;

pub struct TButton {
    text: String,
    color: (Fg, Bg),
    color_hotkey: (Fg, Bg),
}

impl TButton {
    pub fn new() -> Self {
        TButton {
            text: String::new(),
            color: (Fg::Black, Bg::Green),
            color_hotkey: (Fg::Yellow, Bg::Green),
        }
    }

    pub fn new_entity(termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        let termx = termx.termx();
        let view = termx.components().view;
        let layout_view = termx.components().layout_view;
        let t_button = termx.components().t_button;
        let mut world = termx.world.borrow_mut();
        let b = Entity::new(t_button, &mut world);
        b.add(view, &mut world, View::new(TREE_NONE, RENDER_T_BUTTON));
        b.add(layout_view, &mut world, LayoutView::new(LAYOUT_T_BUTTON));
        b.add(t_button, &mut world, TButton::new());
        b
    }

    property!(Termx, t_button, text, ref String as &str, @measure);
    property!(Termx, t_button, color, (Fg, Bg), @render);
    property!(Termx, t_button, color_hotkey, (Fg, Bg), @render);
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
        $crate::layout_view_template! {
            $(#[$attr])*
            $vis struct $name in $mod {
                use $crate::base::serialize_color as components_t_button_serialize_color;
                use $crate::base::deserialize_color as components_t_button_deserialize_color;
                $(use $path as $import;)*

                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub text: Option<$crate::alloc_string_String>,
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
    ($this:ident, $entity:ident, $termx:expr, $names:ident) => {
        $crate::layout_view_apply_template! { $this, $entity, $termx, $names }
        $this.text.as_ref().map(|x|
            $crate::components::t_button::TButton::set_text($entity, $termx, x.clone())
        );
        $this.color.map(|x| $crate::components::t_button::TButton::set_color($entity, $termx, x));
        $this.color_hotkey.map(|x| $crate::components::t_button::TButton::set_color_hotkey($entity, $termx, x));
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

    fn create_entity(&self, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        TButton::new_entity(termx)
    }

    fn apply(&self, entity: Entity<Termx>, termx: &Rc<dyn IsTermx>, names: &mut NameResolver) {
        let this = self;
        t_button_apply_template! { this, entity, termx, names }
    }
}
