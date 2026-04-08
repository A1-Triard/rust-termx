use alloc::rc::{self, Rc};
use basic_oop::{Vtable, import, class_unsafe};
use core::cell::Cell;
use core::cmp::min;
use crate::base::{label_width, Visibility};
use crate::components::view::*;
use crate::render_port::RenderPort;
use crate::systems::input::InputExt;
use crate::termx::IsTermx;
use crate::text_renderer::render_text;
use int_vec_2d::{Vector, Point, Rect, Thickness, HAlign, VAlign};

import! { pub render:
    use [obj basic_oop::obj];
    use crate::termx::Termx;
    use ooecs::{Entity, World};
    use termx_screen_base::Screen;
}

#[class_unsafe(inherits_Obj)]
pub struct Render {
    termx: rc::Weak<dyn IsTermx>,
    cursor: Cell<Option<Point>>,
    invalidated_rect: Cell<Rect>,
    screen_rect: Cell<Rect>,
    root: Cell<Option<Entity<Termx>>>,
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
    set_visual_offset: fn(entity: Entity<Termx>, world: &mut World<Termx>, value: Vector),
    #[non_virt]
    set_shadow: fn(entity: Entity<Termx>, world: &mut World<Termx>, value: Thickness),
    #[non_virt]
    root: fn() -> Option<Entity<Termx>>,
    #[non_virt]
    set_root: fn(root: Option<Entity<Termx>>, world: &mut World<Termx>),
    #[non_virt]
    in_tree: fn(entity: Entity<Termx>, world: &World<Termx>) -> bool,
    #[non_virt]
    point_from_screen: fn(entity: Entity<Termx>, world: &World<Termx>, point: Point) -> Point,
    #[non_virt]
    hit_test_input_element: fn(point: Point, world: &World<Termx>) -> Option<Entity<Termx>>,
    #[non_virt]
    perform: fn(world: &World<Termx>, screen: &mut dyn Screen) -> Option<Point>,
}

fn render_static_text(
    this: &Rc<dyn IsRender>,
    entity: Entity<Termx>,
    world: &World<Termx>,
    rp: &mut RenderPort,
    inner_bounds: Rect,
) {
    let render = this.render();
    let termx = render.termx.upgrade().unwrap();
    let c = termx.termx().components();
    let static_text = entity.get(c.static_text, world).unwrap();
    rp.fill_bg(static_text.color());
    let text_bounds = render_text(
        |p, s| rp.text(p, static_text.color(), s),
        inner_bounds,
        static_text.text_align().into(),
        static_text.text_wrapping(),
        static_text.text(),
    );
    if (text_bounds.w() as u16) > (inner_bounds.w() as u16)
        || (text_bounds.h() as u16) > (inner_bounds.h() as u16)
    {
        rp.text(inner_bounds.br_inner(), static_text.color(), "►");
    }
}

fn render_background(
    this: &Rc<dyn IsRender>,
    entity: Entity<Termx>,
    world: &World<Termx>,
    rp: &mut RenderPort,
    _inner_bounds: Rect,
) {
    let render = this.render();
    let termx = render.termx.upgrade().unwrap();
    let c = termx.termx().components();
    let background = entity.get(c.background, world).unwrap();
    rp.fill(|rp, p| rp.text(p, background.color(), background.pattern().as_str()));
}

fn render_m_button(
    this: &Rc<dyn IsRender>,
    entity: Entity<Termx>,
    world: &World<Termx>,
    rp: &mut RenderPort,
    inner_bounds: Rect,
) {
    let render = this.render();
    let termx = render.termx.upgrade().unwrap();
    let termx = termx.termx();
    let c = termx.components();
    let m_button = entity.get(c.m_button, world).unwrap();
    let button = entity.get(c.button, world).unwrap();
    let is_enabled = entity.get(c.focus_scope, world).unwrap().is_enabled();
    let is_focused = entity.get(c.input_element, world).unwrap().is_focused;

    let (color, color_hotkey) = if !is_enabled {
        (m_button.color_disabled(), m_button.color_disabled())
    } else if button.is_pressed() {
        (m_button.color_pressed(), m_button.color_pressed())
    } else if is_focused {
        (m_button.color_focused(), m_button.color_focused_hotkey())
    } else {
        (m_button.color(), m_button.color_hotkey())
    };

    let text_bounds = Thickness::new(2, 0, 2, 0).shrink_rect(inner_bounds);
    let text_align = Thickness::align(
        Vector { x: min(label_width(button.text()), text_bounds.w()), y: min(1, text_bounds.h()) },
        text_bounds.size,
        HAlign::Center,
        VAlign::Center
    );
    let text_bounds = text_align.shrink_rect(text_bounds);
    rp.fill_bg(color);
    rp.label_in_rect(text_bounds, color, color_hotkey, button.text());
    rp.text(inner_bounds.tl, color, "[");
    rp.text(inner_bounds.br_inner(), color, "]");
}

