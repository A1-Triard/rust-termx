use basic_oop::{Vtable, import, class_unsafe};
use int_vec_2d::{HAlign, VAlign, Thickness};
use std::cell::RefCell;
use std::cmp::min;

pub struct View {
    visual_parent: Option<Entity>,
    measure_size: Option<(Option<i16>, Option<i16>)>,
    desired_size: Vector,
    arrange_size: Option<Vector>,
    render_bounds: Rect,
    min_size: Vector,
    max_width: Option<i16>,
    max_height: Option<i16>,
    width: Option<i16>,
    height: Option<i16>,
    margin: Thickness,
    h_align: Option<HAlign>,
    v_align: Option<VAlign>,
}

impl View {
    pub fn set_min_size(entity: Entity, termx: Rc<dyn IsTermx>, value: Vector) {
        let termx = termx.termx();
        entity.component::<View>(&mut termx.world.borrow_mut(), termx.view).unwrap().min_size = value;
        termx.layout.invalidate_measure(entity, &mut termx.world.borrow_mut());
    }
}

import! { pub termx:
    use [obj basic_oop::obj];
    use std::rc::Rc;
    use ooecs::{Entity, Component, World};
}

#[class_unsafe(inherits_Obj)]
pub struct Termx {
    pub world: Rc<RefCell<World>>,
    pub view: Component,
    pub layout: Rc<dyn IsLayout>,
    #[virt]
    create_layout: fn() -> Rc<dyn IsLayout>,
}

impl Termx {
    pub fn create_layout_impl(this: &Rc<dyn IsTermx>) -> Rc<dyn IsLayout> {
        let termx = this.termx();
        Layout::new(termx.view)
    }
}

import! { pub layout:
    use [obj basic_oop::obj];
    use int_vec_2d::Vector;
    use ooecs::{Entity, Component, World};
}

#[class_unsafe(inherits_Obj)]
pub struct Layout {
    view: Component,
    #[virt]
    visual_children_count: fn(entity: Entity, world: &mut World) -> usize,
    #[virt]
    visual_child: fn(entity: Entity, world: &mut World, index: usize) -> Entity,
    #[virt]
    measure_override: fn(entity: Entity, world: &mut World, w: Option<i16>, h: Option<i16>) -> Vector,
    #[non_virt]
    add_visual_child: fn(parent: Entity, child: Entity, world: &mut World),
    #[non_virt]
    remove_visual_child: fn(parent: Entity, child: Entity, world: &mut World),
    #[non_virt]
    invalidate_measure: fn(entity: Entity, world: &mut World),
    #[non_virt]
    measure: fn(entity: Entity, world: &mut World, w: Option<i16>, h: Option<i16>),
    #[non_virt]
    perform: fn(root: Entity, world: &mut World, size: Vector),
}

impl Layout {
    pub fn new(view: Component) -> Rc<dyn IsLayout> {
        Rc::new(unsafe { Self::new_raw(view, LAYOUT_VTABLE.as_ptr()) })
    }

    pub unsafe fn new_raw(view: Component, vtable: Vtable) -> Self {
        Layout {
            obj: unsafe { Obj::new_raw(vtable) },
            view,
        }
    }

    pub fn visual_children_count_impl(_this: &Rc<dyn IsLayout>, _entity: Entity, _world: &mut World) -> usize {
        0
    }

    pub fn visual_child_impl(
        _this: &Rc<dyn IsLayout>,
        _entity: Entity,
        _world: &mut World,
        _index: usize,
    ) -> Entity {
        panic!()
    }

    pub fn measure_override_impl(
        _this: &Rc<dyn IsLayout>,
        _entity: Entity,
        _world: &mut World,
        _w: Option<i16>,
        _h: Option<i16>
    ) -> Vector {
        Vector::null()
    }

    pub fn add_visual_child_impl(this: &Rc<dyn IsLayout>, parent: Entity, child: Entity, world: &mut World) {
        let layout = this.layout();
        child.component::<View>(world, layout.view).unwrap().visual_parent = Some(parent);
    }

    pub fn remove_visual_child_impl(
        this: &Rc<dyn IsLayout>,
        parent: Entity,
        child: Entity,
        world: &mut World
    ) {
        let layout = this.layout();
        let mut component = child.component::<View>(world, layout.view).unwrap();
        assert_eq!(component.visual_parent, Some(parent));
        component.visual_parent = None;
    }

    pub fn invalidate_measure_impl(this: &Rc<dyn IsLayout>, mut entity: Entity, world: &mut World) {
        let layout = this.layout();
        loop {
            let mut component = entity.component::<View>(world, layout.view).unwrap();
            component.measure_size = None;
            let Some(parent) = component.visual_parent else { break; };
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
            let component = entity.component::<View>(world, layout.view).unwrap();
            if component.measure_size == Some((w, h)) { return; }
            let max_width = component.width.or(component.max_width);
            let max_height = component.height.or(component.max_height);
            let max_size = Vector { x: max_width.unwrap_or(-1), y: max_height.unwrap_or(-1) };
            let min_size = Vector {
                x: component.width.unwrap_or(component.min_size.x),
                y: component.height.unwrap_or(component.min_size.y),
            };
            let g_w = if component.h_align.is_some() { None } else { w };
            let g_h = if component.v_align.is_some() { None } else { h };
            let g_w = g_w.or(max_width);
            let g_h = g_h.or(max_height);
            let a = Vector { x: g_w.unwrap_or(0), y: g_h.unwrap_or(0) };
            let a = component.margin.shrink_rect_size(a);
            let a = a.min(max_size).max(min_size);
            (g_w.map(|_| a.x), g_h.map(|_| a.y), max_size, min_size)
        };
        let desired_size = this.measure_override(entity, world, a_w, a_h);
        let layout = this.layout();
        let mut component = entity.component::<View>(world, layout.view).unwrap();
        let desired_size = desired_size.min(max_size).max(min_size);
        let desired_size = component.margin.expand_rect_size(desired_size);
        let desired_size = Vector {
            x: w.map_or(desired_size.x, |w| min(w as u16, desired_size.x as u16) as i16),
            y: h.map_or(desired_size.y, |h| min(h as u16, desired_size.y as u16) as i16),
        };
        component.measure_size = Some((w, h));
        component.desired_size = desired_size;
    }

    pub fn perform_impl(this: &Rc<dyn IsLayout>, root: Entity, world: &mut World, size: Vector) {
        this.measure(root, world, Some(size.x), Some(size.y));
    }
}

