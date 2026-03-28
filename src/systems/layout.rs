use basic_oop::{Vtable, import, class_unsafe};
use crate::components::decorator::Decorator;
use crate::components::view_layout::*;
use crate::components::view::View;
use crate::systems::render::RenderExt;
use crate::termx::IsTermx;
use int_vec_2d::{HAlign, VAlign, Thickness, Point};
use ooecs::Component;
use std::cmp::min;
use std::rc::{self, Rc};

import! { pub layout:
    use [obj basic_oop::obj];
    use int_vec_2d::{Vector, Rect};
    use ooecs::{Entity, World};
}

#[class_unsafe(inherits_Obj)]
pub struct Layout {
    pub termx: rc::Weak<dyn IsTermx>,
    pub view_layout: Component,
    pub view: Component,
    pub background: Component,
    pub decorator: Component,
    #[virt]
    measure_override: fn(entity: Entity, world: &mut World, w: Option<i16>, h: Option<i16>) -> Vector,
    #[virt]
    arrange_override: fn(entity: Entity, world: &mut World, inner_bounds: Rect) -> Vector,
    #[non_virt]
    invalidate_measure: fn(entity: Entity, world: &mut World),
    #[non_virt]
    invalidate_arrange: fn(entity: Entity, world: &mut World),
    #[non_virt]
    measure: fn(entity: Entity, world: &mut World, w: Option<i16>, h: Option<i16>),
    #[non_virt]
    arrange: fn(entity: Entity, world: &mut World, bounds: Rect),
    #[non_virt]
    perform: fn(root: Entity, world: &mut World, size: Vector),
}

impl Layout {
    pub fn new(
        termx: &Rc<dyn IsTermx>,
        view_layout: Component,
        view: Component,
        background: Component,
        decorator: Component,
    ) -> Rc<dyn IsLayout> {
        Rc::new(unsafe { Self::new_raw(
            termx,
            view_layout,
            view,
            background,
            decorator,
            LAYOUT_VTABLE.as_ptr(),
        ) })
    }

    pub unsafe fn new_raw(
        termx: &Rc<dyn IsTermx>,
        view_layout: Component,
        view: Component,
        background: Component,
        decorator: Component,
        vtable: Vtable,
    ) -> Self {
        Layout {
            obj: unsafe { Obj::new_raw(vtable) },
            termx: Rc::downgrade(termx),
            view_layout,
            view,
            background,
            decorator,
        }
    }

    pub fn measure_override_impl(
        this: &Rc<dyn IsLayout>,
        entity: Entity,
        world: &mut World,
        w: Option<i16>,
        h: Option<i16>
    ) -> Vector {
        let layout = this.layout();
        match entity.get::<ViewLayout>(layout.view_layout, world).unwrap().layout() {
            LAYOUT_BACKGROUND => {
                let child = entity.get::<Decorator>(layout.decorator, world).unwrap().child;
                if let Some(child) = child {
                    this.measure(child, world, w, h);
                    child.get::<ViewLayout>(layout.view_layout, world).unwrap().desired_size
                } else {
                    Vector::null()
                }
            },
            _ => Vector::null()
        }
    }

    pub fn arrange_override_impl(
        this: &Rc<dyn IsLayout>,
        entity: Entity,
        world: &mut World,
        inner_bounds: Rect,
    ) -> Vector {
        let layout = this.layout();
        match entity.get::<ViewLayout>(layout.view_layout, world).unwrap().layout() {
            LAYOUT_BACKGROUND => {
                let child = entity.get::<Decorator>(layout.decorator, world).unwrap().child;
                if let Some(child) = child {
                    this.arrange(child, world, inner_bounds);
                    child.get::<ViewLayout>(layout.view_layout, world).unwrap().render_bounds.size
                } else {
                    inner_bounds.size
                }
            },
            _ => Vector::null()
        }
    }

    pub fn invalidate_measure_impl(this: &Rc<dyn IsLayout>, mut entity: Entity, world: &mut World) {
        let layout = this.layout();
        loop {
            {
                let view_layout = entity.get_mut::<ViewLayout>(layout.view_layout, world).unwrap();
                view_layout.measure_size = None;
                view_layout.arrange_size = None;
            }
            let view = entity.get::<View>(layout.view, world).unwrap();
            let Some(parent) = view.visual_parent else { break; };
            entity = parent;
        }
    }

