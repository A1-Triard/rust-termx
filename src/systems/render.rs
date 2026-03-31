use alloc::rc::Rc;
use basic_oop::{Vtable, import, class_unsafe};
use core::cell::Cell;
use core::cmp::min;
use crate::base::{label_width, Visibility};
use crate::components::background::Background;
use crate::components::t_button::TButton;
use crate::components::decorator::Decorator;
use crate::components::focus_scope::FocusScope;
use crate::components::panel::Panel;
use crate::components::view::*;
use crate::render_port::RenderPort;
use int_vec_2d::{Vector, Point, Rect, Thickness, HAlign, VAlign};
use ooecs::Component;

import! { pub render:
    use [obj basic_oop::obj];
    use crate::termx::Termx;
    use ooecs::{Entity, World};
    use termx_screen_base::Screen;
}

#[class_unsafe(inherits_Obj)]
pub struct Render {
    pub view: Component<View, Termx>,
    pub decorator: Component<Decorator, Termx>,
    pub panel: Component<Panel, Termx>,
    pub background: Component<Background, Termx>,
    pub t_button: Component<TButton, Termx>,
    pub focus_scope: Component<FocusScope, Termx>,
    cursor: Cell<Option<Point>>,
    invalidated_rect: Cell<Rect>,
    screen_rect: Cell<Rect>,
    #[virt]
    visual_children_count: fn(entity: Entity<Termx>, world: &World<Termx>) -> usize,
    #[virt]
    visual_child: fn(entity: Entity<Termx>, world: &World<Termx>, index: usize) -> Entity<Termx>,
    #[virt]
    render_view: fn(entity: Entity<Termx>, world: &World<Termx>, rp: &mut RenderPort, inner_bounds: Rect),
    #[virt]
    is_enabled_changed: fn(entity: Entity<Termx>, world: &mut World<Termx>, is_enabled: bool),
    #[non_virt]
    add_visual_child: fn(parent: Entity<Termx>, child: Entity<Termx>, world: &mut World<Termx>),
    #[non_virt]
    remove_visual_child: fn(parent: Entity<Termx>, child: Entity<Termx>, world: &mut World<Termx>),
    #[non_virt]
    invalidate_render: fn(entity: Entity<Termx>, world: &World<Termx>),
    #[non_virt]
    perform: fn(root: Entity<Termx>, world: &World<Termx>, screen: &mut dyn Screen) -> Option<Point>,
}

fn render_background(
    this: &Rc<dyn IsRender>,
    entity: Entity<Termx>,
    world: &World<Termx>,
    rp: &mut RenderPort,
    _inner_bounds: Rect,
) {
    let render = this.render();
    let background = entity.get(render.background, world).unwrap();
    rp.fill(|rp, p| rp.text(p, background.color, &background.pattern));
}

fn render_t_button(
    this: &Rc<dyn IsRender>,
    entity: Entity<Termx>,
    world: &World<Termx>,
    rp: &mut RenderPort,
    inner_bounds: Rect,
) {
    let render = this.render();
    let t_button = entity.get(render.t_button, world).unwrap();
    let bg_bounds = Thickness::new(0, 0, 1, 1).shrink_rect(inner_bounds);
    let text_bounds = Thickness::new(1, 0, 1, 0).shrink_rect(bg_bounds);
    let text_align = Thickness::align(
        Vector { x: min(label_width(t_button.text()), text_bounds.w()), y: min(1, text_bounds.h()) },
        text_bounds.size,
        HAlign::Center,
        VAlign::Center
    );
    let text_bounds = text_align.shrink_rect(text_bounds);
    let bottom_shadow_bounds = Thickness::new(1, 0, 0, 0).shrink_rect(inner_bounds.b_line());
    let right_shadow_bounds = Thickness::new(0, 1, 0, 1).shrink_rect(inner_bounds.r_line());
    rp.fill_rect(bg_bounds, |rp, p| rp.text(p, t_button.color(), " "));
    rp.label_in_rect(text_bounds, t_button.color(), t_button.color_hotkey(), t_button.text());
    rp.fill_rect(bottom_shadow_bounds, |rp, p| rp.half_shadow(p, "▀"));
    rp.fill_rect(right_shadow_bounds, |rp, p| rp.half_shadow(p, "█"));
    rp.half_shadow(inner_bounds.tr_inner(), "▄");
}

