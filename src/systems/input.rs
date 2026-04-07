use alloc::rc::{self, Rc};
use basic_oop::{Vtable, import, class_unsafe};
use core::cell::Cell;
use crate::base::Visibility;
use crate::components::input_element::*;
use crate::systems::render::RenderExt;
use crate::systems::reactive::ReactiveExt;
use crate::termx::IsTermx;
use timer_no_std::MonoTime;

pub type TimerDesc = for<'a> fn(
    entity: Entity<Termx>,
    world: &'a mut World<Termx>,
    termx: &Rc<dyn IsTermx>,
) -> &'a mut Option<Timer>;

pub struct Timer {
    start: MonoTime,
    duration_ms: u16,
    action: fn(entity: Entity<Termx>, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>),
    next: Option<(Entity<Termx>, TimerDesc)>,
}

import! { pub input:
    use [obj basic_oop::obj];
    use core::num::NonZeroU16;
    use crate::termx::Termx;
    use int_vec_2d::Point;
    use ooecs::{Entity, World};
    use termx_screen_base::{Key, Event};
    use timer_no_std::MonoClock;
}

#[class_unsafe(inherits_Obj)]
pub struct Input {
    termx: rc::Weak<dyn IsTermx>,
    focused: Cell<Option<Entity<Termx>>>,
    click: Cell<Option<(Option<Entity<Termx>>, Point)>>,
    timers: Cell<Option<(Entity<Termx>, TimerDesc)>>,
    #[virt]
    handle_key: fn(entity: Entity<Termx>, world: &mut World<Termx>, clock: &MonoClock, key: Key) -> bool,
    #[virt]
    handle_lmb: fn(
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        clock: &MonoClock,
        point: Point,
        down: Option<bool>,
    ) -> bool,
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
        action: fn(entity: Entity<Termx>, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>),
    ),
    #[virt]
    drop_entity: fn(entity: Entity<Termx>, world: &mut World<Termx>),
}

fn button_handle_click(
    this: &Rc<dyn IsInput>,
    entity: Entity<Termx>,
    world: &mut World<Termx>,
    clock: &MonoClock,
) {
    this.timer(
        entity,
        world,
        clock,
        |entity, world, termx| {
            let c = termx.termx().components();
            &mut entity.get_mut(c.button, world).unwrap().pressed
        },
        100,
        |entity, world, termx| {
            let c = termx.termx().components();
            if !entity.get(c.button, world).unwrap().is_mouse_pressed {
                let s = termx.termx().systems();
                s.render.invalidate_render(entity, world);
                s.reactive.button_is_pressed_changed(entity, world);
            }
        },
    );
    let input = this.input();
    let termx = input.termx.upgrade().unwrap();
    let c = termx.termx().components();
    if !entity.get(c.button, world).unwrap().is_mouse_pressed {
        let s = termx.termx().systems();
        s.render.invalidate_render(entity, world);
        s.reactive.button_is_pressed_changed(entity, world);
    }
    let mut handler = entity.get_mut(c.button, world).unwrap().click_handler.begin_invoke();
    handler.as_mut().map(|f| f(world));
    entity.get_mut(c.button, world).unwrap().click_handler.end_invoke(handler);
}

fn button_handle_lmb(
    this: &Rc<dyn IsInput>,
    entity: Entity<Termx>,
    world: &mut World<Termx>,
    clock: &MonoClock,
    _point: Point,
    down: Option<bool>,
) -> bool {
    let Some(down) = down else {
        button_handle_click(this, entity, world, clock);
        return true;
    };
    let input = this.input();
    let termx = input.termx.upgrade().unwrap();
    let c = termx.termx().components();
    let s = termx.termx().systems();
    let button = entity.get_mut(c.button, world).unwrap();
    if down {
        button.is_mouse_pressed = true;
        if button.pressed.is_none() {
            s.render.invalidate_render(entity, world);
            s.reactive.button_is_pressed_changed(entity, world);
        }
        let mut handler = entity.get_mut(c.button, world).unwrap().click_handler.begin_invoke();
        handler.as_mut().map(|f| f(world));
        entity.get_mut(c.button, world).unwrap().click_handler.end_invoke(handler);
    } else {
        button.is_mouse_pressed = false;
        if button.pressed.is_none() {
            s.render.invalidate_render(entity, world);
            s.reactive.button_is_pressed_changed(entity, world);
        }
    }
    true
}

