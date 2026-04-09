use alloc::rc::Rc;
use alloc::string::String;
use crate::property;
use crate::base::{Fg, Bg};
use crate::components::focus_scope::FocusScope;
use crate::components::input_line::InputLine;
use crate::components::input_element::*;
use crate::components::layout_view::*;
use crate::components::view::*;
use crate::template::{Template, NameResolver};
use crate::termx::{IsTermx, Termx};
use ooecs::{Entity, World};

pub struct MInputLine {
    color: (Fg, Bg),
    color_focused: (Fg, Bg),
    color_disabled: (Fg, Bg),
}

impl MInputLine {
    pub fn new() -> Self {
        MInputLine {
            color: (Fg::Black, Bg::Cyan),
            color_focused: (Fg::Black, Bg::Cyan),
            color_disabled: (Fg::DarkGray, Bg::Cyan),
        }
    }

    pub fn new_entity(world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        let termx_inner = termx.termx();
        let c = termx_inner.components();
        let e = Entity::new(c.m_input_line, world);
        e.add(c.view, world, View::new(TREE_NONE, RENDER_M_INPUT_LINE));
        e.add(c.layout_view, world, LayoutView::new(LAYOUT_M_INPUT_LINE));
        e.add(c.focus_scope, world, FocusScope::new());
        e.add(c.input_element, world, InputElement::new(INPUT_INPUT_LINE));
        e.add(c.input_line, world, InputLine::new());
        e.add(c.m_input_line, world, MInputLine::new());
        e
    }

    property!(Termx, m_input_line, color, (Fg, Bg), @render);
    property!(Termx, m_input_line, color_focused, (Fg, Bg), @render);
    property!(Termx, m_input_line, color_disabled, (Fg, Bg), @render);
}

#[macro_export]
macro_rules! m_input_line_template {
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
        $crate::input_line_template! {
            $(#[$attr])*
            $vis struct $name in $mod {
                use $crate::base::serialize_color as components_m_input_line_serialize_color;
                use $crate::base::deserialize_color as components_m_input_line_deserialize_color;
                $(use $path as $import;)*

                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_m_input_line_serialize_color")]
                #[serde(deserialize_with="components_m_input_line_deserialize_color")]
                pub color: Option<($crate::base::Fg, $crate::base::Bg)>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_m_input_line_serialize_color")]
                #[serde(deserialize_with="components_m_input_line_deserialize_color")]
                pub color_focused: Option<($crate::base::Fg, $crate::base::Bg)>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_m_input_line_serialize_color")]
                #[serde(deserialize_with="components_m_input_line_deserialize_color")]
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
macro_rules! m_input_line_apply_template {
    ($this:ident, $entity:ident, $world:expr, $termx:expr, $names:ident) => {
        $crate::input_line_apply_template! { $this, $entity, $world, $termx, $names }
        $this.color.map(|x|
            $crate::components::m_input_line::MInputLine::set_color($entity, $world, $termx, x)
        );
        $this.color_focused.map(|x|
            $crate::components::m_input_line::MInputLine::set_color_focused($entity, $world, $termx, x)
        );
        $this.color_disabled.map(|x|
            $crate::components::m_input_line::MInputLine::set_color_disabled($entity, $world, $termx, x)
        );
    };
}

m_input_line_template! {
    #[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
    #[serde(rename="MInputLine")]
    pub struct MInputLineTemplate in template { }
}

#[typetag::serde(name="MInputLine")]
impl Template for MInputLineTemplate {
    fn name(&self) -> Option<&String> {
        Some(&self.name)
    }

    fn create_entity(&self, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        MInputLine::new_entity(world, termx)
    }

    fn apply(
        &self,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        names: &mut NameResolver,
    ) {
        let this = self;
        m_input_line_apply_template! { this, entity, world, termx, names }
    }
}
