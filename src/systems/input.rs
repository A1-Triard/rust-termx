use alloc::rc::{self, Rc};
use basic_oop::{Vtable, import, class_unsafe};
use core::cell::Cell;
use crate::base::Visibility;
use crate::components::decorator::Decorator;
use crate::components::focus_scope::FocusScope;
use crate::components::input_element::InputElement;
use crate::components::panel::Panel;
use crate::components::view::View;
use crate::systems::render::RenderExt;
use crate::termx::IsTermx;
use ooecs::Component;

import! { pub input:
    use [obj basic_oop::obj];
    use core::num::NonZeroU16;
    use crate::termx::Termx;
    use ooecs::{Entity, World};
    use termx_screen_base::{Key, Event};
}

#[class_unsafe(inherits_Obj)]
pub struct Input {
    termx: rc::Weak<dyn IsTermx>,
    pub view: Component<View, Termx>,
    pub decorator: Component<Decorator, Termx>,
    pub panel: Component<Panel, Termx>,
    pub focus_scope: Component<FocusScope, Termx>,
    pub input_element: Component<InputElement, Termx>,
    focused: Cell<Option<Entity<Termx>>>,
    root: Cell<Option<Entity<Termx>>>,
    #[virt]
    handle_key: fn(entity: Entity<Termx>, world: &mut World<Termx>, key: Key) -> bool,
    #[virt]
    process: fn(world: &mut World<Termx>, e: Event),
    #[non_virt]
    set_root: fn(root: Option<Entity<Termx>>),
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
}

impl Input {
    pub fn new(
        termx: &Rc<dyn IsTermx>,
        view: Component<View, Termx>,
        decorator: Component<Decorator, Termx>,
        panel: Component<Panel, Termx>,
        focus_scope: Component<FocusScope, Termx>,
        input_element: Component<InputElement, Termx>,
    ) -> Rc<dyn IsInput> {
        Rc::new(unsafe { Self::new_raw(
            termx,
            view,
            decorator,
            panel,
            focus_scope,
            input_element,
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
            focused: Cell::new(None),
            root: Cell::new(None),
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

    pub fn set_root_impl(this: &Rc<dyn IsInput>, root: Option<Entity<Termx>>) {
        this.input().root.set(root);
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
                let root = input.root.get().unwrap();
                if this.allow_focus(root, world) {
                    Self::focus_raw(this, Some(root), world);
                }
                return;
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
                let mut next_tab_index = if forward { i8::MAX } else { i8::MIN };
                let children_count = render.visual_children_count(focus, world);
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
                           (forward && focus_scope.tab_index < next_tab_index)
                        || (!forward && focus_scope.tab_index >= next_tab_index)
                    {
                        next_tab_index = focus_scope.tab_index;
                        focus = child;
                    }
                }
            }
            if focus == focused {
                while let Some(parent) = focus.get(input.view, world).unwrap().visual_parent {
                    let tab_index = focus.get(input.focus_scope, world).unwrap().tab_index;
                    let mut before_focus = true;
                    let mut next_tab_index = if forward { i8::MAX } else { i8::MIN };
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
                                   (forward && focus_scope.tab_index < next_tab_index)
                                || (!forward && focus_scope.tab_index >= next_tab_index)
                            )
                        {
                            next_tab_index = focus_scope.tab_index;
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
        if entity.map_or(true, |x| this.allow_focus(x, world)) {
            Self::focus_raw(this, entity, world);
        }
    }

    pub fn focus_next_impl(this: &Rc<dyn IsInput>, world: &mut World<Termx>) {
        Self::move_focus(this, world, true);
    }

    pub fn focus_prev_impl(this: &Rc<dyn IsInput>, world: &mut World<Termx>) {
        Self::move_focus(this, world, false);
    }

    pub fn handle_key_impl(
        _this: &Rc<dyn IsInput>,
        _entity: Entity<Termx>,
        _world: &mut World<Termx>,
        _key: Key
    ) -> bool {
        false
    }

    pub fn process_impl(this: &Rc<dyn IsInput>, world: &mut World<Termx>, e: Event) {
        match e {
            Event::Key(n, key) => for _ in 0 .. n.get() {
                let input = this.input();
                if let Some(focused) = input.focused.get() {
                    if !this.handle_key(focused, world, key) {
                        match key {
                            Key::Tab => this.focus_next(world),
                            _ => { }
                        }
                    }
                }
            },
            _ => { }
        }
    }
}
