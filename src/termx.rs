use basic_oop::{Vtable, import, class_unsafe};
use core::cell::{self, RefCell};
use core::ops::Deref;
use crate::components::background::Background;
use crate::components::t_button::TButton;
use crate::components::decorator::Decorator;
use crate::components::panel::Panel;
use crate::components::view::View;
use crate::components::layout_view::LayoutView;
use crate::components::view_layout::ViewLayout;
use crate::components::stack_panel::StackPanel;
use crate::components::canvas_layout::CanvasLayout;
use crate::components::canvas::Canvas;
use crate::components::focus_scope::FocusScope;
use crate::components::input_element::InputElement;
use crate::systems::layout::{IsLayout, Layout, LayoutExt};
use crate::systems::render::{IsRender, Render, RenderExt};
use crate::systems::input::{IsInput, Input, InputExt};
use ooecs::{Component, World};

import! { pub termx:
    use [obj basic_oop::obj];
    use alloc::rc::Rc;
    use int_vec_2d::{Vector, Thickness, HAlign, VAlign};
    use ooecs::{Entity};
    use termx_screen_base::{Screen, Error};
}

pub struct TermxComponents {
    pub view: Component<View, Termx>,
    pub layout_view: Component<LayoutView, Termx>,
    pub decorator: Component<Decorator, Termx>,
    pub panel: Component<Panel, Termx>,
    pub view_layout: Component<ViewLayout, Termx>,
    pub background: Component<Background, Termx>,
    pub stack_panel: Component<StackPanel, Termx>,
    pub canvas_layout: Component<CanvasLayout, Termx>,
    pub canvas: Component<Canvas, Termx>,
    pub t_button: Component<TButton, Termx>,
    pub focus_scope: Component<FocusScope, Termx>,
    pub input_element: Component<InputElement, Termx>,
}

pub struct TermxSystems {
    pub render: Rc<dyn IsRender>,
    pub layout: Rc<dyn IsLayout>,
    pub input: Rc<dyn IsInput>,
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
    pub world: RefCell<World<Termx>>,
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
    #[virt]
    create_input: fn() -> Rc<dyn IsInput>,
    #[non_virt]
    run: fn(root: Entity<Termx>, screen: &mut dyn Screen) -> Result<(), Error>,
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
        let view: Component<View, Termx> = Component::new_base(&mut world);
        let layout_view: Component<LayoutView, Termx> = Component::new(view, &mut world);
        let focus_scope: Component<FocusScope, Termx> = Component::new(layout_view, &mut world);
        let decorator: Component<Decorator, Termx> = Component::new(layout_view, &mut world);
        let panel: Component<Panel, Termx> = Component::new(layout_view, &mut world);
        let view_layout: Component<ViewLayout, Termx> = Component::new_base(&mut world);
        let background: Component<Background, Termx> = Component::new(decorator, &mut world);
        let stack_panel: Component<StackPanel, Termx> = Component::new(panel, &mut world);
        let canvas_layout: Component<CanvasLayout, Termx> = Component::new(view_layout, &mut world);
        let canvas: Component<Canvas, Termx> = Component::new(panel, &mut world);
        let input_element: Component<InputElement, Termx> = Component::new(focus_scope, &mut world);
        let t_button: Component<TButton, Termx> = Component::new(input_element, &mut world);
        termx.components.replace(Some(TermxComponents {
            view,
            layout_view,
            decorator,
            panel,
            view_layout,
            background,
            stack_panel,
            canvas_layout,
            canvas,
            t_button,
            focus_scope,
            input_element,
        }));
    }

    pub fn init_systems_impl(this: &Rc<dyn IsTermx>) {
        let render = this.create_render();
        let layout = this.create_layout();
        let input = this.create_input();
        let termx = this.termx();
        termx.systems.replace(Some(TermxSystems {
            render,
            layout,
            input,
        }));
    }

    pub fn create_render_impl(this: &Rc<dyn IsTermx>) -> Rc<dyn IsRender> {
        let termx = this.termx();
        let components = termx.components.borrow();
        Render::new(
            components.as_ref().unwrap().view,
            components.as_ref().unwrap().decorator,
            components.as_ref().unwrap().panel,
            components.as_ref().unwrap().background,
            components.as_ref().unwrap().t_button,
            components.as_ref().unwrap().focus_scope,
        )
    }

    pub fn create_input_impl(this: &Rc<dyn IsTermx>) -> Rc<dyn IsInput> {
        let termx = this.termx();
        let components = termx.components.borrow();
        let c = components.as_ref().unwrap();
        Input::new(
            this,
            c.view,
            c.decorator,
            c.panel,
            c.focus_scope,
            c.input_element,
        )
    }

    pub fn create_layout_impl(this: &Rc<dyn IsTermx>) -> Rc<dyn IsLayout> {
        let termx = this.termx();
        let components = termx.components.borrow();
        Layout::new(
            this,
            components.as_ref().unwrap().layout_view,
            components.as_ref().unwrap().view,
            components.as_ref().unwrap().background,
            components.as_ref().unwrap().decorator,
            components.as_ref().unwrap().panel,
            components.as_ref().unwrap().stack_panel,
            components.as_ref().unwrap().canvas_layout,
            components.as_ref().unwrap().t_button,
        )
    }

    pub fn run_impl(this: &Rc<dyn IsTermx>, root: Entity<Termx>, screen: &mut dyn Screen) -> Result<(), Error> {
        let termx = this.termx();
        {
            let world = termx.world.borrow();
            termx.systems().render.set_root(Some(root), &world);
            termx.systems().input.set_root(Some(root));
        }
        loop {
            let mut world = termx.world.borrow_mut();
            let screen_size = screen.size();
            termx.systems().layout.perform(root, &mut world, screen_size);
            let cursor = termx.systems().render.perform(&mut world, screen);
            if let Some(e) = screen.update(cursor, true)? {
                termx.systems().input.process(&mut world, e);
            }
        }
    }
}
