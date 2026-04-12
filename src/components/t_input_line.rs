use alloc::rc::Rc;
use alloc::string::String;
use crate::property;
use crate::base::{Fg, Bg};
use crate::components::focus_scope::FocusScope;
use crate::components::input_line::InputLine;
use crate::components::input_element::*;
use crate::components::layout_view::*;
use crate::components::view::*;
use crate::resources::Resources;
use crate::template::{Template, NameResolver};
use crate::termx::{IsTermx, Termx};
use ooecs::{Entity, World};

pub struct TInputLine {
    color: (Fg, Bg),
    color_focused: (Fg, Bg),
    color_disabled: (Fg, Bg),
    color_ellipsis: (Fg, Bg),
}

impl TInputLine {
    pub fn new() -> Self {
        TInputLine {
            color: (Fg::White, Bg::Blue),
            color_focused: (Fg::White, Bg::Blue),
            color_disabled: (Fg::LightGray, Bg::Blue),
            color_ellipsis: (Fg::BrightGreen, Bg::Blue),
        }
    }

    pub fn new_entity(world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        let termx_inner = termx.termx();
        let c = termx_inner.components();
        let e = Entity::new(c.t_input_line, world);
        e.add(c.view, world, View::new(TREE_NONE, RENDER_T_INPUT_LINE));
        e.add(c.layout_view, world, LayoutView::new(LAYOUT_T_INPUT_LINE));
        e.add(c.focus_scope, world, FocusScope::new());
        e.add(c.input_element, world, InputElement::new(INPUT_INPUT_LINE));
        e.add(c.input_line, world, InputLine::new());
        e.add(c.t_input_line, world, TInputLine::new());
        e
    }

    property!(Termx, t_input_line, color, (Fg, Bg), @render);
    property!(Termx, t_input_line, color_focused, (Fg, Bg), @render);
    property!(Termx, t_input_line, color_disabled, (Fg, Bg), @render);
    property!(Termx, t_input_line, color_ellipsis, (Fg, Bg), @render);
}

#[macro_export]
macro_rules! t_input_line_template {
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
                use $crate::base::serialize_color as components_t_input_line_serialize_color;
                use $crate::base::deserialize_color as components_t_input_line_deserialize_color;
                $(use $path as $import;)*

                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_t_input_line_serialize_color")]
                #[serde(deserialize_with="components_t_input_line_deserialize_color")]
                pub color: Option<($crate::base::Fg, $crate::base::Bg)>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_t_input_line_serialize_color")]
                #[serde(deserialize_with="components_t_input_line_deserialize_color")]
                pub color_focused: Option<($crate::base::Fg, $crate::base::Bg)>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_t_input_line_serialize_color")]
                #[serde(deserialize_with="components_t_input_line_deserialize_color")]
                pub color_disabled: Option<($crate::base::Fg, $crate::base::Bg)>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_t_input_line_serialize_color")]
                #[serde(deserialize_with="components_t_input_line_deserialize_color")]
                pub color_ellipsis: Option<($crate::base::Fg, $crate::base::Bg)>,
                $($(
                    $(#[$field_attr])*
                    pub $field_name : $field_ty
                ),+)?
            }
        }
    };
}

#[macro_export]
macro_rules! t_input_line_apply_template {
    ($this:ident, $entity:ident, $world:expr, $termx:expr, $names:ident) => {
        $crate::input_line_apply_template! { $this, $entity, $world, $termx, $names }
        $this.color.map(|x|
            $crate::components::t_input_line::TInputLine::set_color($entity, $world, $termx, x)
        );
        $this.color_focused.map(|x|
            $crate::components::t_input_line::TInputLine::set_color_focused($entity, $world, $termx, x)
        );
        $this.color_disabled.map(|x|
            $crate::components::t_input_line::TInputLine::set_color_disabled($entity, $world, $termx, x)
        );
        $this.color_ellipsis.map(|x|
            $crate::components::t_input_line::TInputLine::set_color_ellipsis($entity, $world, $termx, x)
        );
    };
}

t_input_line_template! {
    #[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
    #[serde(rename="TInputLine")]
    pub struct TInputLineTemplate in template { }
}

#[typetag::serde(name="TInputLine")]
impl Template for TInputLineTemplate {
    fn name(&self) -> Option<&String> {
        Some(&self.name)
    }

    fn create_entity(&self, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        TInputLine::new_entity(world, termx)
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
        t_input_line_apply_template! { this, entity, world, termx, names }
    }
}
