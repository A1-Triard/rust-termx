use alloc::rc::Rc;
use alloc::string::String;
use crate::property;
use crate::base::{Fg, Bg, TextWrapping, TextAlign};
use crate::components::layout_view::*;
use crate::components::view::*;
use crate::template::{Template, NameResolver};
use crate::termx::{IsTermx, Termx};
use ooecs::{Entity, World};

pub struct StaticText {
    text: Rc<String>,
    text_align: TextAlign,
    text_wrapping: TextWrapping,
    color: (Fg, Bg),
}

impl StaticText {
    pub fn new() -> Self {
        StaticText {
            text: Rc::new(String::new()),
            text_align: TextAlign::Left,
            text_wrapping: TextWrapping::NoWrap,
            color: (Fg::LightGray, Bg::None),
        }
    }

    pub fn new_entity(world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        let termx_inner = termx.termx();
        let view = termx_inner.components().view;
        let layout_view = termx_inner.components().layout_view;
        let static_text = termx_inner.components().static_text;
        let e = Entity::new(static_text, world);
        e.add(view, world, View::new(TREE_NONE, RENDER_STATIC_TEXT));
        e.add(layout_view, world, LayoutView::new(LAYOUT_STATIC_TEXT));
        e.add(static_text, world, StaticText::new());
        e
    }

    property!(Termx, static_text, text, ref Rc<String>, @render+measure);
    property!(Termx, static_text, text_align, TextAlign, @render);
    property!(Termx, static_text, text_wrapping, TextWrapping, @render+measure);
    property!(Termx, static_text, color, (Fg, Bg), @render);

    pub fn apply_implicit_style(entity: Entity<Termx>, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) {
        View::apply_style(entity, world, termx, "IMPLICIT_StaticText");
    }
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
    ($this:ident, $entity:ident, $world:expr, $termx:expr, $names:ident) => {
        $crate::layout_view_apply_template! { $this, $entity, $world, $termx, $names }
        $this.text.as_ref().map(|x|
            $crate::components::static_text::StaticText::set_text(
                $entity,
                $world,
                $termx,
                $crate::alloc_rc_Rc::new(x.clone())
            )
        );
        $this.text_align.map(|x|
            $crate::components::static_text::StaticText::set_text_align($entity, $world, $termx, x)
        );
        $this.text_wrapping.map(|x|
            $crate::components::static_text::StaticText::set_text_wrapping($entity, $world, $termx, x)
        );
        $this.color.map(|x|
            $crate::components::static_text::StaticText::set_color($entity, $world, $termx, x)
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

    fn create_entity(&self, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        StaticText::new_entity(world, termx)
    }

    fn apply_resources<'a>(
        &self,
        entity: Entity<Termx>,
        world: &'a mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
    ) -> Option<&'a Rc<dyn Template>> {
        View::apply_resources(
            self.resources.clone(), entity, world, termx, &self.style_key, "IMPLICIT_StaticText"
        )
    }

    fn apply(
        &self,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        names: &mut NameResolver,
    ) {
        let this = self;
        static_text_apply_template! { this, entity, world, termx, names }
    }
}
