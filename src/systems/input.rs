use alloc::rc::{self, Rc};
use basic_oop::{Vtable, import, class_unsafe};
use core::cell::Cell;
use crate::base::{Visibility, Thickness, Vector};
use crate::components::decorator::Decorator;
use crate::components::focus_scope::FocusScope;
use crate::components::input_element::*;
use crate::components::panel::Panel;
use crate::components::view::View;
use crate::components::t_button::TButton;
use crate::systems::render::RenderExt;
use crate::termx::IsTermx;
use ooecs::Component;
use timer_no_std::MonoTime;

pub type TimerDesc = for<'a> fn(
    entity: Entity<Termx>,
    termx: &Rc<dyn IsTermx>,
    world: &'a mut World<Termx>
) -> &'a mut Option<Timer>;

pub struct Timer {
    start: MonoTime,
    duration_ms: u16,
    action: fn(entity: Entity<Termx>, termx: &Rc<dyn IsTermx>, world: &mut World<Termx>),
    next: Option<(Entity<Termx>, TimerDesc)>,
}

import! { pub input:
    use [obj basic_oop::obj];
    use core::num::NonZeroU16;
    use crate::termx::Termx;
    use ooecs::{Entity, World};
    use termx_screen_base::{Key, Event};
    use timer_no_std::MonoClock;
}

#[class_unsafe(inherits_Obj)]
pub struct Input {
    termx: rc::Weak<dyn IsTermx>,
    pub view: Component<View, Termx>,
    pub decorator: Component<Decorator, Termx>,
    pub panel: Component<Panel, Termx>,
    pub focus_scope: Component<FocusScope, Termx>,
    pub input_element: Component<InputElement, Termx>,
    pub t_button: Component<TButton, Termx>,
    focused: Cell<Option<Entity<Termx>>>,
    timers: Cell<Option<(Entity<Termx>, TimerDesc)>>,
    #[virt]
    handle_key: fn(entity: Entity<Termx>, world: &mut World<Termx>, clock: &MonoClock, key: Key) -> bool,
    #[virt]
    process: fn(world: &mut World<Termx>, clock: &MonoClock, e: Option<Event>) -> bool,
    #[non_virt]
    allow_focus: fn(entity: Entity<Termx>, world: &World<Termx>) -> bool,
    #[non_virt]
    focused: fn() -> Option<Entity<Termx>>,
    #[non_virt]
    focus: fn(entity: Option<Entity<Termx>>, world: &mut World<Termx>),
    #[non_virt]
    focus_next: fn(world: &mut World<Termx>),
    #[non_virt]
    focus_prev: fn(world: &mut World<Termx>),
    #[non_virt]
    timer: fn(
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        clock: &MonoClock,
        desc: TimerDesc,
        duration_ms: u16,
        action: fn(entity: Entity<Termx>, termx: &Rc<dyn IsTermx>, world: &mut World<Termx>),
    ),
}

fn t_button_handle_key(
    this: &Rc<dyn IsInput>,
    entity: Entity<Termx>,
    world: &mut World<Termx>,
    clock: &MonoClock,
    key: Key,
) -> bool {
    let input = this.input();
    match key {
        Key::Enter => {
            this.timer(
                entity,
                world,
                clock, 
                |entity, termx, world| {
                    let t_button = termx.termx().components().t_button;
                    &mut entity.get_mut(t_button, world).unwrap().pressed
                },
                100,
                |entity, termx, world| {
                    let render = &termx.termx().systems().render;
                    render.invalidate_render(entity, world);
                    render.set_shadow(entity, world, Thickness::new(0, 0, 1, 1));
                    render.set_visual_offset(entity, world, Vector::null());
                },
            );
            let termx = input.termx.upgrade().unwrap();
            let render = &termx.termx().systems().render;
            render.invalidate_render(entity, world);
            render.set_shadow(entity, world, Thickness::all(0));
            render.set_visual_offset(entity, world, Vector { x: 1, y: 0 });
            true
        },
        _ => false
    }
}


impl Input {
    pub fn new(
        termx: &Rc<dyn IsTermx>,
        view: Component<View, Termx>,
        decorator: Component<Decorator, Termx>,
        panel: Component<Panel, Termx>,
        focus_scope: Component<FocusScope, Termx>,
        input_element: Component<InputElement, Termx>,
        t_button: Component<TButton, Termx>,
    ) -> Rc<dyn IsInput> {
        Rc::new(unsafe { Self::new_raw(
            termx,
            view,
            decorator,
            panel,
            focus_scope,
            input_element,
            t_button,
            INPUT_VTABLE.as_ptr(),
        ) })
    }

