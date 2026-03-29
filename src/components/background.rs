use alloc::string::{String, ToString};
use alloc::rc::Rc;
use crate::components::decorator::Decorator;
use crate::components::view::*;
use crate::components::layout_view::*;
use crate::property;
use crate::template::{Template, NameResolver};
use crate::termx::{IsTermx, Termx};
use ooecs::Entity;
use termx_screen_base::{Bg, Fg};

pub struct Background {
    pub(crate) pattern: String,
    pub(crate) color: (Fg, Bg),
}

impl Background {
    pub fn new() -> Self {
        Background {
            pattern: "░".to_string(),
            color: (Fg::LightGray, Bg::Black),
        }
    }

    pub fn new_entity(termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        let termx = termx.termx();
        let view = termx.components().view;
        let layout_view = termx.components().layout_view;
        let decorator = termx.components().decorator;
        let background = termx.components().background;
        let mut world = termx.world.borrow_mut();
        let bg = Entity::new(background, &mut world);
        bg.add(view, &mut world, View::new(TREE_DECORATOR, RENDER_BACKGROUND));
        bg.add(layout_view, &mut world, LayoutView::new(LAYOUT_BACKGROUND));
        bg.add(decorator, &mut world, Decorator::new());
        bg.add(background, &mut world, Background::new());
        bg
    }

    property!(Termx, background, pattern, ref String as &str, @render);
    property!(Termx, background, color, (Fg, Bg), @render);
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
                $(use $path as $import;)*

                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub pattern: Option<$crate::alloc_string_String>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
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
macro_rules! background_apply_template {
    ($this:ident, $entity:ident, $termx:expr, $names:ident) => {
        $crate::decorator_apply_template! { $this, $entity, $termx, $names }
        $this.pattern.as_ref().map(|x|
            $crate::components::background::Background::set_pattern($entity, $termx, x.clone())
        );
        $this.color.map(|x| $crate::components::background::Background::set_color($entity, $termx, x));
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

    fn create_entity(&self, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        Background::new_entity(termx)
    }

    fn apply(&self, entity: Entity<Termx>, termx: &Rc<dyn IsTermx>, names: &mut NameResolver) {
        let this = self;
        background_apply_template! { this, entity, termx, names }
    }
}
