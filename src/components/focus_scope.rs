use alloc::rc::Rc;
use crate::systems::render::RenderExt;
use crate::termx::{Termx, IsTermx};
use ooecs::Entity;

pub struct FocusScope {
    pub(crate) is_enabled_core: bool,
    pub(crate) parent_is_enabled: bool,
}

impl FocusScope {
    pub fn new() -> Self {
        FocusScope {
            is_enabled_core: true,
            parent_is_enabled: true,
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
}