    pub unsafe fn new_raw(
        termx: &Rc<dyn IsTermx>,
        view: Component<View, Termx>,
        decorator: Component<Decorator, Termx>,
        panel: Component<Panel, Termx>,
        focus_scope: Component<FocusScope, Termx>,
        input_element: Component<InputElement, Termx>,
        t_button: Component<TButton, Termx>,
        vtable: Vtable,
    ) -> Self {
        Input {
            obj: unsafe { Obj::new_raw(vtable) },
            termx: Rc::downgrade(termx),
            view,
            decorator,
            panel,
            focus_scope,
            input_element,
            t_button,
            focused: Cell::new(None),
            timers: Cell::new(None),
        }
    }

    pub fn allow_focus_impl(this: &Rc<dyn IsInput>, entity: Entity<Termx>, world: &World<Termx>) -> bool {
        let input = this.input();
        let focusable = entity.get(input.input_element, world).map_or(false, |x| x.focusable);
        if !focusable { return false; }
        let is_enabled = entity.get(input.focus_scope, world).map_or(true, |x| x.is_enabled());
        if !is_enabled { return false; }
        entity.get(input.view, world).unwrap().visibility() == Visibility::Visible
    }

    pub fn focused_impl(this: &Rc<dyn IsInput>) -> Option<Entity<Termx>> {
        this.input().focused.get()
    }

    fn focus_raw(this: &Rc<dyn IsInput>, entity: Option<Entity<Termx>>, world: &mut World<Termx>) {
        let input = this.input();
        let termx = input.termx.upgrade().unwrap();
        let render = &termx.termx().systems().render;
        if let Some(prev) = input.focused.get() {
            prev.get_mut(input.input_element, world).unwrap().is_focused = false;
            render.invalidate_render(prev, world);
        }
        input.focused.set(entity);
        if let Some(next) = entity {
            next.get_mut(input.input_element, world).unwrap().is_focused = true;
            render.invalidate_render(next, world);
        }
    }

    fn move_focus(this: &Rc<dyn IsInput>, world: &mut World<Termx>, forward: bool) {
        let input = this.input();
        let focused = match input.focused.get() {
            Some(f) => f,
            None => {
                let root = input.termx.upgrade().unwrap().termx().systems().render.root().unwrap();
                if this.allow_focus(root, world) {
                    Self::focus_raw(this, Some(root), world);
                    return;
                }
                root
            },
        };
        let termx = input.termx.upgrade().unwrap();
        let render = &termx.termx().systems().render;
        let mut focus = focused;
        loop {
            if
                   focus != focused // is_enabled & visibility checked already
                || (
                       focus.get(input.focus_scope, world).unwrap().is_enabled()
                    && focus.get(input.view, world).unwrap().visibility() == Visibility::Visible
                )
            {
                let mut next_tab_index = if forward { i16::MAX } else { i16::MIN };
                let children_count = render.visual_children_count(focus, world);
                let mut next = focus;
                for i in 0 .. children_count {
                    let child = render.visual_child(focus, world, i);
                    let Some(focus_scope) = child.get(input.focus_scope, world) else { continue; };
                    if
                           !focus_scope.is_enabled()
                        || child.get(input.view, world).unwrap().visibility() != Visibility::Visible
                    {
                        continue;
                    }
                    if
                           (forward && i16::from(focus_scope.tab_index) < next_tab_index)
                        || (!forward && i16::from(focus_scope.tab_index) >= next_tab_index)
                    {
                        next_tab_index = i16::from(focus_scope.tab_index);
                        next = child;
                    }
                }
                focus = next;
            }
            if focus == focused {
                while let Some(parent) = focus.get(input.view, world).unwrap().visual_parent {
                    let tab_index = focus.get(input.focus_scope, world).unwrap().tab_index;
                    let mut before_focus = true;
                    let mut next_tab_index = if forward { i16::MAX } else { i16::MIN };
                    let mut next = focus;
                    let parent_children_count = render.visual_children_count(parent, world);
                    for i in 0 .. parent_children_count {
                        let sibling = render.visual_child(parent, world, i);
                        if before_focus && sibling == focus {
                            before_focus = false;
                            continue;
                        }
                        let Some(focus_scope) = sibling.get(input.focus_scope, world) else { continue; };
                        if
                               !focus_scope.is_enabled()
                            || sibling.get(input.view, world).unwrap().visibility() != Visibility::Visible
                        {
                            continue;
                        }
                        let candidate =
                               (forward && focus_scope.tab_index > tab_index)
                            || (!forward && focus_scope.tab_index < tab_index)
                            || (forward && !before_focus && focus_scope.tab_index == tab_index)
                            || (!forward && before_focus && focus_scope.tab_index == tab_index)
                        ;
                        if     candidate 
                            && (
                                   (forward && i16::from(focus_scope.tab_index) < next_tab_index)
                                || (!forward && i16::from(focus_scope.tab_index) >= next_tab_index)
                            )
                        {
                            next_tab_index = i16::from(focus_scope.tab_index);
                            next = sibling;
                        }
                    }
                    if next != focus {
                        focus = next;
                        break;
                    } else {
                        focus = parent;
                    }
                }
            }
            if focus == focused { break; }
            let focusable = focus.get(input.input_element, world).map_or(false, |x| x.focusable);
            if focusable {
                Self::focus_raw(this, Some(focus), world);
                break;
            }
        }
    }

