use basic_oop::{Vtable, import, class_unsafe};
use crate::components::background::Background;
use crate::components::decorator::Decorator;
use crate::components::view::View;
use crate::components::view_layout::ViewLayout;
use crate::systems::layout::{IsLayout, Layout, LayoutExt};
use crate::systems::render::{IsRender, Render, RenderExt};
use ooecs::{Component, World};
use std::cell::RefCell;

import! { pub termx:
    use [obj basic_oop::obj];
    use int_vec_2d::{Vector, Thickness, HAlign, VAlign};
    use std::rc::Rc;
    use ooecs::{Entity};
}

pub struct TermxComponents {
    pub view: Component,
    pub view_layout: Component,
    pub decorator: Component,
    pub background: Component,
}

pub struct TermxSystems {
    pub render: Rc<dyn IsRender>,
    pub layout: Rc<dyn IsLayout>,
}

#[class_unsafe(inherits_Obj)]
pub struct Termx {
    pub world: RefCell<World>,
    pub components: RefCell<Option<TermxComponents>>,
    pub systems: RefCell<Option<TermxSystems>>,
    #[virt]
    init: fn(),
    #[virt]
    init_components: fn(),
    #[virt]
    init_systems: fn(),
    #[virt]
    create_render: fn() -> Rc<dyn IsRender>,
    #[virt]
    create_layout: fn() -> Rc<dyn IsLayout>,
    #[non_virt]
    set_view_layout_min_size: fn(entity: Entity, value: Vector),
    #[non_virt]
    set_view_layout_max_width: fn(entity: Entity, value: Option<i16>),
    #[non_virt]
    set_view_layout_max_height: fn(entity: Entity, value: Option<i16>),
    #[non_virt]
    set_view_layout_width: fn(entity: Entity, value: Option<i16>),
    #[non_virt]
    set_view_layout_height: fn(entity: Entity, value: Option<i16>),
    #[non_virt]
    set_view_layout_margin: fn(entity: Entity, value: Thickness),
    #[non_virt]
    set_view_layout_h_align: fn(entity: Entity, value: Option<HAlign>),
    #[non_virt]
    set_view_layout_v_align: fn(entity: Entity, value: Option<VAlign>),
    #[non_virt]
    set_decorator_child: fn(entity: Entity, value: Option<Entity>),
}

impl Termx {
    pub fn new() -> Rc<dyn IsTermx> {
        let res: Rc<dyn IsTermx> = Rc::new(unsafe { Self::new_raw(TERMX_VTABLE.as_ptr()) });
        res.init();
        res
    }

    pub unsafe fn new_raw(vtable: Vtable) -> Self {
        Termx {
            obj: unsafe { Obj::new_raw(vtable) },
            world: RefCell::new(World::new()),
            components: RefCell::new(None),
            systems: RefCell::new(None),
        }
    }

    pub fn init_impl(this: &Rc<dyn IsTermx>) {
        this.init_components();
        this.init_systems();
    }

    pub fn init_components_impl(this: &Rc<dyn IsTermx>) {
        let termx = this.termx();
        let mut world = termx.world.borrow_mut();
        *termx.components.borrow_mut() = Some(TermxComponents {
            view: Component::new::<View>(&mut world),
            view_layout: Component::new::<ViewLayout>(&mut world),
            decorator: Component::new::<Decorator>(&mut world),
            background: Component::new::<Background>(&mut world),
        });
    }

    pub fn init_systems_impl(this: &Rc<dyn IsTermx>) {
        let render = this.create_render();
        let layout = this.create_layout();
        let termx = this.termx();
        *termx.systems.borrow_mut() = Some(TermxSystems {
            render,
            layout,
        });
    }

    pub fn create_render_impl(this: &Rc<dyn IsTermx>) -> Rc<dyn IsRender> {
        let termx = this.termx();
        let components = termx.components.borrow();
        Render::new(components.as_ref().unwrap().view, components.as_ref().unwrap().decorator)
    }

    pub fn create_layout_impl(this: &Rc<dyn IsTermx>) -> Rc<dyn IsLayout> {
        let termx = this.termx();
        let components = termx.components.borrow();
        Layout::new(
            this,
            components.as_ref().unwrap().view_layout,
            components.as_ref().unwrap().view,
            components.as_ref().unwrap().background,
            components.as_ref().unwrap().decorator,
        )
    }

