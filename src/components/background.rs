use alloc::string::{String, ToString};
use alloc::rc::Rc;
use basic_oop::obj::IsObj;
use crate::components::decorator::Decorator;
use crate::components::view::*;
use crate::components::view_layout::*;
use crate::property;
use crate::template::{Template, NameResolver};
use crate::termx::IsTermx;
use dynamic_cast::dyn_cast_rc;
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

    pub fn new_entity(termx: &Rc<dyn IsTermx>) -> Entity {
        let termx = termx.termx();
        let view = termx.components().view;
        let view_layout = termx.components().view_layout;
        let decorator = termx.components().decorator;
        let background = termx.components().background;
        let mut world = termx.world.borrow_mut();
        let bg = Entity::new(background, &mut world);
        bg.add(view, &mut world, View::new(TREE_DECORATOR, RENDER_BACKGROUND));
        bg.add(view_layout, &mut world, ViewLayout::new(LAYOUT_BACKGROUND));
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

    fn create_entity(&self, world_owner: &Rc<dyn IsObj>) -> Entity {
        let termx: Rc<dyn IsTermx> = dyn_cast_rc(world_owner.clone()).unwrap();
        Background::new_entity(&termx)
    }

    fn apply(&self, entity: Entity, world_owner: &Rc<dyn IsObj>, names: &mut NameResolver) {
        let termx: Rc<dyn IsTermx> = dyn_cast_rc(world_owner.clone()).unwrap();
        let this = self;
        background_apply_template! { this, entity, &termx, names }
    }
}