fn render_t_button(
    this: &Rc<dyn IsRender>,
    entity: Entity<Termx>,
    world: &World<Termx>,
    rp: &mut RenderPort,
    inner_bounds: Rect,
) {
    let render = this.render();
    let termx = render.termx.upgrade().unwrap();
    let termx = termx.termx();
    let c = termx.components();
    let t_button = entity.get(c.t_button, world).unwrap();
    let button = entity.get(c.button, world).unwrap();
    let is_enabled = entity.get(c.focus_scope, world).unwrap().is_enabled();
    let is_focused = entity.get(c.input_element, world).unwrap().is_focused;

    let (color, color_hotkey) = if !is_enabled {
        (t_button.color_disabled(), t_button.color_disabled())
    } else if is_focused {
        (t_button.color_focused(), t_button.color_focused_hotkey())
    } else {
        (t_button.color(), t_button.color_hotkey())
    };

    if button.is_pressed() {
        let text_bounds = Thickness::new(1, 0, 1, 0).shrink_rect(inner_bounds);
        let text_align = Thickness::align(
            Vector { x: min(label_width(button.text()), text_bounds.w()), y: min(1, text_bounds.h()) },
            text_bounds.size,
            HAlign::Center,
            VAlign::Center
        );
        let text_bounds = text_align.shrink_rect(text_bounds);
        rp.fill_bg(color);
        rp.label_in_rect(text_bounds, color, color_hotkey, button.text());
    } else {
        let bg_bounds = Thickness::new(0, 0, 1, 1).shrink_rect(inner_bounds);
        let text_bounds = Thickness::new(1, 0, 1, 0).shrink_rect(bg_bounds);
        let text_align = Thickness::align(
            Vector { x: min(label_width(button.text()), text_bounds.w()), y: min(1, text_bounds.h()) },
            text_bounds.size,
            HAlign::Center,
            VAlign::Center
        );
        let text_bounds = text_align.shrink_rect(text_bounds);
        let bottom_shadow_bounds = Thickness::new(1, 0, 0, 0).shrink_rect(inner_bounds.b_line());
        let right_shadow_bounds = Thickness::new(0, 1, 0, 1).shrink_rect(inner_bounds.r_line());
        rp.fill_rect(bg_bounds, |rp, p| rp.text(p, color, " "));
        rp.label_in_rect(text_bounds, color, color_hotkey, button.text());
        rp.fill_rect(bottom_shadow_bounds, |rp, p| rp.half_shadow(p, "▀"));
        rp.fill_rect(right_shadow_bounds, |rp, p| rp.half_shadow(p, "█"));
        rp.half_shadow(inner_bounds.tr_inner(), "▄");
    }
}

fn render_border(
    this: &Rc<dyn IsRender>,
    entity: Entity<Termx>,
    world: &World<Termx>,
    rp: &mut RenderPort,
    inner_bounds: Rect,
) {
    let render = this.render();
    let termx = render.termx.upgrade().unwrap();
    let c = termx.termx().components();
    let border = entity.get(c.border, world).unwrap();
    rp.fill_bg(border.color());
    rp.h_line(inner_bounds.tl, inner_bounds.w(), border.double(), border.color());
    rp.h_line(inner_bounds.bl_inner(), inner_bounds.w(), border.double(), border.color());
    rp.v_line(inner_bounds.tl, inner_bounds.h(), border.double(), border.color());
    rp.v_line(inner_bounds.tr_inner(), inner_bounds.h(), border.double(), border.color());
    rp.tl_edge(inner_bounds.tl, border.double(), border.color());
    rp.tr_edge(inner_bounds.tr_inner(), border.double(), border.color());
    rp.br_edge(inner_bounds.br_inner(), border.double(), border.color());
    rp.bl_edge(inner_bounds.bl_inner(), border.double(), border.color());
}

