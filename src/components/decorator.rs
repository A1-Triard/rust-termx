use alloc::rc::Rc;
use crate::property_ro;
use crate::systems::layout::LayoutExt;
use crate::systems::render::RenderExt;
use crate::termx::IsTermx;
use ooecs::Entity;

pub struct Decorator {
    pub(crate) child: Option<Entity>,
}

impl Decorator {
    pub fn new() -> Self {
        Decorator { child: None }
    }

    property_ro!(Termx, decorator, child, Option<Entity>);

    pub fn set_child(entity: Entity, termx: &Rc<dyn IsTermx>, value: Option<Entity>) {
        let termx = termx.termx();
        let component = termx.components().decorator;
        let mut world = termx.world.borrow_mut();
        let old_child = entity.get::<Decorator>(component, &mut world).unwrap().child;
        if let Some(child) = old_child {
            termx.systems().render.remove_visual_child(entity, child, &mut world);
        }
        entity.get_mut::<Self>(component, &mut world).unwrap().child = value;
        if let Some(child) = value {
            termx.systems().render.add_visual_child(entity, child, &mut world);
        }
        termx.systems().layout.invalidate_measure(entity, &mut world);
    }
}