    pub fn invalidate_arrange_impl(this: &Rc<dyn IsLayout>, mut entity: Entity, world: &mut World) {
        let layout = this.layout();
        loop {
            {
                let view_layout = entity.get_mut::<ViewLayout>(layout.view_layout, world).unwrap();
                view_layout.arrange_size = None;
            }
            let view = entity.get::<View>(layout.view, world).unwrap();
            let Some(parent) = view.visual_parent else { break; };
            entity = parent;
        }
    }

    pub fn measure_impl(
        this: &Rc<dyn IsLayout>,
        entity: Entity,
        world: &mut World,
        w: Option<i16>,
        h: Option<i16>,
    ) {
        let (a_w, a_h, max_size, min_size) = {
            let layout = this.layout();
            let component = entity.get::<ViewLayout>(layout.view_layout, world).unwrap();
            if component.measure_size == Some((w, h)) { return; }
            let max_width = component.width().or(component.max_width());
            let max_height = component.height().or(component.max_height());
            let max_size = Vector { x: max_width.unwrap_or(-1), y: max_height.unwrap_or(-1) };
            let min_size = Vector {
                x: component.width().unwrap_or(component.min_size().x),
                y: component.height().unwrap_or(component.min_size().y),
            };
            let g_w = if component.h_align().is_some() { None } else { w };
            let g_h = if component.v_align().is_some() { None } else { h };
            let g_w = g_w.or(max_width);
            let g_h = g_h.or(max_height);
            let a = Vector { x: g_w.unwrap_or(0), y: g_h.unwrap_or(0) };
            let a = component.margin().shrink_rect_size(a);
            let a = a.min(max_size).max(min_size);
            (g_w.map(|_| a.x), g_h.map(|_| a.y), max_size, min_size)
        };
        let desired_size = this.measure_override(entity, world, a_w, a_h);
        let layout = this.layout();
        let component = entity.get_mut::<ViewLayout>(layout.view_layout, world).unwrap();
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
        entity: Entity,
        world: &mut World,
        bounds: Rect,
    ) {
        let (a_size, max_size, min_size) = {
            let layout = this.layout();
            let component = entity.get::<ViewLayout>(layout.view_layout, world).unwrap();
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
                x: if h_align.is_none() { a_size.x } else { desired_size.x },
                y: if v_align.is_none() { a_size.y } else { desired_size.y }
            };
            let render_size = this.arrange_override(
                entity, world, Rect { tl: Point { x: 0, y: 0 }, size: a_size }
            );
            let layout = this.layout();
            let component = entity.get::<ViewLayout>(layout.view_layout, world).unwrap();
            component.margin().expand_rect_size(render_size.min(max_size).max(min_size)).min(bounds.size)
        } else {
            let layout = this.layout();
            let component = entity.get::<ViewLayout>(layout.view_layout, world).unwrap();
            component.render_bounds.size
        };
        let (render_bounds, real_render_bounds) = {
            let layout = this.layout();
            let component = entity.get::<ViewLayout>(layout.view_layout, world).unwrap();
            let h_align = component.h_align().unwrap_or(HAlign::Left);
            let v_align = component.v_align().unwrap_or(VAlign::Top);
            let align = Thickness::align(render_size, bounds.size, h_align, v_align);
            let render_bounds = align.shrink_rect(bounds);
            let real_render_bounds = component.margin().shrink_rect(render_bounds);
            (render_bounds, real_render_bounds)
        };
        let layout = this.layout();
        if real_render_bounds == entity.get::<View>(layout.view, world).unwrap().real_render_bounds {
            let component = entity.get_mut::<ViewLayout>(layout.view_layout, world).unwrap();
            component.arrange_size = Some(bounds.size);
            component.render_bounds = render_bounds;
            return;
        }
        let termx = layout.termx.upgrade().unwrap();
        termx.termx().systems().render.invalidate_render(entity, world);
        {
            let component = entity.get_mut::<ViewLayout>(layout.view_layout, world).unwrap();
            component.arrange_size = Some(bounds.size);
            component.render_bounds = render_bounds;
        }
        entity.get_mut::<View>(layout.view, world).unwrap().real_render_bounds = real_render_bounds;
        termx.termx().systems().render.invalidate_render(entity, world);
    }

    pub fn perform_impl(this: &Rc<dyn IsLayout>, root: Entity, world: &mut World, size: Vector) {
        this.measure(root, world, Some(size.x), Some(size.y));
        this.arrange(root, world, Rect { tl: Point { x: 0, y: 0 }, size });
    }
}
