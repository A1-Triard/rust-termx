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
    #[virt]
    render: fn(entity: Entity, world: &mut World, rp: &mut RenderPort),
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
        match entity.get::<View>(render.view, world).unwrap().ty {
            View::DECORATOR | View::BACKGROUND => {
                let decorator = entity.get::<Decorator>(render.decorator, world).unwrap();
                if decorator.child.is_some() { 1 } else { 0 }
            },
            _ => 0,
        }
    }

    pub fn visual_child_impl(
        this: &Rc<dyn IsRender>,
        entity: Entity,
        world: &World,
        index: usize,
    ) -> Entity {
        let render = this.render();
        match entity.get::<View>(render.view, world).unwrap().ty {
            View::DECORATOR | View::BACKGROUND => {
                let decorator = entity.get::<Decorator>(render.decorator, world).unwrap();
                assert_eq!(index, 0);
                decorator.child.unwrap()
            },
            _ => panic!(),
        }
    }

    pub render_impl(this: &Rc<dyn IsRender>, entity: Entity, world: &mut World, rp: &mut RenderPort) {
        let render = this.render();
        match entity.get::<View>(render.view, world).unwrap().ty {
            View::BACKGROUND => {
                let background = entity.get::<Background>(render.background, world).unwrap();
                rp.fill(|rp, p| rp.text(p, background.color, &background.pattern));
            },
            _ => { },
        }
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

    fn render_entity(this: &Rc<dyn IsRender>, entity: Entity, world: &World, rp: &mut RenderPort) {
        if rp.invalidated_rect.intersect(rp.bounds).is_empty() {
            return;
        }
        //if view.visibility() != Visibility::Visible { return; }
        this.render(entity, rp);
        let base_offset = rp.offset;
        let base_bounds = rp.bounds;
        for i in 0 .. this.visual_children_count(entity, world) {
            let child = this.visual_child(entity, world, i);
            let view = entity.get::<View>(render.view, world).unwrap();
            let bounds = child.margin().shrink_rect(child.render_bounds()).offset(base_offset);
            rp.bounds = bounds.intersect(base_bounds);
            rp.offset = Vector { x: bounds.l(), y: bounds.t() };
            Self::render_entity(this, &child, rp);
        }
    }


    pub fn perform_impl(_this: &Rc<dyn IsRender>, _root: Entity, _world: &mut World) {
    }
}
