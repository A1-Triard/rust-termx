use alloc::rc::{self, Rc};
use basic_oop::{Vtable, import, class_unsafe};
use crate::components::view::*;
use crate::systems::render::RenderExt;
use crate::termx::IsTermx;
use int_vec_2d::{Thickness, Vector};

import! { pub reactive:
    use [obj basic_oop::obj];
    use crate::termx::Termx;
    use ooecs::{Entity, World};
}

#[class_unsafe(inherits_Obj)]
pub struct Reactive {
    termx: rc::Weak<dyn IsTermx>,
    #[virt]
    button_is_pressed_changed: fn(entity: Entity<Termx>, world: &mut World<Termx>),
}

impl Reactive {
    pub fn new(termx: &Rc<dyn IsTermx>) -> Rc<dyn IsReactive> {
        Rc::new(unsafe { Self::new_raw(termx, REACTIVE_VTABLE.as_ptr()) })
    }

    pub unsafe fn new_raw(termx: &Rc<dyn IsTermx>, vtable: Vtable) -> Self {
        Reactive {
            obj: unsafe { Obj::new_raw(vtable) },
            termx: Rc::downgrade(termx),
        }
    }

    pub fn button_is_pressed_changed_impl(
        this: &Rc<dyn IsReactive>,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
    ) {
        let reactive = this.reactive();
        let termx = reactive.termx.upgrade().unwrap();
        let c = termx.termx().components();
        let s = termx.termx().systems();
        if entity.get(c.view, world).unwrap().render() == RENDER_T_BUTTON {
            if entity.get(c.button, world).unwrap().is_pressed() {
                s.render.set_shadow(entity, world, Thickness::all(0));
                s.render.set_visual_offset(entity, world, Vector { x: 1, y: 0 });
            } else {
                s.render.set_shadow(entity, world, Thickness::new(0, 0, 1, 1));
                s.render.set_visual_offset(entity, world, Vector::null());
            }
        }
    }
}
