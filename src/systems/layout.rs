use alloc::rc::{self, Rc};
use basic_oop::{Vtable, import, class_unsafe};
use core::cmp::min;
use core::mem::replace;
use crate::base::{ViewHAlign, ViewVAlign, label_width, Visibility};
use crate::components::layout_view::*;
use crate::systems::render::RenderExt;
use crate::termx::IsTermx;
use crate::text_renderer::render_text;
use int_vec_2d::{HAlign, VAlign, Thickness, Point};
use iter_identify_first_last::IteratorIdentifyFirstLastExt;

import! { pub layout:
    use [obj basic_oop::obj];
    use crate::termx::Termx;
    use int_vec_2d::{Vector, Rect};
    use ooecs::{Entity, World};
}

#[class_unsafe(inherits_Obj)]
pub struct Layout {
    pub termx: rc::Weak<dyn IsTermx>,
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

fn measure_background(
    this: &Rc<dyn IsLayout>,
    entity: Entity<Termx>,
    world: &mut World<Termx>,
    w: Option<i16>,
    h: Option<i16>,
) -> Vector {
    let layout = this.layout();
    let termx = layout.termx.upgrade().unwrap();
    let c = termx.termx().components();
    let child = entity.get(c.decorator, world).unwrap().child();
    if let Some(child) = child {
        this.measure(child, world, w, h);
        child.get(c.layout_view, world).unwrap().desired_size
    } else {
        Vector::null()
    }
}

fn arrange_background(
    this: &Rc<dyn IsLayout>,
    entity: Entity<Termx>,
    world: &mut World<Termx>,
    inner_bounds: Rect,
) -> Vector {
    let layout = this.layout();
    let termx = layout.termx.upgrade().unwrap();
    let c = termx.termx().components();
    let child = entity.get(c.decorator, world).unwrap().child();
    if let Some(child) = child {
        this.arrange(child, world, inner_bounds);
        if entity.get(c.background, world).unwrap().fit_to_content() {
            child.get(c.layout_view, world).unwrap().render_bounds.size
        } else {
            inner_bounds.size
        }
    } else {
        if entity.get(c.background, world).unwrap().fit_to_content() {
            Vector::null()
        } else {
            inner_bounds.size
        }
    }
}

fn measure_stack_panel(
    this: &Rc<dyn IsLayout>,
    entity: Entity<Termx>,
    world: &mut World<Termx>,
    w: Option<i16>,
    h: Option<i16>,
) -> Vector {
    let layout = this.layout();
    let termx = layout.termx.upgrade().unwrap();
    let c = termx.termx().components();
    let children = entity.get(c.panel, world).unwrap().children().to_vec();
    if entity.get(c.stack_panel, world).unwrap().vertical() {
        let mut size = Vector::null();
        for child in children {
            this.measure(child, world, w, None);
            let desired_size = child.get(c.layout_view, world).unwrap().desired_size();
            size += Vector { x: 0, y: desired_size.y };
            size = size.max(Vector { x: desired_size.x, y: 0 });
        }
        size
    } else {
        let mut size = Vector::null();
        for child in children {
            this.measure(child, world, None, h);
            let desired_size = child.get(c.layout_view, world).unwrap().desired_size();
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
    let termx = layout.termx.upgrade().unwrap();
    let c = termx.termx().components();
    let children = entity.get(c.panel, world).unwrap().children().to_vec();
    if entity.get(c.stack_panel, world).unwrap().vertical() {
        let mut pos = inner_bounds.tl;
        let mut size = Vector::null();
        for child in children {
            let desired_size = child.get(c.layout_view, world).unwrap().desired_size();
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
            let desired_size = child.get(c.layout_view, world).unwrap().desired_size();
            let child_size = Vector { x: desired_size.x, y: inner_bounds.h() };
            this.arrange(child, world, Rect { tl: pos, size: child_size });
            pos = pos.offset(Vector { x: child_size.x, y: 0 });
            size += Vector { x: child_size.x, y: 0 };
            size = size.max(Vector { x: 0, y: child_size.y });
        }
        size
    }
}

fn measure_canvas(
    this: &Rc<dyn IsLayout>,
    entity: Entity<Termx>,
    world: &mut World<Termx>,
    w: Option<i16>,
    h: Option<i16>,
) -> Vector {
    let layout = this.layout();
    let termx = layout.termx.upgrade().unwrap();
    let c = termx.termx().components();
    let children = entity.get(c.panel, world).unwrap().children().to_vec();
    for child in children {
        this.measure(child, world, None, None);
    }
    Vector { x: w.unwrap_or(1), y: h.unwrap_or(1) }
}

fn arrange_canvas(
    this: &Rc<dyn IsLayout>,
    entity: Entity<Termx>,
    world: &mut World<Termx>,
    inner_bounds: Rect,
) -> Vector {
    let layout = this.layout();
    let termx = layout.termx.upgrade().unwrap();
    let c = termx.termx().components();
    let children = entity.get(c.panel, world).unwrap().children().to_vec();
    for child in children {
        let child_size = child.get(c.layout_view, world).unwrap().desired_size();
        let child_layout = child.get(c.view, world).unwrap().layout();
        let child_layout = child_layout.and_then(|x| x.get(c.canvas_layout, world));
        let tl = child_layout.map_or(Point { x: 0, y: 0 }, |x| x.tl());
        this.arrange(child, world, Rect { tl, size: child_size });
    }
    inner_bounds.size
}

fn measure_content_presenter(
    this: &Rc<dyn IsLayout>,
    entity: Entity<Termx>,
    world: &mut World<Termx>,
    w: Option<i16>,
    h: Option<i16>,
) -> Vector {
    let layout = this.layout();
    let termx = layout.termx.upgrade().unwrap();
    let c = termx.termx().components();
    let child = entity.get(c.content_presenter, world).unwrap().actual_child;
    if let Some(child) = child {
        this.measure(child, world, w, h);
        child.get(c.layout_view, world).unwrap().desired_size
    } else {
        Vector::null()
    }
}

fn arrange_content_presenter(
    this: &Rc<dyn IsLayout>,
    entity: Entity<Termx>,
    world: &mut World<Termx>,
    inner_bounds: Rect,
) -> Vector {
    let layout = this.layout();
    let termx = layout.termx.upgrade().unwrap();
    let c = termx.termx().components();
    let child = entity.get(c.content_presenter, world).unwrap().actual_child;
    if let Some(child) = child {
        this.arrange(child, world, inner_bounds);
        child.get(c.layout_view, world).unwrap().render_bounds.size
    } else {
        Vector::null()
    }
}

fn measure_control(
    this: &Rc<dyn IsLayout>,
    entity: Entity<Termx>,
    world: &mut World<Termx>,
    w: Option<i16>,
    h: Option<i16>,
) -> Vector {
    let layout = this.layout();
    let termx = layout.termx.upgrade().unwrap();
    let c = termx.termx().components();
    let child = entity.get(c.control, world).unwrap().visual_tree;
    if let Some(child) = child {
        this.measure(child, world, w, h);
        child.get(c.layout_view, world).unwrap().desired_size
    } else {
        Vector::null()
    }
}

fn arrange_control(
    this: &Rc<dyn IsLayout>,
    entity: Entity<Termx>,
    world: &mut World<Termx>,
    inner_bounds: Rect,
) -> Vector {
    let layout = this.layout();
    let termx = layout.termx.upgrade().unwrap();
    let c = termx.termx().components();
    let child = entity.get(c.control, world).unwrap().visual_tree;
    if let Some(child) = child {
        this.arrange(child, world, inner_bounds);
        child.get(c.layout_view, world).unwrap().render_bounds.size
    } else {
        Vector::null()
    }
}

fn measure_static_text(
    this: &Rc<dyn IsLayout>,
    entity: Entity<Termx>,
    world: &mut World<Termx>,
    w: Option<i16>,
    h: Option<i16>,
) -> Vector {
    let layout = this.layout();
    let termx = layout.termx.upgrade().unwrap();
    let c = termx.termx().components();
    let static_text = entity.get(c.static_text, world).unwrap();
    render_text(
        |_, _| { },
        Rect { tl: Point { x: 0, y: 0 }, size: Vector { x: w.unwrap_or(-1), y: h.unwrap_or(-1) } },
        static_text.text_align().into(),
        static_text.text_wrapping(),
        static_text.text(),
    ).size
}

fn arrange_static_text(
    this: &Rc<dyn IsLayout>,
    entity: Entity<Termx>,
    world: &mut World<Termx>,
    inner_bounds: Rect,
) -> Vector {
    let layout = this.layout();
    let termx = layout.termx.upgrade().unwrap();
    let c = termx.termx().components();
    let static_text = entity.get(c.static_text, world).unwrap();
    render_text(
        |_, _| { },
        inner_bounds,
        static_text.text_align().into(),
        static_text.text_wrapping(),
        static_text.text(),
    ).size
}

fn measure_button(
    this: &Rc<dyn IsLayout>,
    entity: Entity<Termx>,
    world: &mut World<Termx>,
    _w: Option<i16>,
    _h: Option<i16>,
) -> Vector {
    let layout = this.layout();
    let termx = layout.termx.upgrade().unwrap();
    let c = termx.termx().components();
    let text = &entity.get(c.button, world).unwrap().text();
    let text_width = label_width(text);
    Thickness::new(2, 0, 2, 0).expand_rect_size(Vector { x: text_width, y: 1 })
}

fn arrange_button(
    _this: &Rc<dyn IsLayout>,
    _entity: Entity<Termx>,
    _world: &mut World<Termx>,
    inner_bounds: Rect,
) -> Vector {
    inner_bounds.size
}

fn measure_t_button(
    this: &Rc<dyn IsLayout>,
    entity: Entity<Termx>,
    world: &mut World<Termx>,
    _w: Option<i16>,
    _h: Option<i16>,
) -> Vector {
    let layout = this.layout();
    let termx = layout.termx.upgrade().unwrap();
    let c = termx.termx().components();
    let text = &entity.get(c.button, world).unwrap().text();
    let text_width = label_width(text);
    Thickness::new(1, 0, 1, 0).expand_rect_size(Vector { x: text_width, y: 1 })
}

fn arrange_t_button(
    _this: &Rc<dyn IsLayout>,
    _entity: Entity<Termx>,
    _world: &mut World<Termx>,
    inner_bounds: Rect,
) -> Vector {
    inner_bounds.size
}

fn measure_border(
    this: &Rc<dyn IsLayout>,
    entity: Entity<Termx>,
    world: &mut World<Termx>,
    w: Option<i16>,
    h: Option<i16>,
) -> Vector {
    let layout = this.layout();
    let termx = layout.termx.upgrade().unwrap();
    let c = termx.termx().components();
    let child = entity.get(c.decorator, world).unwrap().child();
    if let Some(child) = child {
        let child_w = w.map(|x| Thickness::all(1).shrink_rect_size(Vector { x, y: 0 }).x);
        let child_h = h.map(|y| Thickness::all(1).shrink_rect_size(Vector { x: 0, y }).y);
        this.measure(child, world, child_w, child_h);
        let desired = child.get(c.layout_view, world).unwrap().desired_size;
        Thickness::all(1).expand_rect_size(desired)
    } else {
        Thickness::all(1).expand_rect_size(Vector::null())
    }
}

fn arrange_border(
    this: &Rc<dyn IsLayout>,
    entity: Entity<Termx>,
    world: &mut World<Termx>,
    inner_bounds: Rect,
) -> Vector {
    let layout = this.layout();
    let termx = layout.termx.upgrade().unwrap();
    let c = termx.termx().components();
    let child = entity.get(c.decorator, world).unwrap().child();
    if let Some(child) = child {
        let child_bounds = Thickness::all(1).shrink_rect(inner_bounds);
        this.arrange(child, world, child_bounds);
    }
    inner_bounds.size
}

fn measure_adorners_panel(
    this: &Rc<dyn IsLayout>,
    entity: Entity<Termx>,
    world: &mut World<Termx>,
    w: Option<i16>,
    h: Option<i16>,
) -> Vector {
    let layout = this.layout();
    let termx = layout.termx.upgrade().unwrap();
    let c = termx.termx().components();
    let children = entity.get(c.panel, world).unwrap().children().to_vec();
    let mut render_size = Vector::null();
    let mut desired_size = Vector::null();
    for (is_first, child) in children.into_iter().identify_first() {
        if is_first {
            render_size = child.get(c.layout_view, world).unwrap().render_bounds.size;
            this.measure(child, world, w, h);
            desired_size = child.get(c.layout_view, world).unwrap().desired_size;
        } else {
            this.measure(child, world, Some(render_size.x), Some(render_size.y));
        }
    }
    desired_size
}

fn arrange_adorners_panel(
    this: &Rc<dyn IsLayout>,
    entity: Entity<Termx>,
    world: &mut World<Termx>,
    inner_bounds: Rect,
) -> Vector {
    let layout = this.layout();
    let termx = layout.termx.upgrade().unwrap();
    let c = termx.termx().components();
    let children = entity.get(c.panel, world).unwrap().children().to_vec();
    let mut child_bounds = inner_bounds;
    for (is_first, child) in children.into_iter().identify_first() {
        if is_first {
            let prev_render_size = child.get(c.layout_view, world).unwrap().render_bounds.size;
            this.arrange(child, world, inner_bounds);
            child_bounds = child.get(c.layout_view, world).unwrap().render_bounds;
            if child_bounds.size != prev_render_size {
                this.invalidate_measure(entity, world);
                return Vector::null();
            }
        } else {
            this.arrange(child, world, child_bounds);
        }
    }
    child_bounds.size
}

impl Layout {
    pub fn new(termx: &Rc<dyn IsTermx>) -> Rc<dyn IsLayout> {
        Rc::new(unsafe { Self::new_raw(termx, LAYOUT_VTABLE.as_ptr()) })
    }

    pub unsafe fn new_raw(termx: &Rc<dyn IsTermx>, vtable: Vtable) -> Self {
        Layout {
            obj: unsafe { Obj::new_raw(vtable) },
            termx: Rc::downgrade(termx),
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
        let termx = layout.termx.upgrade().unwrap();
        let c = termx.termx().components();
        match entity.get(c.layout_view, world).unwrap().layout() {
            LAYOUT_BACKGROUND => measure_background(this, entity, world, w, h),
            LAYOUT_STACK_PANEL => measure_stack_panel(this, entity, world, w, h),
            LAYOUT_CANVAS => measure_canvas(this, entity, world, w, h),
            LAYOUT_T_BUTTON => measure_t_button(this, entity, world, w, h),
            LAYOUT_STATIC_TEXT => measure_static_text(this, entity, world, w, h),
            LAYOUT_CONTENT_PRESENTER => measure_content_presenter(this, entity, world, w, h),
            LAYOUT_BORDER => measure_border(this, entity, world, w, h),
            LAYOUT_ADORNERS_PANEL => measure_adorners_panel(this, entity, world, w, h),
            LAYOUT_CONTROL => measure_control(this, entity, world, w, h),
            LAYOUT_BUTTON => measure_button(this, entity, world, w, h),
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
        let termx = layout.termx.upgrade().unwrap();
        let c = termx.termx().components();
        match entity.get(c.layout_view, world).unwrap().layout() {
            LAYOUT_BACKGROUND => arrange_background(this, entity, world, inner_bounds),
            LAYOUT_STACK_PANEL => arrange_stack_panel(this, entity, world, inner_bounds),
            LAYOUT_CANVAS => arrange_canvas(this, entity, world, inner_bounds),
            LAYOUT_T_BUTTON => arrange_t_button(this, entity, world, inner_bounds),
            LAYOUT_STATIC_TEXT => arrange_static_text(this, entity, world, inner_bounds),
            LAYOUT_CONTENT_PRESENTER => arrange_content_presenter(this, entity, world, inner_bounds),
            LAYOUT_BORDER => arrange_border(this, entity, world, inner_bounds),
            LAYOUT_ADORNERS_PANEL => arrange_adorners_panel(this, entity, world, inner_bounds),
            LAYOUT_CONTROL => arrange_control(this, entity, world, inner_bounds),
            LAYOUT_BUTTON => arrange_button(this, entity, world, inner_bounds),
            _ => Vector::null()
        }
    }

    pub fn invalidate_measure_impl(
        this: &Rc<dyn IsLayout>,
        mut entity: Entity<Termx>,
        world: &mut World<Termx>,
    ) {
        let layout = this.layout();
        let termx = layout.termx.upgrade().unwrap();
        let c = termx.termx().components();
        loop {
            {
                let layout_view = entity.get_mut(c.layout_view, world).unwrap();
                layout_view.measure_size = None;
                layout_view.arrange_size = None;
            }
            let view = entity.get(c.view, world).unwrap();
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
        let termx = layout.termx.upgrade().unwrap();
        let c = termx.termx().components();
        loop {
            {
                let layout_view = entity.get_mut(c.layout_view, world).unwrap();
                layout_view.arrange_size = None;
            }
            let view = entity.get(c.view, world).unwrap();
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
        let layout = this.layout();
        let termx = layout.termx.upgrade().unwrap();
        let c = termx.termx().components();
        let component = entity.get_mut(c.layout_view, world).unwrap();
        if component.measure_size == Some((w, h)) { return; }
        component.measure_size = Some((w, h));
        if entity.get(c.view, world).unwrap().visibility() == Visibility::Collapsed {
            entity.get_mut(c.layout_view, world).unwrap().desired_size = Vector::null();
            return;
        }
        let component = entity.get(c.layout_view, world).unwrap();
        let (a_w, a_h, max_size, min_size) = {
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
        let component = entity.get_mut(c.layout_view, world).unwrap();
        let desired_size = desired_size.min(max_size).max(min_size);
        let desired_size = component.margin().expand_rect_size(desired_size);
        let desired_size = Vector {
            x: w.map_or(desired_size.x, |w| min(w as u16, desired_size.x as u16) as i16),
            y: h.map_or(desired_size.y, |h| min(h as u16, desired_size.y as u16) as i16),
        };
        component.desired_size = desired_size;
    }

    pub fn arrange_impl(
        this: &Rc<dyn IsLayout>,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        bounds: Rect,
    ) {
        let layout = this.layout();
        let termx = layout.termx.upgrade().unwrap();
        let c = termx.termx().components();
        let collapsed = entity.get(c.view, world).unwrap().visibility() == Visibility::Collapsed;
        let do_arrange = replace(
            &mut entity.get_mut(c.layout_view, world).unwrap().arrange_size,
            Some(bounds.size)
        ) != Some(bounds.size);
        let (render_bounds, real_render_bounds, real_render_bounds_with_shadow) = if collapsed {
            let rect = Rect { tl: Point { x: 0, y: 0 }, size: Vector::null() };
            (rect, rect, rect)
        } else {
            let (a_size, max_size, min_size) = {
                let component = entity.get(c.layout_view, world).unwrap();
                let max_width = component.width().or(component.max_width());
                let max_height = component.height().or(component.max_height());
                let max_size = Vector { x: max_width.unwrap_or(-1), y: max_height.unwrap_or(-1) };
                let min_size = Vector {
                    x: component.width().unwrap_or(component.min_size().x),
                    y: component.height().unwrap_or(component.min_size().y),
                };
                if !do_arrange {
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
                let component = entity.get(c.layout_view, world).unwrap();
                component.margin().expand_rect_size(render_size.min(max_size).max(min_size)).min(bounds.size)
            } else {
                let component = entity.get(c.layout_view, world).unwrap();
                component.render_bounds.size
            };
            let component = entity.get(c.layout_view, world).unwrap();
            let h_align = <Option<HAlign>>::from(component.h_align()).unwrap_or(HAlign::Left);
            let v_align = <Option<VAlign>>::from(component.v_align()).unwrap_or(VAlign::Top);
            let align = Thickness::align(render_size, bounds.size, h_align, v_align);
            let render_bounds = align.shrink_rect(bounds);
            let shadow = entity.get(c.view, world).unwrap().shadow;
            let real_render_bounds = component.margin().shrink_rect(render_bounds);
            let real_render_bounds_with_shadow = shadow.expand_rect(real_render_bounds);
            (render_bounds, real_render_bounds, real_render_bounds_with_shadow)
        };
        {
            let view = entity.get(c.view, world).unwrap();
            if
                   real_render_bounds == view.real_render_bounds
                && real_render_bounds_with_shadow == view.real_render_bounds_with_shadow
            {
                let component = entity.get_mut(c.layout_view, world).unwrap();
                component.render_bounds = render_bounds;
                return;
            }
        }
        termx.termx().systems().render.invalidate_render(entity, world);
        {
            let component = entity.get_mut(c.layout_view, world).unwrap();
            component.render_bounds = render_bounds;
        }
        {
            let view = entity.get_mut(c.view, world).unwrap();
            view.real_render_bounds = real_render_bounds;
            view.real_render_bounds_with_shadow = real_render_bounds_with_shadow;
        }
        termx.termx().systems().render.invalidate_render(entity, world);
    }

    pub fn perform_impl(this: &Rc<dyn IsLayout>, root: Entity<Termx>, world: &mut World<Termx>, size: Vector) {
        const MAX_ROUNDS: usize = 8;
        let layout = this.layout();
        let termx = layout.termx.upgrade().unwrap();
        let c = termx.termx().components();
        for _ in 0 .. MAX_ROUNDS {
            this.measure(root, world, Some(size.x), Some(size.y));
            this.arrange(root, world, Rect { tl: Point { x: 0, y: 0 }, size });
            let layout_view = root.get(c.layout_view, world).unwrap();
            if layout_view.measure_size.is_some() && layout_view.arrange_size.is_some() {
                break;
            }
        }
    }
}
