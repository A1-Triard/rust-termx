use alloc::rc::{self, Rc};
use basic_oop::{Vtable, import, class_unsafe};
use core::cmp::min;
use crate::base::{ViewHAlign, ViewVAlign};
use crate::components::background::Background;
use crate::components::decorator::Decorator;
use crate::components::layout_view::*;
use crate::components::panel::Panel;
use crate::components::stack_panel::StackPanel;
use crate::components::view::View;
use crate::systems::render::RenderExt;
use crate::termx::IsTermx;
use int_vec_2d::{HAlign, VAlign, Thickness, Point};
use ooecs::Component;

import! { pub layout:
    use [obj basic_oop::obj];
    use crate::termx::Termx;
    use int_vec_2d::{Vector, Rect};
    use ooecs::{Entity, World};
}

#[class_unsafe(inherits_Obj)]
pub struct Layout {
    pub termx: rc::Weak<dyn IsTermx>,
    pub layout_view: Component<LayoutView, Termx>,
    pub view: Component<View, Termx>,
    pub background: Component<Background, Termx>,
    pub decorator: Component<Decorator, Termx>,
    pub panel: Component<Panel, Termx>,
    pub stack_panel: Component<StackPanel, Termx>,
    #[virt]
    measure_override: fn(
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        w: Option<i16>,
        h: Option<i16>
    ) -> Vector,
    #[virt]
    arrange_override: fn(entity: Entity<Termx>, world: &mut World<Termx>, inner_bounds: Rect) -> Vector,
    #[non_virt]
    invalidate_measure: fn(entity: Entity<Termx>, world: &mut World<Termx>),
    #[non_virt]
    invalidate_arrange: fn(entity: Entity<Termx>, world: &mut World<Termx>),
    #[non_virt]
    measure: fn(entity: Entity<Termx>, world: &mut World<Termx>, w: Option<i16>, h: Option<i16>),
    #[non_virt]
    arrange: fn(entity: Entity<Termx>, world: &mut World<Termx>, bounds: Rect),
    #[non_virt]
    perform: fn(root: Entity<Termx>, world: &mut World<Termx>, size: Vector),
}

fn measure_stack_panel(
    this: &Rc<dyn IsLayout>,
    entity: Entity<Termx>,
    world: &mut World<Termx>,
    w: Option<i16>,
    h: Option<i16>,
) -> Vector {
    let layout = this.layout();
    let children = entity.get(layout.panel, world).unwrap().children().to_vec();
    if entity.get(layout.stack_panel, world).unwrap().vertical() {
        let mut size = Vector::null();
        for child in children {
            this.measure(child, world, w, None);
            let desired_size = child.get(layout.layout_view, world).unwrap().desired_size();
            size += Vector { x: 0, y: desired_size.y };
            size = size.max(Vector { x: desired_size.x, y: 0 });
        }
        size
    } else {
        let mut size = Vector::null();
        for child in children {
            this.measure(child, world, None, h);
            let desired_size = child.get(layout.layout_view, world).unwrap().desired_size();
            size += Vector { x: desired_size.x, y: 0 };
            size = size.max(Vector { x: 0, y: desired_size.y });
        }
        size
    }
}

fn arrange_stack_panel(
    this: &Rc<dyn IsLayout>,
    entity: Entity<Termx>,
    world: &mut World<Termx>,
    inner_bounds: Rect,
) -> Vector {
    let layout = this.layout();
    let children = entity.get(layout.panel, world).unwrap().children().to_vec();
    if entity.get(layout.stack_panel, world).unwrap().vertical() {
        let mut pos = inner_bounds.tl;
        let mut size = Vector::null();
        for child in children {
            let desired_size = child.get(layout.layout_view, world).unwrap().desired_size();
            let child_size = Vector { x: inner_bounds.w(), y: desired_size.y };
            this.arrange(child, world, Rect { tl: pos, size: child_size });
            pos = pos.offset(Vector { x: 0, y: child_size.y });
            size += Vector { x: 0, y: child_size.y };
            size = size.max(Vector { x: child_size.x, y: 0 });
        }
        size
    } else {
        let mut pos = inner_bounds.tl;
        let mut size = Vector::null();
        for child in children {
            let desired_size = child.get(layout.layout_view, world).unwrap().desired_size();
            let child_size = Vector { x: desired_size.x, y: inner_bounds.h() };
            this.arrange(child, world, Rect { tl: pos, size: child_size });
            pos = pos.offset(Vector { x: child_size.x, y: 0 });
            size += Vector { x: child_size.x, y: 0 };
            size = size.max(Vector { x: 0, y: child_size.y });
        }
        size
    }
}