    pub fn focus_impl(this: &Rc<dyn IsInput>, entity: Option<Entity<Termx>>, world: &mut World<Termx>) {
        if let Some(entity) = entity {
            let input = this.input();
            let termx = input.termx.upgrade().unwrap();
            let render = &termx.termx().systems().render;
            assert!(render.root().is_none() || render.in_tree(entity, world));
        }
        if entity.map_or(true, |x| this.allow_focus(x, world)) {
            Self::focus_raw(this, entity, world);
        }
    }

    pub fn focus_next_impl(this: &Rc<dyn IsInput>, world: &mut World<Termx>) {
        let input = this.input();
        assert!(input.termx.upgrade().unwrap().termx().systems().render.root().is_some());
        Self::move_focus(this, world, true);
    }

    pub fn focus_prev_impl(this: &Rc<dyn IsInput>, world: &mut World<Termx>) {
        let input = this.input();
        assert!(input.termx.upgrade().unwrap().termx().systems().render.root().is_some());
        Self::move_focus(this, world, false);
    }

    pub fn handle_key_impl(
        this: &Rc<dyn IsInput>,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        clock: &MonoClock,
        key: Key,
    ) -> bool {
        let input = this.input();
        match entity.get(input.input_element, world).unwrap().input() {
            INPUT_T_BUTTON => t_button_handle_key(this, entity, world, clock, key),
            _ => false
        }
    }

    pub fn timer_impl(
        this: &Rc<dyn IsInput>,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        clock: &MonoClock,
        desc: TimerDesc,
        duration_ms: u16,
        action: fn(entity: Entity<Termx>, termx: &Rc<dyn IsTermx>, world: &mut World<Termx>),
    ) {
        let input = this.input();
        let termx = input.termx.upgrade().unwrap();
        let timer = desc(entity, &termx, world);
        if let Some(existing_timer) = timer.as_mut() {
            existing_timer.start = clock.time();
            existing_timer.duration_ms = duration_ms;
            existing_timer.action = action;
            return;
        }
        *timer = Some(Timer {
            start: clock.time(),
            duration_ms,
            action,
            next: input.timers.get(),
        });
        input.timers.set(Some((entity, desc)));
    }

    pub fn process_impl(
        this: &Rc<dyn IsInput>,
        world: &mut World<Termx>,
        clock: &MonoClock,
        e: Option<Event>
    ) -> bool {
        let input = this.input();
        let termx = input.termx.upgrade().unwrap();
        assert!(termx.termx().systems().render.root().is_some());
        if let Some(timers) = input.timers.get() {
            let time = clock.time();
            let mut prev: Option<(Entity<Termx>, TimerDesc)> = None;
            let mut timer = timers;
            loop {
                let timer_ref = (timer.1)(timer.0, &termx, world);
                let timer_data = timer_ref.as_mut().unwrap();
                let next = timer_data.next;
                if time.delta_ms_u16(timer_data.start).map_or(true, |x| x >= timer_data.duration_ms) {
                    let action = timer_data.action;
                    *timer_ref = None;
                    action(timer.0, &termx, world);
                    if let Some(prev) = prev {
                        (prev.1)(prev.0, &termx, world).as_mut().unwrap().next = next;
                    } else {
                        input.timers.set(next);
                    }
                } else {
                    prev = Some(timer);
                }
                let Some(next) = next else { break; };
                timer = next;
            }
        }
        if let Some(e) = e {
            match e {
                Event::Key(n, key) => for _ in 0 .. n.get() {
                    let handled = if let Some(focused) = input.focused.get() {
                        this.handle_key(focused, world, clock, key)
                    } else {
                        false
                    };
                    if !handled {
                        match key {
                            Key::Tab => this.focus_next(world),
                            _ => { }
                        }
                    }
                },
                _ => { }
            }
        }
        input.timers.get().is_none()
    }
}
