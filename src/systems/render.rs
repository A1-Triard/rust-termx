use alloc::rc::Rc;
use basic_oop::{Vtable, import, class_unsafe};
use core::cell::Cell;
use crate::components::background::Background;
use crate::components::decorator::Decorator;
use crate::components::view::*;
use crate::render_port::RenderPort;
use int_vec_2d::{Vector, Point, Rect};
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
    pub background: Component<Background, Termx>,
    cursor: Cell<Option<Point>>,
    invalidated_rect: Cell<Rect>,
    screen_rect: Cell<Rect>,
    #[virt]
    visual_children_count: fn(entity: Entity<Termx>, world: &World<Termx>) -> usize,
    #[virt]
    visual_child: fn(entity: Entity<Termx>, world: &World<Termx>, index: usize) -> Entity<Termx>,
    #[virt]
    render_view: fn(entity: Entity<Termx>, world: &World<Termx>, rp: &mut RenderPort),
    #[non_virt]
    add_visual_child: fn(parent: Entity<Termx>, child: Entity<Termx>, world: &mut World<Termx>),
    #[non_virt]
    remove_visual_child: fn(parent: Entity<Termx>, child: Entity<Termx>, world: &mut World<Termx>),
    #[non_virt]
    invalidate_render: fn(entity: Entity<Termx>, world: &World<Termx>),
    #[non_virt]
    perform: fn(root: Entity<Termx>, world: &World<Termx>, screen: &mut dyn Screen) -> Option<Point>,
}

impl Render {
    pub fn new(
        view: Component<View, Termx>,
        decorator: Component<Decorator, Termx>,
        background: Component<Background, Termx>,
    ) -> Rc<dyn IsRender> {
        Rc::new(unsafe { Self::new_raw(
            view,
            decorator,
            background,
            RENDER_VTABLE.as_ptr(),
        ) })
    }

    pub unsafe fn new_raw(
        view: Component<View, Termx>,
        decorator: Component<Decorator, Termx>,
        background: Component<Background, Termx>,
        vtable: Vtable,
    ) -> Self {
        Render {
            obj: unsafe { Obj::new_raw(vtable) },
            view,
            decorator,
            background,
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
        match entity.get::<View>(render.view, world).unwrap().tree() {
            TREE_DECORATOR => {
                let decorator = entity.get::<Decorator>(render.decorator, world).unwrap();
                if decorator.child.is_some() { 1 } else { 0 }
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
        match entity.get::<View>(render.view, world).unwrap().tree() {
            TREE_DECORATOR => {
                let decorator = entity.get::<Decorator>(render.decorator, world).unwrap();
                assert_eq!(index, 0);
                decorator.child.unwrap()
            },
            _ => panic!(),
        }
    }

    pub fn render_view_impl(
        this: &Rc<dyn IsRender>,
        entity: Entity<Termx>,
        world: &World<Termx>,
        rp: &mut RenderPort
    ) {
        let render = this.render();
        match entity.get::<View>(render.view, world).unwrap().render() {
            RENDER_BACKGROUND => {
                let background = entity.get::<Background>(render.background, world).unwrap();
                rp.fill(|rp, p| rp.text(p, background.color, &background.pattern));
            },
            _ => { },
        }
    }

    pub fn add_visual_child_impl(
        this: &Rc<dyn IsRender>,
        parent: Entity<Termx>,
        child: Entity<Termx>,
        world: &mut World<Termx>,
    ) {
        let render = this.render();
        child.get_mut::<View>(render.view, world).unwrap().visual_parent = Some(parent);
        this.invalidate_render(child, world);
    }

    pub fn remove_visual_child_impl(
        this: &Rc<dyn IsRender>,
        parent: Entity<Termx>,
        child: Entity<Termx>,
        world: &mut World<Termx>,
    ) {
        this.invalidate_render(child, world);
        let render = this.render();
        let view = child.get_mut::<View>(render.view, world).unwrap();
        assert_eq!(view.visual_parent, Some(parent));
        view.visual_parent = None;
    }

    pub fn invalidate_render_impl(this: &Rc<dyn IsRender>, entity: Entity<Termx>, world: &World<Termx>) {
        let render = this.render();
        let rect = entity.get::<View>(render.view, world).unwrap().real_render_bounds;
        let union = render.invalidated_rect.get().union_intersect(rect, render.screen_rect.get());
        render.invalidated_rect.set(union);
    }

    fn render_entity(
        this: &Rc<dyn IsRender>,
        entity: Entity<Termx>,
        world: &World<Termx>,
        rp: &mut RenderPort,
    ) {
        if rp.invalidated_rect.intersect(rp.bounds).is_empty() {
            return;
        }
        //if view.visibility() != Visibility::Visible { return; }
        this.render_view(entity, world, rp);
        let render = this.render();
        let base_offset = rp.offset;
        let base_bounds = rp.bounds;
        for i in 0 .. this.visual_children_count(entity, world) {
            let child = this.visual_child(entity, world, i);
            let view = child.get::<View>(render.view, world).unwrap();
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
        let bounds = root.get::<View>(render.view, world).unwrap().real_render_bounds;
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