impl Layout {
    pub fn new(
        termx: &Rc<dyn IsTermx>,
        layout_view: Component<LayoutView, Termx>,
        view: Component<View, Termx>,
        background: Component<Background, Termx>,
        decorator: Component<Decorator, Termx>,
        panel: Component<Panel, Termx>,
        stack_panel: Component<StackPanel, Termx>,
    ) -> Rc<dyn IsLayout> {
        Rc::new(unsafe { Self::new_raw(
            termx,
            layout_view,
            view,
            background,
            decorator,
            panel,
            stack_panel,
            LAYOUT_VTABLE.as_ptr(),
        ) })
    }

    pub unsafe fn new_raw(
        termx: &Rc<dyn IsTermx>,
        layout_view: Component<LayoutView, Termx>,
        view: Component<View, Termx>,
        background: Component<Background, Termx>,
        decorator: Component<Decorator, Termx>,
        panel: Component<Panel, Termx>,
        stack_panel: Component<StackPanel, Termx>,
        vtable: Vtable,
    ) -> Self {
        Layout {
            obj: unsafe { Obj::new_raw(vtable) },
            termx: Rc::downgrade(termx),
            layout_view,
            view,
            background,
            decorator,
            panel,
            stack_panel,
        }
    }

    pub fn measure_override_impl(
        this: &Rc<dyn IsLayout>,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        w: Option<i16>,
        h: Option<i16>,
    ) -> Vector {
        let layout = this.layout();
        match entity.get(layout.layout_view, world).unwrap().layout() {
            LAYOUT_BACKGROUND => {
                let child = entity.get(layout.decorator, world).unwrap().child();
                if let Some(child) = child {
                    this.measure(child, world, w, h);
                    child.get(layout.layout_view, world).unwrap().desired_size
                } else {
                    Vector::null()
                }
            },
            LAYOUT_STACK_PANEL => measure_stack_panel(this, entity, world, w, h),
            _ => Vector::null()
        }
    }

    pub fn arrange_override_impl(
        this: &Rc<dyn IsLayout>,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        inner_bounds: Rect,
    ) -> Vector {
        let layout = this.layout();
        match entity.get(layout.layout_view, world).unwrap().layout() {
            LAYOUT_BACKGROUND => {
                let child = entity.get(layout.decorator, world).unwrap().child();
                if let Some(child) = child {
                    this.arrange(child, world, inner_bounds);
                    child.get(layout.layout_view, world).unwrap().render_bounds.size
                } else {
                    inner_bounds.size
                }
            },
            LAYOUT_STACK_PANEL => arrange_stack_panel(this, entity, world, inner_bounds),
            _ => Vector::null()
        }
    }

    pub fn invalidate_measure_impl(
        this: &Rc<dyn IsLayout>,
        mut entity: Entity<Termx>,
        world: &mut World<Termx>,
    ) {
        let layout = this.layout();
        loop {
            {
                let layout_view = entity.get_mut(layout.layout_view, world).unwrap();
                layout_view.measure_size = None;
                layout_view.arrange_size = None;
            }
            let view = entity.get(layout.view, world).unwrap();
            let Some(parent) = view.visual_parent else { break; };
            entity = parent;
        }
    }

    pub fn invalidate_arrange_impl(
        this: &Rc<dyn IsLayout>,
        mut entity: Entity<Termx>,
        world: &mut World<Termx>,
    ) {
        let layout = this.layout();
        loop {
            {
                let layout_view = entity.get_mut(layout.layout_view, world).unwrap();
                layout_view.arrange_size = None;
            }
            let view = entity.get(layout.view, world).unwrap();
            let Some(parent) = view.visual_parent else { break; };
            entity = parent;
        }
    }

