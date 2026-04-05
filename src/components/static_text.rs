use alloc::rc::Rc;
use alloc::string::String;
use crate::property;
use crate::base::{Fg, Bg, TextWrapping, TextAlign};
use crate::components::layout_view::*;
use crate::components::view::*;
use crate::template::{Template, NameResolver};
use crate::termx::{IsTermx, Termx};
use ooecs::Entity;

pub struct StaticText {
    text: String,
    text_align: TextAlign,
    text_wrapping: TextWrapping,
    color: (Fg, Bg),
}

impl StaticText {
    pub fn new() -> Self {
        StaticText {
            text: String::new(),
            text_align: TextAlign::Left,
            text_wrapping: TextWrapping::NoWrap,
            color: (Fg::LightGray, Bg::None),
        }
    }

    pub fn new_entity(termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        let termx_inner = termx.termx();
        let view = termx_inner.components().view;
        let layout_view = termx_inner.components().layout_view;
        let static_text = termx_inner.components().static_text;
        let mut world = termx_inner.world.borrow_mut();
        let e = Entity::new(static_text, &mut world);
        e.add(view, &mut world, View::new(TREE_NONE, RENDER_STATIC_TEXT));
        e.add(layout_view, &mut world, LayoutView::new(LAYOUT_STATIC_TEXT));
        e.add(static_text, &mut world, StaticText::new());
        e
    }

    property!(Termx, static_text, text, ref String as &str, @render+measure);
    property!(Termx, static_text, text_align, TextAlign, @render);
    property!(Termx, static_text, text_wrapping, TextWrapping, @render+measure);
    property!(Termx, static_text, color, (Fg, Bg), @render);
}

#[macro_export]
macro_rules! static_text_template {
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
                use $crate::base::serialize_color as components_static_text_serialize_color;
                use $crate::base::deserialize_color as components_static_text_deserialize_color;
                $(use $path as $import;)*

                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub text: Option<$crate::alloc_string_String>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub text_align: Option<$crate::base::TextAlign>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub text_wrapping: Option<$crate::base::TextWrapping>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_static_text_serialize_color")]
                #[serde(deserialize_with="components_static_text_deserialize_color")]
                pub color: Option<($crate::base::Fg, $crate::base::Bg)>
                $(,$(
                    $(#[$field_attr])*
                    pub $field_name : $field_ty
                ),+)?
            }
        }
    };
}

#[macro_export]
macro_rules! static_text_apply_template {
    ($this:ident, $entity:ident, $termx:expr, $names:ident) => {
        $crate::layout_view_apply_template! { $this, $entity, $termx, $names }
        $this.text.as_ref().map(|x|
            $crate::components::static_text::StaticText::set_text($entity, $termx, x.clone())
        );
        $this.text_align.map(|x|
            $crate::components::static_text::StaticText::set_text_align($entity, $termx, x)
        );
        $this.text_wrapping.map(|x|
            $crate::components::static_text::StaticText::set_text_wrapping($entity, $termx, x)
        );
        $this.color.map(|x|
            $crate::components::static_text::StaticText::set_color($entity, $termx, x)
        );
    };
}

static_text_template! {
    #[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
    #[serde(rename="StaticText@Text")]
    pub struct StaticTextTemplate in template { }
}

#[typetag::serde(name="StaticText")]
impl Template for StaticTextTemplate {
    fn name(&self) -> Option<&String> {
        Some(&self.name)
    }

    fn create_entity(&self, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        StaticText::new_entity(termx)
    }

    fn apply(&self, entity: Entity<Termx>, termx: &Rc<dyn IsTermx>, names: &mut NameResolver) {
        let this = self;
        static_text_apply_template! { this, entity, termx, names }
    }
}