fn button_handle_key(
    this: &Rc<dyn IsInput>,
    entity: Entity<Termx>,
    world: &mut World<Termx>,
    clock: &MonoClock,
    key: Key,
) -> bool {
    match key {
        Key::Enter => {
            button_handle_click(this, entity, world, clock);
            true
        },
        _ => false
    }
}


impl Input {
    pub fn new(termx: &Rc<dyn IsTermx>) -> Rc<dyn IsInput> {
        Rc::new(unsafe { Self::new_raw(termx, INPUT_VTABLE.as_ptr()) })
    }

    pub unsafe fn new_raw(termx: &Rc<dyn IsTermx>, vtable: Vtable) -> Self {
        Input {
            obj: unsafe { Obj::new_raw(vtable) },
            termx: Rc::downgrade(termx),
            focused: Cell::new(None),
            click: Cell::new(None),
            timers: Cell::new(None),
        }
    }

    pub fn allow_focus_impl(this: &Rc<dyn IsInput>, entity: Entity<Termx>, world: &World<Termx>) -> bool {
        let input = this.input();
        let termx = input.termx.upgrade().unwrap();
        let c = termx.termx().components();
        let focusable = entity.get(c.input_element, world).map_or(false, |x| x.focusable);
        if !focusable { return false; }
        let is_enabled = entity.get(c.focus_scope, world).map_or(true, |x| x.is_enabled());
        if !is_enabled { return false; }
        entity.get(c.view, world).unwrap().visibility() == Visibility::Visible
    }

    pub fn focused_impl(this: &Rc<dyn IsInput>) -> Option<Entity<Termx>> {
        this.input().focused.get()
    }

