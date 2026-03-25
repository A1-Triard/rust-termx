use basic_oop::{Vtable, import, class_unsafe};
use crate::components::decorator::Decorator;
use crate::components::view::View;
use ooecs::Component;
use std::rc::Rc;

import! { pub render:
    use [obj basic_oop::obj];
    use ooecs::{Entity, World};
}

#[class_unsafe(inherits_Obj)]
pub struct Render {
    pub view: Component,
    pub decorator: Component,
    #[virt]
    visual_children_count: fn(entity: Entity, world: &World) -> usize,
    #[virt]
    visual_child: fn(entity: Entity, world: &World, index: usize) -> Entity,
    #[non_virt]
    add_visual_child: fn(parent: Entity, child: Entity, world: &mut World),
    #[non_virt]
    remove_visual_child: fn(parent: Entity, child: Entity, world: &mut World),
    #[non_virt]
    invalidate_render: fn(entity: Entity, world: &mut World),
    #[non_virt]
    perform: fn(root: Entity, world: &mut World),
}

impl Render {
    pub fn new(view: Component, decorator: Component) -> Rc<dyn IsRender> {
        Rc::new(unsafe { Self::new_raw(view, decorator, RENDER_VTABLE.as_ptr()) })
    }

    pub unsafe fn new_raw(view: Component, decorator: Component, vtable: Vtable) -> Self {
        Render {
            obj: unsafe { Obj::new_raw(vtable) },
            view,
            decorator,
        }
    }

    pub fn visual_children_count_impl(this: &Rc<dyn IsRender>, entity: Entity, world: &World) -> usize {
        let render = this.render();
        if let Some(decorator) = entity.get::<Decorator>(render.decorator, world) {
            return if decorator.child.is_some() { 1 } else { 0 };
        }
        0
    }

    pub fn visual_child_impl(
        this: &Rc<dyn IsRender>,
        entity: Entity,
        world: &World,
        index: usize,
    ) -> Entity {
        let render = this.render();
        if let Some(decorator) = entity.get::<Decorator>(render.decorator, world) {
            assert_eq!(index, 0);
            return decorator.child.unwrap();
        }
        panic!()
    }

    pub fn add_visual_child_impl(this: &Rc<dyn IsRender>, parent: Entity, child: Entity, world: &mut World) {
        let render = this.render();
        child.get_mut::<View>(render.view, world).unwrap().visual_parent = Some(parent);
        this.invalidate_render(child, world);
    }

    pub fn remove_visual_child_impl(
        this: &Rc<dyn IsRender>,
        parent: Entity,
        child: Entity,
        world: &mut World
    ) {
        this.invalidate_render(child, world);
        let render = this.render();
        let view = child.get_mut::<View>(render.view, world).unwrap();
        assert_eq!(view.visual_parent, Some(parent));
        view.visual_parent = None;
    }

    pub fn invalidate_render_impl(_this: &Rc<dyn IsRender>, _entity: Entity, _world: &mut World) {
    }

    pub fn perform_impl(_this: &Rc<dyn IsRender>, _root: Entity, _world: &mut World) {
    }
}