impl Render {
    pub fn new(termx: &Rc<dyn IsTermx>) -> Rc<dyn IsRender> {
        Rc::new(unsafe { Self::new_raw(termx, RENDER_VTABLE.as_ptr()) })
    }

    pub unsafe fn new_raw(termx: &Rc<dyn IsTermx>, vtable: Vtable) -> Self {
        Render {
            obj: unsafe { Obj::new_raw(vtable) },
            termx: Rc::downgrade(termx),
            cursor: Cell::new(None),
            invalidated_rect: Cell::new(Rect { tl: Point { x: 0, y: 0 }, size: Vector::null() }),
            screen_rect: Cell::new(Rect { tl: Point { x: 0, y: 0 }, size: Vector::null() }),
            root: Cell::new(None),
        }
    }

    pub fn visual_children_count_impl(
        this: &Rc<dyn IsRender>,
        entity: Entity<Termx>,
        world: &World<Termx>,
    ) -> usize {
        let render = this.render();
        let termx = render.termx.upgrade().unwrap();
        let termx = termx.termx();
        let c = termx.components();
        match entity.get(c.view, world).unwrap().tree() {
            TREE_DECORATOR => {
                let decorator = entity.get(c.decorator, world).unwrap();
                if decorator.child().is_some() { 1 } else { 0 }
            },
            TREE_PANEL => {
                let panel = entity.get(c.panel, world).unwrap();
                panel.children().len()
            },
            TREE_CONTENT_PRESENTER => {
                let content_presenter = entity.get(c.content_presenter, world).unwrap();
                if content_presenter.actual_child.is_some() { 1 } else { 0 }
            },
            TREE_CONTROL => {
                let control = entity.get(c.control, world).unwrap();
                if control.visual_tree.is_some() { 1 } else { 0 }
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
        let termx = render.termx.upgrade().unwrap();
        let termx = termx.termx();
        let c = termx.components();
        match entity.get(c.view, world).unwrap().tree() {
            TREE_DECORATOR => {
                assert_eq!(index, 0);
                let decorator = entity.get(c.decorator, world).unwrap();
                decorator.child().unwrap()
            },
            TREE_PANEL => {
                let panel = entity.get(c.panel, world).unwrap();
                panel.children()[index]
            },
            TREE_CONTENT_PRESENTER => {
                assert_eq!(index, 0);
                let content_presenter = entity.get(c.content_presenter, world).unwrap();
                content_presenter.actual_child.unwrap()
            },
            TREE_CONTROL => {
                assert_eq!(index, 0);
                let control = entity.get(c.control, world).unwrap();
                control.visual_tree.unwrap()
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
        let termx = render.termx.upgrade().unwrap();
        let c = termx.termx().components();
        match entity.get(c.view, world).unwrap().render() {
            RENDER_BACKGROUND => render_background(this, entity, world, rp, inner_bounds),
            RENDER_T_BUTTON => render_t_button(this, entity, world, rp, inner_bounds),
            RENDER_STATIC_TEXT => render_static_text(this, entity, world, rp, inner_bounds),
            RENDER_BORDER => render_border(this, entity, world, rp, inner_bounds),
            RENDER_M_BUTTON => render_m_button(this, entity, world, rp, inner_bounds),
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
        let termx = render.termx.upgrade().unwrap();
        let c = termx.termx().components();
        this.invalidate_render(entity, world);
        let children_count = this.visual_children_count(entity, world);
        for i in 0 .. children_count {
            let child = this.visual_child(entity, world, i);
            let changed = if let Some(focus_scope) = child.get_mut(c.focus_scope, world) {
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
        let termx = render.termx.upgrade().unwrap();
        let c = termx.termx().components();
        assert_ne!(Some(child), render.root.get());
        child.get_mut(c.view, world).unwrap().visual_parent = Some(parent);
        this.invalidate_render(child, world);
        let parent_is_enabled = if let Some(parent_focus_scope) = parent.get(c.focus_scope, world) {
            parent_focus_scope.is_enabled()
        } else {
            true
        };
        if !parent_is_enabled {
            let changed = if let Some(focus_scope) = child.get_mut(c.focus_scope, world) {
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
        let termx = render.termx.upgrade().unwrap();
        let c = termx.termx().components();
        let view = child.get_mut(c.view, world).unwrap();
        assert_eq!(view.visual_parent, Some(parent));
        view.visual_parent = None;
        let parent_is_enabled = if let Some(parent_focus_scope) = parent.get(c.focus_scope, world) {
            parent_focus_scope.is_enabled()
        } else {
            true
        };
        if !parent_is_enabled {
            let changed = if let Some(focus_scope) = child.get_mut(c.focus_scope, world) {
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

    pub fn set_visual_offset_impl(
        this: &Rc<dyn IsRender>,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        value: Vector,
    ) {
        let render = this.render();
        let termx = render.termx.upgrade().unwrap();
        let c = termx.termx().components();
        this.invalidate_render(entity, world);
        entity.get_mut(c.view, world).unwrap().visual_offset = value;
        this.invalidate_render(entity, world);
    }

    pub fn set_shadow_impl(
        this: &Rc<dyn IsRender>,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        value: Thickness,
    ) {
        let render = this.render();
        let termx = render.termx.upgrade().unwrap();
        let c = termx.termx().components();
        this.invalidate_render(entity, world);
        let real_render_bounds = entity.get(c.view, world).unwrap().real_render_bounds;
        let view = entity.get_mut(c.view, world).unwrap();
        view.shadow = value;
        if view.visibility() == Visibility::Collapsed { return; }
        view.real_render_bounds_with_shadow = value.expand_rect(real_render_bounds);
        this.invalidate_render(entity, world);
    }

    pub fn in_tree_impl(this: &Rc<dyn IsRender>, mut entity: Entity<Termx>, world: &World<Termx>) -> bool {
        let render = this.render();
        let root = render.root.get().unwrap();
        let termx = render.termx.upgrade().unwrap();
        let c = termx.termx().components();
        loop {
            if entity == root { return true; }
            let Some(parent) = entity.get(c.view, world).unwrap().visual_parent else { break; };
            entity = parent;
        }
        false
    }

    pub fn hit_test_input_element_impl(
        this: &Rc<dyn IsRender>,
        point: Point,
        world: &World<Termx>,
    ) -> Option<Entity<Termx>> {
        let render = this.render();
        let termx = render.termx.upgrade().unwrap();
        let c = termx.termx().components();
        let mut entity = render.root.get().unwrap();
        let view = entity.get(c.view, world).unwrap();
        if view.visibility() != Visibility::Visible { return None; }
        if !entity.get(c.focus_scope, world).map_or(true, |x| x.is_enabled()) { return None; }
        let bounds = view.real_render_bounds.offset(view.visual_offset);
        if !bounds.contains(point) { return None; }
        let mut offset = Vector { x: bounds.l(), y: bounds.t() };
        'o: loop {
            let children_count = this.visual_children_count(entity, world);
            for i in (0 .. children_count).rev() {
                let child = this.visual_child(entity, world, i);
                let view = child.get(c.view, world).unwrap();
                if view.visibility() != Visibility::Visible { continue; }
                if !child.get(c.focus_scope, world).map_or(true, |x| x.is_enabled()) { continue; }
                let bounds = view.real_render_bounds.offset(view.visual_offset).offset(offset);
                if !bounds.contains(point) { continue; }
                offset = Vector { x: bounds.l(), y: bounds.t() };
                entity = child;
                continue 'o;
            }
            break;
        }
        while !entity.get(c.input_element, world).map_or(false, |x| x.focusable) {
            let Some(parent) = entity.get(c.view, world).unwrap().visual_parent else { return None; };
            entity = parent;
        }
        Some(entity)
    }

    pub fn point_from_screen_impl(
        this: &Rc<dyn IsRender>,
        mut entity: Entity<Termx>,
        world: &World<Termx>,
        point: Point
    ) -> Point {
        let render = this.render();
        let root = render.root.get().unwrap();
        let termx = render.termx.upgrade().unwrap();
        let c = termx.termx().components();
        let mut offset = Vector::null();
        let mut in_tree = false;
        loop {
            in_tree = in_tree || entity == root;
            let view = entity.get(c.view, world).unwrap();
            let tl = view.real_render_bounds.tl;
            offset += Vector { x: tl.x, y: tl.y };
            offset += view.visual_offset;
            let Some(parent) = view.visual_parent else { break; };
            entity = parent;
        }
        assert!(in_tree);
        point.offset(offset)
    }

    pub fn invalidate_render_impl(this: &Rc<dyn IsRender>, entity: Entity<Termx>, world: &World<Termx>) {
        let render = this.render();
        let Some(root) = render.root.get() else { return; };
        let termx = render.termx.upgrade().unwrap();
        let c = termx.termx().components();
        let view = entity.get(c.view, world).unwrap();
        let local_rect = view.real_render_bounds_with_shadow;
        let mut global_offset = view.visual_offset;
        let mut cur = view.visual_parent;
        let mut in_tree = entity == root;
        while let Some(parent) = cur {
            in_tree = in_tree || parent == root;
            let parent_view = parent.get(c.view, world).unwrap();
            let tl = parent_view.real_render_bounds.tl;
            global_offset += Vector { x: tl.x, y: tl.y };
            global_offset += parent_view.visual_offset;
            cur = parent_view.visual_parent;
        }
        if !in_tree { return; }
        let global_rect = local_rect.offset(global_offset);
        let union = render.invalidated_rect.get().union_intersect(global_rect, render.screen_rect.get());
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
        let termx = render.termx.upgrade().unwrap();
        let c = termx.termx().components();
        let view = entity.get(c.view, world).unwrap();
        if view.visibility() != Visibility::Visible { return; }
        let render_bounds = Rect { tl: Point { x: 0, y: 0 }, size: view.real_render_bounds_with_shadow.size };
        this.render_view(entity, world, rp, render_bounds);
        let base_offset = rp.offset;
        let base_bounds = Rect {
            tl: Point { x: base_offset.x, y: base_offset.y },
            size: view.real_render_bounds.size,
        };
        for i in 0 .. this.visual_children_count(entity, world) {
            let child = this.visual_child(entity, world, i);
            let child_view = child.get(c.view, world).unwrap();
            let visual_offset = child_view.visual_offset;
            let bounds = child_view.real_render_bounds.offset(base_offset).offset(visual_offset);
            let bounds_with_shadow
                = child_view.real_render_bounds_with_shadow.offset(base_offset).offset(visual_offset);
            rp.bounds = bounds_with_shadow.intersect(base_bounds);
            rp.offset = Vector { x: bounds.l(), y: bounds.t() };
            Self::render_entity(this, child, world, rp);
        }
    }

    pub fn root_impl(this: &Rc<dyn IsRender>) -> Option<Entity<Termx>> {
        this.render().root.get()
    }

    pub fn set_root_impl(this: &Rc<dyn IsRender>, root: Option<Entity<Termx>>, world: &mut World<Termx>) {
        let render = this.render();
        if let Some(root) = root {
            let termx = render.termx.upgrade().unwrap();
            let c = termx.termx().components();
            assert!(root.get(c.view, world).unwrap().visual_parent.is_none());
            let input = &termx.termx().systems().input;
            if let Some(mut focused) = input.focused() {
                loop {
                    if focused == root { break; }
                    focused = focused.get(c.view, world).unwrap().visual_parent.unwrap();
                }
            }
        }
        render.root.set(root);
        if let Some(root) = root {
            this.invalidate_render(root, world);
        } else {
            let render = this.render();
            let termx = render.termx.upgrade().unwrap();
            let input = &termx.termx().systems().input;
            input.focus(None, world);
        }
    }

    pub fn perform_impl(
        this: &Rc<dyn IsRender>,
        world: &World<Termx>,
        screen: &mut dyn Screen,
    ) -> Option<Point> {
        let render = this.render();
        let termx = render.termx.upgrade().unwrap();
        let c = termx.termx().components();
        let root = render.root.get().unwrap();
        let cursor = render.cursor.get();
        let mut invalidated_rect = render.invalidated_rect.replace(
            Rect { tl: Point { x: 0, y: 0 }, size: Vector::null() }
        );
        let screen_size = screen.size();
        if screen_size != render.screen_rect.get().size {
            render.screen_rect.set(Rect { tl: Point { x: 0, y: 0 }, size: screen_size });
            invalidated_rect = Rect { tl: Point { x: 0, y: 0 }, size: screen_size };
        }
        let root_view = root.get(c.view, world).unwrap();
        let root_bounds = root_view.real_render_bounds;
        let root_bounds_with_shadow = root_view.real_render_bounds_with_shadow;
        let mut rp = RenderPort {
            screen,
            invalidated_rect,
            bounds: root_bounds_with_shadow.intersect(Rect { tl: Point { x: 0, y: 0 }, size: screen_size }),
            offset: Vector { x: root_bounds.l(), y: root_bounds.t() },
            cursor,
        };
        Self::render_entity(this, root, world, &mut rp);
        render.cursor.set(rp.cursor);
        rp.cursor
    }
}
