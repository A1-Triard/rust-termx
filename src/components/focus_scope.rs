use alloc::rc::Rc;
use crate::property_rw;
use crate::systems::render::RenderExt;
use crate::termx::{Termx, IsTermx};
use ooecs::{Entity, World};

pub struct FocusScope {
    pub(crate) is_enabled_core: bool,
    pub(crate) parent_is_enabled: bool,
    pub tab_index: i8,
}

impl FocusScope {
    pub fn new() -> Self {
        FocusScope {
            is_enabled_core: true,
            parent_is_enabled: true,
            tab_index: i8::MAX,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled_core && self.parent_is_enabled
    }

    pub fn get_is_enabled(entity: Entity<Termx>, world: &World<Termx>, termx: &Rc<dyn IsTermx>) -> bool {
        let c = termx.termx().components();
        entity.get(c.focus_scope, world).unwrap().is_enabled()
    }

    pub fn set_is_enabled(
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        value: bool
    ) {
        let c = termx.termx().components();
        let focus_scope = entity.get_mut(c.focus_scope, world).unwrap();
        focus_scope.is_enabled_core = value;
        let parent_is_enabled = focus_scope.parent_is_enabled;
        if parent_is_enabled {
            let s = termx.termx().systems();
            s.render.is_enabled_changed(entity, world, value);
        }
    }

    property_rw!(Termx, focus_scope, tab_index, i8);
}

#[macro_export]
macro_rules! focus_scope_template {
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
                $(use $path as $import;)*

                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub is_enabled: Option<bool>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub tab_index: Option<i8>,
                $($(
                    $(#[$field_attr])*
                    pub $field_name : $field_ty
                ),+)?
            }
        }
    };
}

#[macro_export]
macro_rules! focus_scope_apply_template {
    ($this:ident, $entity:ident, $world:expr, $termx:expr, $names:ident) => {
        $crate::layout_view_apply_template! { $this, $entity, $world, $termx, $names }
        $this.is_enabled.map(|x|
            $crate::components::focus_scope::FocusScope::set_is_enabled($entity, $world, $termx, x)
        );
        $this.tab_index.map(|x|
            $crate::components::focus_scope::FocusScope::set_tab_index($entity, $world, $termx, x)
        );
    };
}
