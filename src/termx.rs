use basic_oop::{Vtable, import, class_unsafe};
use crate::components::background::Background;
use crate::components::decorator::Decorator;
use crate::components::view::View;
use crate::components::view_layout::ViewLayout;
use crate::systems::layout::{IsLayout, Layout, LayoutExt};
use crate::systems::render::{IsRender, Render, RenderExt};
use ooecs::{Component, World};
use std::cell::{self, RefCell};
use std::ops::Deref;
use termx_screen_base::{Bg, Fg};

import! { pub termx:
    use [obj basic_oop::obj];
    use int_vec_2d::{Vector, Thickness, HAlign, VAlign};
    use ooecs::{Entity};
    use std::rc::Rc;
    use termx_screen_base::{Screen, Error};
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

pub struct Ref<'a, T>(cell::Ref<'a, Option<T>>);

impl<'a, T> Deref for Ref<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.0.as_ref().unwrap()
    }
}

#[class_unsafe(inherits_Obj)]
pub struct Termx {
    pub world: RefCell<World>,
    components: RefCell<Option<TermxComponents>>,
    systems: RefCell<Option<TermxSystems>>,
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
    run: fn(root: Entity, screen: &mut dyn Screen) -> Result<(), Error>,
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
            world: RefCell::new(World::new()),
            components: RefCell::new(None),
            systems: RefCell::new(None),
        }
    }

    pub fn components(&self) -> Ref<'_, TermxComponents> {
        Ref(self.components.borrow())
    }

    pub fn systems(&self) -> Ref<'_, TermxSystems> {
        Ref(self.systems.borrow())
    }

    pub fn init_impl(this: &Rc<dyn IsTermx>) {
        this.init_components();
        this.init_systems();
    }

    pub fn init_components_impl(this: &Rc<dyn IsTermx>) {
        let termx = this.termx();
        let mut world = termx.world.borrow_mut();
        let view = Component::new::<View>(None, &mut world);
        let view_layout = Component::new::<ViewLayout>(Some(view), &mut world);
        let decorator = Component::new::<Decorator>(Some(view_layout), &mut world);
        let background = Component::new::<Background>(Some(decorator), &mut world);
        termx.components.replace(Some(TermxComponents {
            view,
            view_layout,
            decorator,
            background,
        }));
    }

    pub fn init_systems_impl(this: &Rc<dyn IsTermx>) {
        let render = this.create_render();
        let layout = this.create_layout();
        let termx = this.termx();
        termx.systems.replace(Some(TermxSystems {
            render,
            layout,
        }));
    }

    pub fn create_render_impl(this: &Rc<dyn IsTermx>) -> Rc<dyn IsRender> {
        let termx = this.termx();
        let components = termx.components.borrow();
        Render::new(
            components.as_ref().unwrap().view,
            components.as_ref().unwrap().decorator,
            components.as_ref().unwrap().background,
        )
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

    pub fn run_impl(this: &Rc<dyn IsTermx>, root: Entity, screen: &mut dyn Screen) -> Result<(), Error> {
        let termx = this.termx();
        loop {
            let screen_size = screen.size();
            let mut world = termx.world.borrow_mut();
            termx.systems().layout.perform(root, &mut world, screen_size);
            let cursor = termx.systems().render.perform(root, &mut world, screen);
            screen.update(cursor, true)?;
        }
    }

    pub fn set_view_layout_min_size_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Vector) {
        let termx = this.termx();
        let view_layout = termx.components().view_layout;
        let mut world = termx.world.borrow_mut();
        entity.get_mut::<ViewLayout>(view_layout, &mut world).unwrap().min_size = value;
        termx.systems().layout.invalidate_measure(entity, &mut world);
    }

    pub fn set_view_layout_max_width_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Option<i16>) {
        let termx = this.termx();
        let view_layout = termx.components().view_layout;
        let mut world = termx.world.borrow_mut();
        entity.get_mut::<ViewLayout>(view_layout, &mut world).unwrap().max_width = value;
        termx.systems().layout.invalidate_measure(entity, &mut world);
    }

    pub fn set_view_layout_max_height_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Option<i16>) {
        let termx = this.termx();
        let view_layout = termx.components().view_layout;
        let mut world = termx.world.borrow_mut();
        entity.get_mut::<ViewLayout>(view_layout, &mut world).unwrap().max_height = value;
        termx.systems().layout.invalidate_measure(entity, &mut world);
    }

    pub fn set_view_layout_width_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Option<i16>) {
        let termx = this.termx();
        let view_layout = termx.components().view_layout;
        let mut world = termx.world.borrow_mut();
        entity.get_mut::<ViewLayout>(view_layout, &mut world).unwrap().width = value;
        termx.systems().layout.invalidate_measure(entity, &mut world);
    }

    pub fn set_view_layout_height_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Option<i16>) {
        let termx = this.termx();
        let view_layout = termx.components().view_layout;
        let mut world = termx.world.borrow_mut();
        entity.get_mut::<ViewLayout>(view_layout, &mut world).unwrap().height = value;
        termx.systems().layout.invalidate_measure(entity, &mut world);
    }

    pub fn set_view_layout_margin_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Thickness) {
        let termx = this.termx();
        let view_layout = termx.components().view_layout;
        let mut world = termx.world.borrow_mut();
        entity.get_mut::<ViewLayout>(view_layout, &mut world).unwrap().margin = value;
        termx.systems().layout.invalidate_measure(entity, &mut world);
    }

    pub fn set_view_layout_h_align_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Option<HAlign>) {
        let termx = this.termx();
        let view_layout = termx.components().view_layout;
        let mut world = termx.world.borrow_mut();
        entity.get_mut::<ViewLayout>(view_layout, &mut world).unwrap().h_align = value;
        termx.systems().layout.invalidate_measure(entity, &mut world);
    }

    pub fn set_view_layout_v_align_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Option<VAlign>) {
        let termx = this.termx();
        let view_layout = termx.components().view_layout;
        let mut world = termx.world.borrow_mut();
        entity.get_mut::<ViewLayout>(view_layout, &mut world).unwrap().v_align = value;
        termx.systems().layout.invalidate_measure(entity, &mut world);
    }

    pub fn set_decorator_child_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: Option<Entity>) {
        let termx = this.termx();
        let decorator = termx.components().decorator;
        let mut world = termx.world.borrow_mut();
        let old_child = entity.get::<Decorator>(decorator, &mut world).unwrap().child;
        if let Some(child) = old_child {
            termx.systems().render.remove_visual_child(entity, child, &mut world);
        }
        entity.get_mut::<Decorator>(decorator, &mut world).unwrap().child = value;
        if let Some(child) = value {
            termx.systems().render.add_visual_child(entity, child, &mut world);
        }
        termx.systems().layout.invalidate_measure(entity, &mut world);
    }

    pub fn new_background_impl(this: &Rc<dyn IsTermx>) -> Entity {
        let termx = this.termx();
        let view = termx.components().view;
        let view_layout = termx.components().view_layout;
        let decorator = termx.components().decorator;
        let background = termx.components().background;
        let mut world = termx.world.borrow_mut();
        let bg = Entity::new(background, &mut world);
        bg.add(view, &mut world, View::new(View::BACKGROUND));
        bg.add(view_layout, &mut world, ViewLayout::new());
        bg.add(decorator, &mut world, Decorator::new());
        bg.add(background, &mut world, Background::new());
        bg
    }

    pub fn set_background_pattern_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: String) {
        let termx = this.termx();
        let background = termx.components().background;
        let mut world = termx.world.borrow_mut();
        entity.get_mut::<Background>(background, &mut world).unwrap().pattern = value;
        termx.systems().render.invalidate_render(entity, &mut world);
    }

    pub fn set_background_color_impl(this: &Rc<dyn IsTermx>, entity: Entity, value: (Fg, Bg)) {
        let termx = this.termx();
        let background = termx.components().background;
        let mut world = termx.world.borrow_mut();
        entity.get_mut::<Background>(background, &mut world).unwrap().color = value;
        termx.systems().render.invalidate_render(entity, &mut world);
    }
}