    pub fn set_view_layout_min_size_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Vector) {
        let termx = this.termx();
        let mut world = termx.world.borrow_mut();
        let view_layout = termx.components.borrow().as_ref().unwrap().view_layout;
        entity.component::<ViewLayout>(&mut world, view_layout).unwrap().min_size = value;
        termx.systems.borrow().as_ref().unwrap().layout.invalidate_measure(entity, &mut world);
    }

    pub fn set_view_layout_max_width_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Option<i16>) {
        let termx = this.termx();
        let mut world = termx.world.borrow_mut();
        let view_layout = termx.components.borrow().as_ref().unwrap().view_layout;
        entity.component::<ViewLayout>(&mut world, view_layout).unwrap().max_width = value;
        termx.systems.borrow().as_ref().unwrap().layout.invalidate_measure(entity, &mut world);
    }

    pub fn set_view_layout_max_height_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Option<i16>) {
        let termx = this.termx();
        let mut world = termx.world.borrow_mut();
        let view_layout = termx.components.borrow().as_ref().unwrap().view_layout;
        entity.component::<ViewLayout>(&mut world, view_layout).unwrap().max_height = value;
        termx.systems.borrow().as_ref().unwrap().layout.invalidate_measure(entity, &mut world);
    }

    pub fn set_view_layout_width_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Option<i16>) {
        let termx = this.termx();
        let mut world = termx.world.borrow_mut();
        let view_layout = termx.components.borrow().as_ref().unwrap().view_layout;
        entity.component::<ViewLayout>(&mut world, view_layout).unwrap().width = value;
        termx.systems.borrow().as_ref().unwrap().layout.invalidate_measure(entity, &mut world);
    }

    pub fn set_view_layout_height_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Option<i16>) {
        let termx = this.termx();
        let mut world = termx.world.borrow_mut();
        let view_layout = termx.components.borrow().as_ref().unwrap().view_layout;
        entity.component::<ViewLayout>(&mut world, view_layout).unwrap().height = value;
        termx.systems.borrow().as_ref().unwrap().layout.invalidate_measure(entity, &mut world);
    }

    pub fn set_view_layout_margin_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Thickness) {
        let termx = this.termx();
        let mut world = termx.world.borrow_mut();
        let view_layout = termx.components.borrow().as_ref().unwrap().view_layout;
        entity.component::<ViewLayout>(&mut world, view_layout).unwrap().margin = value;
        termx.systems.borrow().as_ref().unwrap().layout.invalidate_measure(entity, &mut world);
    }

    pub fn set_view_layout_h_align_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Option<HAlign>) {
        let termx = this.termx();
        let mut world = termx.world.borrow_mut();
        let view_layout = termx.components.borrow().as_ref().unwrap().view_layout;
        entity.component::<ViewLayout>(&mut world, view_layout).unwrap().h_align = value;
        termx.systems.borrow().as_ref().unwrap().layout.invalidate_measure(entity, &mut world);
    }

    pub fn set_view_layout_v_align_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Option<VAlign>) {
        let termx = this.termx();
        let mut world = termx.world.borrow_mut();
        let view_layout = termx.components.borrow().as_ref().unwrap().view_layout;
        entity.component::<ViewLayout>(&mut world, view_layout).unwrap().v_align = value;
        termx.systems.borrow().as_ref().unwrap().layout.invalidate_measure(entity, &mut world);
    }

    pub fn set_decorator_child_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Option<Entity>) {
        let termx = this.termx();
        let mut world = termx.world.borrow_mut();
        let decorator = termx.components.borrow().as_ref().unwrap().decorator;
        let old_child = entity.component::<Decorator>(&mut world, decorator).unwrap().child;
        if let Some(child) = old_child {
            termx.systems.borrow().as_ref().unwrap().render.remove_visual_child(entity, child, &mut world);
        }
        entity.component::<Decorator>(&mut world, decorator).unwrap().child = value;
        if let Some(child) = value {
            termx.systems.borrow().as_ref().unwrap().render.add_visual_child(entity, child, &mut world);
        }
        termx.systems.borrow().as_ref().unwrap().layout.invalidate_measure(entity, &mut world);
    }
}
