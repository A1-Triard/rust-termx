use alloc::rc::Rc;
use crate::property_rw;
use crate::systems::render::RenderExt;
use crate::termx::{Termx, IsTermx};
use ooecs::Entity;

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

    pub fn get_is_enabled(entity: Entity<Termx>, termx: &Rc<dyn IsTermx>) -> bool {
        let termx = termx.termx();
        let focus_scope = termx.components().focus_scope;
        let world = termx.world.borrow();
        entity.get(focus_scope, &world).unwrap().is_enabled()
    }

    pub fn set_is_enabled(entity: Entity<Termx>, termx: &Rc<dyn IsTermx>, value: bool) {
        let termx = termx.termx();
        let focus_scope = termx.components().focus_scope;
        let mut world = termx.world.borrow_mut();
        let component = entity.get_mut(focus_scope, &mut world).unwrap();
        component.is_enabled_core = value;
        let parent_is_enabled = component.parent_is_enabled;
        if parent_is_enabled {
            termx.systems().render.is_enabled_changed(entity, &mut world, value);
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
    ($this:ident, $entity:ident, $termx:expr, $names:ident) => {
        $crate::layout_view_apply_template! { $this, $entity, $termx, $names }
        $this.is_enabled.map(|x|
            $crate::components::focus_scope::FocusScope::set_is_enabled($entity, $termx, x)
        );
        $this.tab_index.map(|x|
            $crate::components::focus_scope::FocusScope::set_tab_index($entity, $termx, x)
        );
    };
}
