use basic_oop::{Vtable, import, class_unsafe};
use crate::components::background::Background;
use crate::components::decorator::Decorator;
use crate::components::view::View;
use crate::components::view_layout::ViewLayout;
use crate::systems::layout::{IsLayout, Layout, LayoutExt};
use crate::systems::render::{IsRender, Render, RenderExt};
use ooecs::{Component, World};
use std::cell::RefCell;
use termx_screen_base::{Bg, Fg};

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

pub struct TermxData {
    pub world: World,
    pub components: Option<TermxComponents>,
    pub systems: Option<TermxSystems>,
}

#[class_unsafe(inherits_Obj)]
pub struct Termx {
    pub data: RefCell<TermxData>,
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
    #[non_virt]
    new_background: fn() -> Entity,
    #[non_virt]
    set_background_pattern: fn(entity: Entity, value: String),
    #[non_virt]
    set_background_color: fn(entity: Entity, value: (Fg, Bg)),
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
            data: RefCell::new(TermxData {
                world: World::new(),
                components: None,
                systems: None,
            }),
        }
    }

    pub fn init_impl(this: &Rc<dyn IsTermx>) {
        this.init_components();
        this.init_systems();
    }

    pub fn init_components_impl(this: &Rc<dyn IsTermx>) {
        let termx = this.termx();
        let mut data = termx.data.borrow_mut();
        let view = Component::new::<View>(None, &mut data.world);
        let view_layout = Component::new::<ViewLayout>(Some(view), &mut data.world);
        let decorator = Component::new::<Decorator>(Some(view_layout), &mut data.world);
        let background = Component::new::<Background>(Some(decorator), &mut data.world);
        data.components = Some(TermxComponents {
            view,
            view_layout,
            decorator,
            background,
        });
    }

    pub fn init_systems_impl(this: &Rc<dyn IsTermx>) {
        let render = this.create_render();
        let layout = this.create_layout();
        let termx = this.termx();
        let mut data = termx.data.borrow_mut();
        data.systems = Some(TermxSystems {
            render,
            layout,
        });
    }

    pub fn create_render_impl(this: &Rc<dyn IsTermx>) -> Rc<dyn IsRender> {
        let termx = this.termx();
        let data = termx.data.borrow();
        Render::new(
            data.components.as_ref().unwrap().view,
            data.components.as_ref().unwrap().decorator,
            data.components.as_ref().unwrap().background,
        )
    }

    pub fn create_layout_impl(this: &Rc<dyn IsTermx>) -> Rc<dyn IsLayout> {
        let termx = this.termx();
        let data = termx.data.borrow();
        Layout::new(
            this,
            data.components.as_ref().unwrap().view_layout,
            data.components.as_ref().unwrap().view,
            data.components.as_ref().unwrap().background,
            data.components.as_ref().unwrap().decorator,
        )
    }

    pub fn set_view_layout_min_size_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Vector) {
        let termx = this.termx();
        let mut data = termx.data.borrow_mut();
        let view_layout = data.components.as_ref().unwrap().view_layout;
        entity.get_mut::<ViewLayout>(view_layout, &mut data.world).unwrap().min_size = value;
        data.systems.as_ref().unwrap().layout.clone().invalidate_measure(entity, &mut data.world);
    }

    pub fn set_view_layout_max_width_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Option<i16>) {
        let termx = this.termx();
        let mut data = termx.data.borrow_mut();
        let view_layout = data.components.as_ref().unwrap().view_layout;
        entity.get_mut::<ViewLayout>(view_layout, &mut data.world).unwrap().max_width = value;
        data.systems.as_ref().unwrap().layout.clone().invalidate_measure(entity, &mut data.world);
    }

    pub fn set_view_layout_max_height_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Option<i16>) {
        let termx = this.termx();
        let mut data = termx.data.borrow_mut();
        let view_layout = data.components.as_ref().unwrap().view_layout;
        entity.get_mut::<ViewLayout>(view_layout, &mut data.world).unwrap().max_height = value;
        data.systems.as_ref().unwrap().layout.clone().invalidate_measure(entity, &mut data.world);
    }

    pub fn set_view_layout_width_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Option<i16>) {
        let termx = this.termx();
        let mut data = termx.data.borrow_mut();
        let view_layout = data.components.as_ref().unwrap().view_layout;
        entity.get_mut::<ViewLayout>(view_layout, &mut data.world).unwrap().width = value;
        data.systems.as_ref().unwrap().layout.clone().invalidate_measure(entity, &mut data.world);
    }

    pub fn set_view_layout_height_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Option<i16>) {
        let termx = this.termx();
        let mut data = termx.data.borrow_mut();
        let view_layout = data.components.as_ref().unwrap().view_layout;
        entity.get_mut::<ViewLayout>(view_layout, &mut data.world).unwrap().height = value;
        data.systems.as_ref().unwrap().layout.clone().invalidate_measure(entity, &mut data.world);
    }

    pub fn set_view_layout_margin_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Thickness) {
        let termx = this.termx();
        let mut data = termx.data.borrow_mut();
        let view_layout = data.components.as_ref().unwrap().view_layout;
        entity.get_mut::<ViewLayout>(view_layout, &mut data.world).unwrap().margin = value;
        data.systems.as_ref().unwrap().layout.clone().invalidate_measure(entity, &mut data.world);
    }

    pub fn set_view_layout_h_align_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Option<HAlign>) {
        let termx = this.termx();
        let mut data = termx.data.borrow_mut();
        let view_layout = data.components.as_ref().unwrap().view_layout;
        entity.get_mut::<ViewLayout>(view_layout, &mut data.world).unwrap().h_align = value;
        data.systems.as_ref().unwrap().layout.clone().invalidate_measure(entity, &mut data.world);
    }

    pub fn set_view_layout_v_align_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Option<VAlign>) {
        let termx = this.termx();
        let mut data = termx.data.borrow_mut();
        let view_layout = data.components.as_ref().unwrap().view_layout;
        entity.get_mut::<ViewLayout>(view_layout, &mut data.world).unwrap().v_align = value;
        data.systems.as_ref().unwrap().layout.clone().invalidate_measure(entity, &mut data.world);
    }

    pub fn set_decorator_child_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Option<Entity>) {
        let termx = this.termx();
        let mut data = termx.data.borrow_mut();
        let decorator = data.components.as_ref().unwrap().decorator;
        let old_child = entity.get::<Decorator>(decorator, &mut data.world).unwrap().child;
        if let Some(child) = old_child {
            data.systems.as_ref().unwrap().render.clone().remove_visual_child(entity, child, &mut data.world);
        }
        entity.get_mut::<Decorator>(decorator, &mut data.world).unwrap().child = value;
        if let Some(child) = value {
            data.systems.as_ref().unwrap().render.clone().add_visual_child(entity, child, &mut data.world);
        }
        data.systems.as_ref().unwrap().layout.clone().invalidate_measure(entity, &mut data.world);
    }

    pub fn new_background_impl(this: &Rc<dyn IsTermx>) -> Entity {
        let mut data = this.termx().data.borrow_mut();
        let view = data.components.as_ref().unwrap().view;
        let view_layout = data.components.as_ref().unwrap().view_layout;
        let decorator = data.components.as_ref().unwrap().decorator;
        let background = data.components.as_ref().unwrap().background;
        let bg = Entity::new(background, &mut data.world);
        bg.add(view, &mut data.world, View::new(View::BACKGROUND));
        bg.add(view_layout, &mut data.world, ViewLayout::new());
        bg.add(decorator, &mut data.world, Decorator::new());
        bg.add(background, &mut data.world, Background::new());
        bg
    }

    pub fn set_background_pattern_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: String) {
        let termx = this.termx();
        let mut data = termx.data.borrow_mut();
        let background = data.components.as_ref().unwrap().background;
        entity.get_mut::<Background>(background, &mut data.world).unwrap().pattern = value;
        data.systems.as_ref().unwrap().render.clone().invalidate_render(entity, &mut data.world);
    }

    pub fn set_background_color_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: (Fg, Bg)) {
        let termx = this.termx();
        let mut data = termx.data.borrow_mut();
        let background = data.components.as_ref().unwrap().background;
        entity.get_mut::<Background>(background, &mut data.world).unwrap().color = value;
        data.systems.as_ref().unwrap().render.clone().invalidate_render(entity, &mut data.world);
    }
}
