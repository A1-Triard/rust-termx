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
use crate::components::content_presenter::ContentPresenter;
use crate::components::static_text::StaticText;
use crate::components::border::Border;
use crate::components::adorners_panel::AdornersPanel;
use crate::components::control::Control;
use crate::components::group_box::GroupBox;
use crate::components::button::Button;
use crate::components::m_button::MButton;
use crate::systems::layout::{IsLayout, Layout, LayoutExt};
use crate::systems::render::{IsRender, Render, RenderExt};
use crate::systems::input::{IsInput, Input, InputExt};
use crate::systems::init::{IsInit, Init};
use crate::systems::reactive::{IsReactive, Reactive};
use ooecs::Component;

import! { pub termx:
    use [obj basic_oop::obj];
    use alloc::rc::Rc;
    use int_vec_2d::{Vector, Thickness, HAlign, VAlign};
    use ooecs::{Entity, World};
    use termx_screen_base::{Screen, Error};
    use timer_no_std::MonoClock;
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
    pub static_text: Component<StaticText, Termx>,
    pub content_presenter: Component<ContentPresenter, Termx>,
    pub border: Component<Border, Termx>,
    pub adorners_panel: Component<AdornersPanel, Termx>,
    pub control: Component<Control, Termx>,
    pub group_box: Component<GroupBox, Termx>,
    pub button: Component<Button, Termx>,
    pub m_button: Component<MButton, Termx>,
}

pub struct TermxSystems {
    pub render: Rc<dyn IsRender>,
    pub layout: Rc<dyn IsLayout>,
    pub input: Rc<dyn IsInput>,
    pub init: Rc<dyn IsInit>,
    pub reactive: Rc<dyn IsReactive>,
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
    components: RefCell<Option<TermxComponents>>,
    systems: RefCell<Option<TermxSystems>>,
    #[virt]
    init: fn(world: &mut World<Termx>),
    #[virt]
    init_components: fn(world: &mut World<Termx>),
    #[virt]
    init_systems: fn(),
    #[virt]
    create_render: fn() -> Rc<dyn IsRender>,
    #[virt]
    create_layout: fn() -> Rc<dyn IsLayout>,
    #[virt]
    create_input: fn() -> Rc<dyn IsInput>,
    #[virt]
    create_init: fn() -> Rc<dyn IsInit>,
    #[virt]
    create_reactive: fn() -> Rc<dyn IsReactive>,
    #[non_virt]
    run: fn(
        root: Entity<Termx>,
        screen: &mut dyn Screen,
        world: &mut World<Termx>,
        clock: &MonoClock,
    ) -> Result<(), Error>,
    #[virt]
    drop_entity: fn(entity: Entity<Termx>, world: &mut World<Termx>),
}

impl Termx {
    pub fn new(world: &mut World<Termx>) -> Rc<dyn IsTermx> {
        let res: Rc<dyn IsTermx> = Rc::new(unsafe { Self::new_raw(TERMX_VTABLE.as_ptr()) });
        res.init(world);
        res
    }

    pub unsafe fn new_raw(vtable: Vtable) -> Self {
        Termx {
            obj: unsafe { Obj::new_raw(vtable) },
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

    pub fn init_impl(this: &Rc<dyn IsTermx>, world: &mut World<Termx>) {
        this.init_components(world);
        this.init_systems();
    }

    pub fn init_components_impl(this: &Rc<dyn IsTermx>, world: &mut World<Termx>) {
        let termx = this.termx();
        let view: Component<View, Termx> = Component::new_base(world);
        let layout_view: Component<LayoutView, Termx> = Component::new(view, world);
        let focus_scope: Component<FocusScope, Termx> = Component::new(layout_view, world);
        let decorator: Component<Decorator, Termx> = Component::new(focus_scope, world);
        let panel: Component<Panel, Termx> = Component::new(focus_scope, world);
        let view_layout: Component<ViewLayout, Termx> = Component::new_base(world);
        let background: Component<Background, Termx> = Component::new(decorator, world);
        let stack_panel: Component<StackPanel, Termx> = Component::new(panel, world);
        let canvas_layout: Component<CanvasLayout, Termx> = Component::new(view_layout, world);
        let canvas: Component<Canvas, Termx> = Component::new(panel, world);
        let input_element: Component<InputElement, Termx> = Component::new(focus_scope, world);
        let static_text: Component<StaticText, Termx> = Component::new(layout_view, world);
        let content_presenter: Component<ContentPresenter, Termx> = Component::new(focus_scope, world);
        let border: Component<Border, Termx> = Component::new(decorator, world);
        let adorners_panel: Component<AdornersPanel, Termx> = Component::new(panel, world);
        let control: Component<Control, Termx> = Component::new(input_element, world);
        let group_box: Component<GroupBox, Termx> = Component::new(control, world);
        let button: Component<Button, Termx> = Component::new(input_element, world);
        let t_button: Component<TButton, Termx> = Component::new(button, world);
        let m_button: Component<MButton, Termx> = Component::new(button, world);
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
            static_text,
            content_presenter,
            border,
            adorners_panel,
            control,
            group_box,
            button,
            m_button,
        }));
    }

    pub fn init_systems_impl(this: &Rc<dyn IsTermx>) {
        let render = this.create_render();
        let layout = this.create_layout();
        let input = this.create_input();
        let init = this.create_init();
        let reactive = this.create_reactive();
        let termx = this.termx();
        termx.systems.replace(Some(TermxSystems {
            render,
            layout,
            input,
            init,
            reactive,
        }));
    }

    pub fn create_render_impl(this: &Rc<dyn IsTermx>) -> Rc<dyn IsRender> {
        Render::new(this)
    }

    pub fn create_input_impl(this: &Rc<dyn IsTermx>) -> Rc<dyn IsInput> {
        Input::new(this)
    }

    pub fn create_layout_impl(this: &Rc<dyn IsTermx>) -> Rc<dyn IsLayout> {
        Layout::new(this)
    }

    pub fn create_init_impl(this: &Rc<dyn IsTermx>) -> Rc<dyn IsInit> {
        Init::new(this)
    }

    pub fn create_reactive_impl(this: &Rc<dyn IsTermx>) -> Rc<dyn IsReactive> {
        Reactive::new(this)
    }

    pub fn drop_entity_impl(this: &Rc<dyn IsTermx>, entity: Entity<Termx>, world: &mut World<Termx>) {
        let termx = this.termx();
        let render = &termx.systems().render;
        assert_ne!(Some(entity), render.root(), "cannot drop the root entity");
        termx.systems().input.drop_entity(entity, world);
        entity.drop_entity(world);
    }

    pub fn run_impl(
        this: &Rc<dyn IsTermx>,
        root: Entity<Termx>,
        screen: &mut dyn Screen,
        world: &mut World<Termx>,
        clock: &MonoClock,
    ) -> Result<(), Error> {
        const FPS: u16 = 40;
        let termx = this.termx();
        termx.systems().render.set_root(Some(root), world);
        let mut wait = true;
        let mut time = clock.time();
        loop {
            let screen_size = screen.size();
            termx.systems().layout.perform(root, world, screen_size);
            let cursor = termx.systems().render.perform(world, screen);
            let e = screen.update(cursor, wait)?;
            wait = termx.systems().input.process(world, clock, e);
            let ms = time.split_ms_u16(clock).unwrap_or(u16::MAX);
            if !wait {
                assert!(FPS != 0 && u16::MAX / FPS > 8);
                clock.sleep_ms_u16((1000 / FPS).saturating_sub(ms));
            }
        }
    }
}
