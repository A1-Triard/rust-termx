use basic_oop::{Vtable, import, class_unsafe};
use core::cell::{self, RefCell};
use core::ops::Deref;
use crate::components::background::Background;
use crate::components::decorator::Decorator;
use crate::components::view::View;
use crate::components::view_layout::ViewLayout;
use crate::systems::layout::{IsLayout, Layout, LayoutExt};
use crate::systems::render::{IsRender, Render, RenderExt};
use ooecs::{Component, World};

import! { pub termx:
    use [obj basic_oop::obj];
    use alloc::rc::Rc;
    use int_vec_2d::{Vector, Thickness, HAlign, VAlign};
    use ooecs::{Entity};
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
}