    fn focus_raw(this: &Rc<dyn IsInput>, entity: Option<Entity<Termx>>, world: &mut World<Termx>) {
        let input = this.input();
        let termx = input.termx.upgrade().unwrap();
        let c = termx.termx().components();
        let render = &termx.termx().systems().render;
        if let Some(prev) = input.focused.get() {
            prev.get_mut(c.input_element, world).unwrap().is_focused = false;
            render.invalidate_render(prev, world);
        }
        input.focused.set(entity);
        if let Some(next) = entity {
            next.get_mut(c.input_element, world).unwrap().is_focused = true;
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
        let c = termx.termx().components();
        let mut focus = focused;
        loop {
            let next = if
                   focus != focused // is_enabled & visibility checked already
                || (
                       focus.get(c.focus_scope, world).unwrap().is_enabled()
                    && focus.get(c.view, world).unwrap().visibility() == Visibility::Visible
                )
            {
                let mut next_tab_index = if forward { i16::MAX } else { i16::MIN };
                let children_count = render.visual_children_count(focus, world);
                let mut next = None;
                for i in 0 .. children_count {
                    let child = render.visual_child(focus, world, i);
                    let Some(focus_scope) = child.get(c.focus_scope, world) else { continue; };
                    if
                           !focus_scope.is_enabled()
                        || child.get(c.view, world).unwrap().visibility() != Visibility::Visible
                    {
                        continue;
                    }
                    if
                           (forward && i16::from(focus_scope.tab_index) < next_tab_index)
                        || (!forward && i16::from(focus_scope.tab_index) >= next_tab_index)
                    {
                        next_tab_index = i16::from(focus_scope.tab_index);
                        next = Some(child);
                    }
                }
                next
            } else {
                None
            };
            if let Some(next) = next {
                focus = next;
            } else {
                while let Some(parent) = focus.get(c.view, world).unwrap().visual_parent {
                    let tab_index = focus.get(c.focus_scope, world).unwrap().tab_index;
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
                        let Some(focus_scope) = sibling.get(c.focus_scope, world) else { continue; };
                        if
                               !focus_scope.is_enabled()
                            || sibling.get(c.view, world).unwrap().visibility() != Visibility::Visible
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
            let focusable = focus.get(c.input_element, world).map_or(false, |x| x.focusable);
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
        let termx = input.termx.upgrade().unwrap();
        let c = termx.termx().components();
        match entity.get(c.input_element, world).unwrap().input() {
            INPUT_BUTTON => button_handle_key(this, entity, world, clock, key),
            _ => false
        }
    }

    pub fn handle_lmb_impl(
        this: &Rc<dyn IsInput>,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        clock: &MonoClock,
        point: Point,
        down: Option<bool>,
    ) -> bool {
        let input = this.input();
        let termx = input.termx.upgrade().unwrap();
        let c = termx.termx().components();
        match entity.get(c.input_element, world).unwrap().input() {
            INPUT_BUTTON => button_handle_lmb(this, entity, world, clock, point, down),
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
        action: fn(entity: Entity<Termx>, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>),
    ) {
        let input = this.input();
        let termx = input.termx.upgrade().unwrap();
        let timer = desc(entity, world, &termx);
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

    pub fn drop_entity_impl(this: &Rc<dyn IsInput>, entity: Entity<Termx>, world: &mut World<Termx>) {
        let input = this.input();
        let termx = input.termx.upgrade().unwrap();
        if let Some(mut focused) = input.focused.get() {
            let c = termx.termx().components();
            let clear_focus = loop {
                if focused == entity { break true; }
                let Some(parent) = focused.get(c.view, world).unwrap().visual_parent else { break false; };
                focused = parent;
            };
            if clear_focus {
                this.focus(None, world);
            }
        }
        if let Some(click) = input.click.get() && click.0 == Some(entity) {
            input.click.set(Some((None, click.1)));
        }
        let mut prev: Option<(Entity<Termx>, TimerDesc)> = None;
        let mut cur = input.timers.get();
        while let Some(timer) = cur {
            let timer_data = (timer.1)(timer.0, world, &termx);
            let next = timer_data.as_ref().unwrap().next;
            if timer.0 == entity {
                *timer_data = None;
                if let Some(prev) = prev {
                    (prev.1)(prev.0, world, &termx).as_mut().unwrap().next = next;
                } else {
                    input.timers.set(next);
                }
            } else {
                prev = Some(timer);
            }
            cur = next;
        }
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
                let timer_ref = (timer.1)(timer.0, world, &termx);
                let timer_data = timer_ref.as_mut().unwrap();
                let next = timer_data.next;
                if time.delta_ms_u16(timer_data.start).map_or(true, |x| x >= timer_data.duration_ms) {
                    let action = timer_data.action;
                    *timer_ref = None;
                    action(timer.0, world, &termx);
                    if let Some(prev) = prev {
                        (prev.1)(prev.0, world, &termx).as_mut().unwrap().next = next;
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
                Event::LmbDown(point) => {
                    let render = &termx.termx().systems().render;
                    if let Some(entity) = render.hit_test_input_element(point, world) {
                        let point = render.point_from_screen(entity, world, point);
                        input.click.set(Some((Some(entity), point)));
                        Self::focus_raw(this, Some(entity), world);
                        this.handle_lmb(entity, world, clock, point, Some(true));
                    }
                },
                Event::LmbUp(point) => {
                    if let Some((entity, point)) = input.click.take() {
                        if let Some(entity) = entity {
                            this.handle_lmb(entity, world, clock, point, Some(false));
                        }
                    } else {
                        let render = &termx.termx().systems().render;
                        if let Some(entity) = render.hit_test_input_element(point, world) {
                            Self::focus_raw(this, Some(entity), world);
                            let point = render.point_from_screen(entity, world, point);
                            this.handle_lmb(entity, world, clock, point, None);
                        }
                    }
                },
                _ => { }
            }
        }
        input.timers.get().is_none()
    }
}