    pub fn measure_impl(
        this: &Rc<dyn IsLayout>,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        w: Option<i16>,
        h: Option<i16>,
    ) {
        let (a_w, a_h, max_size, min_size) = {
            let layout = this.layout();
            let component = entity.get(layout.layout_view, world).unwrap();
            if component.measure_size == Some((w, h)) { return; }
            let max_width = component.width().or(component.max_width());
            let max_height = component.height().or(component.max_height());
            let max_size = Vector { x: max_width.unwrap_or(-1), y: max_height.unwrap_or(-1) };
            let min_size = Vector {
                x: component.width().unwrap_or(component.min_size().x),
                y: component.height().unwrap_or(component.min_size().y),
            };
            let g_w = if component.h_align() != ViewHAlign::Stretch { None } else { w };
            let g_h = if component.v_align() != ViewVAlign::Stretch { None } else { h };
            let g_w = g_w.or(max_width);
            let g_h = g_h.or(max_height);
            let a = Vector { x: g_w.unwrap_or(0), y: g_h.unwrap_or(0) };
            let a = component.margin().shrink_rect_size(a);
            let a = a.min(max_size).max(min_size);
            (g_w.map(|_| a.x), g_h.map(|_| a.y), max_size, min_size)
        };
        let desired_size = this.measure_override(entity, world, a_w, a_h);
        let layout = this.layout();
        let component = entity.get_mut(layout.layout_view, world).unwrap();
        let desired_size = desired_size.min(max_size).max(min_size);
        let desired_size = component.margin().expand_rect_size(desired_size);
        let desired_size = Vector {
            x: w.map_or(desired_size.x, |w| min(w as u16, desired_size.x as u16) as i16),
            y: h.map_or(desired_size.y, |h| min(h as u16, desired_size.y as u16) as i16),
        };
        component.measure_size = Some((w, h));
        component.desired_size = desired_size;
    }

    pub fn arrange_impl(
        this: &Rc<dyn IsLayout>,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        bounds: Rect,
    ) {
        let (a_size, max_size, min_size) = {
            let layout = this.layout();
            let component = entity.get(layout.layout_view, world).unwrap();
            let max_width = component.width().or(component.max_width());
            let max_height = component.height().or(component.max_height());
            let max_size = Vector { x: max_width.unwrap_or(-1), y: max_height.unwrap_or(-1) };
            let min_size = Vector {
                x: component.width().unwrap_or(component.min_size().x),
                y: component.height().unwrap_or(component.min_size().y),
            };
            if Some(bounds.size) == component.arrange_size {
                (None, max_size, min_size)
            } else {
                let a_size = component.margin().shrink_rect_size(bounds.size).min(max_size).max(min_size);
                let d_size = component.margin().shrink_rect_size(component.desired_size);
                (Some((a_size, component.h_align(), component.v_align(), d_size)), max_size, min_size)
            }
        };
        let render_size = if let Some((a_size, h_align, v_align, desired_size)) = a_size {
            let a_size = Vector {
                x: if h_align == ViewHAlign::Stretch { a_size.x } else { desired_size.x },
                y: if v_align == ViewVAlign::Stretch { a_size.y } else { desired_size.y }
            };
            let render_size = this.arrange_override(
                entity, world, Rect { tl: Point { x: 0, y: 0 }, size: a_size }
            );
            let layout = this.layout();
            let component = entity.get(layout.layout_view, world).unwrap();
            component.margin().expand_rect_size(render_size.min(max_size).max(min_size)).min(bounds.size)
        } else {
            let layout = this.layout();
            let component = entity.get(layout.layout_view, world).unwrap();
            component.render_bounds.size
        };
        let (render_bounds, real_render_bounds) = {
            let layout = this.layout();
            let component = entity.get(layout.layout_view, world).unwrap();
            let h_align = <Option<HAlign>>::from(component.h_align()).unwrap_or(HAlign::Left);
            let v_align = <Option<VAlign>>::from(component.v_align()).unwrap_or(VAlign::Top);
            let align = Thickness::align(render_size, bounds.size, h_align, v_align);
            let render_bounds = align.shrink_rect(bounds);
            let real_render_bounds = component.margin().shrink_rect(render_bounds);
            (render_bounds, real_render_bounds)
        };
        let layout = this.layout();
        if real_render_bounds == entity.get(layout.view, world).unwrap().real_render_bounds {
            let component = entity.get_mut(layout.layout_view, world).unwrap();
            component.arrange_size = Some(bounds.size);
            component.render_bounds = render_bounds;
            return;
        }
        let termx = layout.termx.upgrade().unwrap();
        termx.termx().systems().render.invalidate_render(entity, world);
        {
            let component = entity.get_mut(layout.layout_view, world).unwrap();
            component.arrange_size = Some(bounds.size);
            component.render_bounds = render_bounds;
        }
        entity.get_mut(layout.view, world).unwrap().real_render_bounds = real_render_bounds;
        termx.termx().systems().render.invalidate_render(entity, world);
    }

    pub fn perform_impl(this: &Rc<dyn IsLayout>, root: Entity<Termx>, world: &mut World<Termx>, size: Vector) {
        this.measure(root, world, Some(size.x), Some(size.y));
        this.arrange(root, world, Rect { tl: Point { x: 0, y: 0 }, size });
    }
}