impl Render {
    pub fn new(
        view: Component<View, Termx>,
        decorator: Component<Decorator, Termx>,
        panel: Component<Panel, Termx>,
        background: Component<Background, Termx>,
        t_button: Component<TButton, Termx>,
        focus_scope: Component<FocusScope, Termx>,
    ) -> Rc<dyn IsRender> {
        Rc::new(unsafe { Self::new_raw(
            view,
            decorator,
            panel,
            background,
            t_button,
            focus_scope,
            RENDER_VTABLE.as_ptr(),
        ) })
    }

    pub unsafe fn new_raw(
        view: Component<View, Termx>,
        decorator: Component<Decorator, Termx>,
        panel: Component<Panel, Termx>,
        background: Component<Background, Termx>,
        t_button: Component<TButton, Termx>,
        focus_scope: Component<FocusScope, Termx>,
        vtable: Vtable,
    ) -> Self {
        Render {
            obj: unsafe { Obj::new_raw(vtable) },
            view,
            decorator,
            panel,
            background,
            t_button,
            focus_scope,
            cursor: Cell::new(None),
            invalidated_rect: Cell::new(Rect { tl: Point { x: 0, y: 0 }, size: Vector::null() }),
            screen_rect: Cell::new(Rect { tl: Point { x: 0, y: 0 }, size: Vector::null() }),
        }
    }

    pub fn visual_children_count_impl(
        this: &Rc<dyn IsRender>,
        entity: Entity<Termx>,
        world: &World<Termx>,
    ) -> usize {
        let render = this.render();
        match entity.get(render.view, world).unwrap().tree() {
            TREE_DECORATOR => {
                let decorator = entity.get(render.decorator, world).unwrap();
                if decorator.child().is_some() { 1 } else { 0 }
            },
            TREE_PANEL => {
                let panel = entity.get(render.panel, world).unwrap();
                panel.children().len()
            },
            _ => 0,
        }
    }

    pub fn visual_child_impl(
        this: &Rc<dyn IsRender>,
        entity: Entity<Termx>,
        world: &World<Termx>,
        index: usize,
    ) -> Entity<Termx> {
        let render = this.render();
        match entity.get(render.view, world).unwrap().tree() {
            TREE_DECORATOR => {
                let decorator = entity.get(render.decorator, world).unwrap();
                assert_eq!(index, 0);
                decorator.child().unwrap()
            },
            TREE_PANEL => {
                let panel = entity.get(render.panel, world).unwrap();
                panel.children()[index]
            },
            _ => panic!(),
        }
    }

    pub fn render_view_impl(
        this: &Rc<dyn IsRender>,
        entity: Entity<Termx>,
        world: &World<Termx>,
        rp: &mut RenderPort,
        inner_bounds: Rect,
    ) {
        let render = this.render();
        match entity.get(render.view, world).unwrap().render() {
            RENDER_BACKGROUND => render_background(this, entity, world, rp, inner_bounds),
            RENDER_T_BUTTON => render_t_button(this, entity, world, rp, inner_bounds),
            _ => { },
        }
    }

    pub fn is_enabled_changed_impl(
        this: &Rc<dyn IsRender>,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        is_enabled: bool,
    ) {
        let render = this.render();
        this.invalidate_render(entity, world);
        let children_count = this.visual_children_count(entity, world);
        for i in 0 .. children_count {
            let child = this.visual_child(entity, world, i);
            let changed = if let Some(focus_scope) = child.get_mut(render.focus_scope, world) {
                if focus_scope.is_enabled_core {
                    focus_scope.parent_is_enabled = is_enabled;
                    true
                } else {
                    false
                }
            } else {
                false
            };
            if changed {
                this.is_enabled_changed(child, world, is_enabled);
            }
        }
    }

