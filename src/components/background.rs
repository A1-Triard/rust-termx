use alloc::string::{String, ToString};
use alloc::rc::Rc;
use crate::components::decorator::Decorator;
use crate::components::focus_scope::FocusScope;
use crate::components::view::*;
use crate::components::layout_view::*;
use crate::property;
use crate::resources::Resources;
use crate::template::{Template, NameResolver};
use crate::termx::{IsTermx, Termx};
use ooecs::{Entity, World};
use termx_screen_base::{Bg, Fg};

pub struct Background {
    pattern: Rc<String>,
    color: (Fg, Bg),
    fit_to_content: bool,
}

impl Background {
    pub fn new() -> Self {
        Background {
            pattern: Rc::new("░".to_string()),
            color: (Fg::LightGray, Bg::Black),
            fit_to_content: false,
        }
    }

    pub fn new_entity(world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        let c = termx.termx().components();
        let bg = Entity::new(c.background, world);
        bg.add(c.view, world, View::new(TREE_DECORATOR, RENDER_BACKGROUND));
        bg.add(c.layout_view, world, LayoutView::new(LAYOUT_BACKGROUND));
        bg.add(c.focus_scope, world, FocusScope::new());
        bg.add(c.decorator, world, Decorator::new());
        bg.add(c.background, world, Background::new());
        bg
    }

    property!(Termx, background, pattern, ref Rc<String>, @render);
    property!(Termx, background, color, (Fg, Bg), @render);
    property!(Termx, background, fit_to_content, bool, @arrange);
}

#[macro_export]
macro_rules! background_template {
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
                use $crate::base::serialize_color as components_background_serialize_color;
                use $crate::base::deserialize_color as components_background_deserialize_color;
                $(use $path as $import;)*

                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub pattern: Option<$crate::alloc_string_String>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_background_serialize_color")]
                #[serde(deserialize_with="components_background_deserialize_color")]
                pub color: Option<($crate::base::Fg, $crate::base::Bg)>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub fit_to_content: Option<bool>,
                $($(
                    $(#[$field_attr])*
                    pub $field_name : $field_ty
                ),+)?
            }
        }
    };
}

#[macro_export]
macro_rules! background_apply_template {
    ($this:ident, $entity:ident, $world:expr, $termx:expr, $names:ident) => {
        $crate::decorator_apply_template! { $this, $entity, $world, $termx, $names }
        $this.pattern.as_ref().map(|x|
            $crate::components::background::Background::set_pattern(
                $entity,
                $world,
                $termx,
                $crate::alloc_rc_Rc::new(x.clone())
            )
        );
        $this.color.map(|x| $crate::components::background::Background::set_color($entity, $world, $termx, x));
        $this.fit_to_content.map(|x|
            $crate::components::background::Background::set_fit_to_content($entity, $world, $termx, x)
        );
    };
}

background_template! {
    #[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
    #[serde(rename="Background@Child")]
    pub struct BackgroundTemplate in template { }
}

#[typetag::serde(name="Background")]
impl Template for BackgroundTemplate {
    fn name(&self) -> Option<&String> {
        Some(&self.name)
    }

    fn create_entity(&self, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        Background::new_entity(world, termx)
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
        background_apply_template! { this, entity, world, termx, names }
    }
}