    pub fn add_visual_child_impl(
        this: &Rc<dyn IsRender>,
        parent: Entity<Termx>,
        child: Entity<Termx>,
        world: &mut World<Termx>,
    ) {
        let render = this.render();
        child.get_mut(render.view, world).unwrap().visual_parent = Some(parent);
        this.invalidate_render(child, world);
        let parent_is_enabled = if let Some(parent_focus_scope) = parent.get(render.focus_scope, world) {
            parent_focus_scope.is_enabled()
        } else {
            true
        };
        if !parent_is_enabled {
            let changed = if let Some(focus_scope) = child.get_mut(render.focus_scope, world) {
                focus_scope.parent_is_enabled = false;
                focus_scope.is_enabled_core
            } else {
                false
            };
            if changed {
                this.is_enabled_changed(child, world, false);
            }
        }
    }

    pub fn remove_visual_child_impl(
        this: &Rc<dyn IsRender>,
        parent: Entity<Termx>,
        child: Entity<Termx>,
        world: &mut World<Termx>,
    ) {
        this.invalidate_render(child, world);
        let render = this.render();
        let view = child.get_mut(render.view, world).unwrap();
        assert_eq!(view.visual_parent, Some(parent));
        view.visual_parent = None;
        let parent_is_enabled = if let Some(parent_focus_scope) = parent.get(render.focus_scope, world) {
            parent_focus_scope.is_enabled()
        } else {
            true
        };
        if !parent_is_enabled {
            let changed = if let Some(focus_scope) = child.get_mut(render.focus_scope, world) {
                focus_scope.parent_is_enabled = true;
                focus_scope.is_enabled_core
            } else {
                false
            };
            if changed {
                this.is_enabled_changed(child, world, true);
            }
        }
    }

    pub fn invalidate_render_impl(this: &Rc<dyn IsRender>, entity: Entity<Termx>, world: &World<Termx>) {
        let render = this.render();
        // TODO fix: use global bounds
        let rect = entity.get(render.view, world).unwrap().real_render_bounds;
        let union = render.invalidated_rect.get().union_intersect(rect, render.screen_rect.get());
        render.invalidated_rect.set(union);
    }

    fn render_entity(
        this: &Rc<dyn IsRender>,
        entity: Entity<Termx>,
        world: &World<Termx>,
        rp: &mut RenderPort,
    ) {
        if rp.invalidated_rect.intersect(rp.bounds).is_empty() { return; }
        let render = this.render();
        let view = entity.get(render.view, world).unwrap();
        if view.visibility() != Visibility::Visible { return; }
        let render_bounds = Rect { tl: Point { x: 0, y: 0 }, size: view.real_render_bounds.size };
        this.render_view(entity, world, rp, render_bounds);
        let base_offset = rp.offset;
        let base_bounds = rp.bounds;
        for i in 0 .. this.visual_children_count(entity, world) {
            let child = this.visual_child(entity, world, i);
            let view = child.get(render.view, world).unwrap();
            let bounds = view.real_render_bounds.offset(base_offset);
            rp.bounds = bounds.intersect(base_bounds);
            rp.offset = Vector { x: bounds.l(), y: bounds.t() };
            Self::render_entity(this, child, world, rp);
        }
    }

    pub fn perform_impl(
        this: &Rc<dyn IsRender>,
        root: Entity<Termx>,
        world: &World<Termx>,
        screen: &mut dyn Screen,
    ) -> Option<Point> {
        let render = this.render();
        let cursor = render.cursor.get();
        let mut invalidated_rect = render.invalidated_rect.replace(
            Rect { tl: Point { x: 0, y: 0 }, size: Vector::null() }
        );
        let screen_size = screen.size();
        if screen_size != render.screen_rect.get().size {
            render.screen_rect.set(Rect { tl: Point { x: 0, y: 0 }, size: screen_size });
            invalidated_rect = Rect { tl: Point { x: 0, y: 0 }, size: screen_size };
        }
        let bounds = root.get(render.view, world).unwrap().real_render_bounds;
        let mut rp = RenderPort {
            screen,
            invalidated_rect,
            bounds: bounds.intersect(Rect { tl: Point { x: 0, y: 0 }, size: screen_size }),
            offset: Vector { x: bounds.l(), y: bounds.t() },
            cursor,
        };
        Self::render_entity(this, root, world, &mut rp);
        render.cursor.set(rp.cursor);
        rp.cursor
    }
}
