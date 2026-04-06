use alloc::rc::{self, Rc};
use basic_oop::{Vtable, import, class_unsafe};
use crate::systems::render::RenderExt;
use crate::termx::IsTermx;
use int_vec_2d::Thickness;

import! { pub init:
    use [obj basic_oop::obj];
    use crate::termx::Termx;
    use ooecs::{Entity, World};
}

#[class_unsafe(inherits_Obj)]
pub struct Init {
    termx: rc::Weak<dyn IsTermx>,
    #[virt]
    init_t_button: fn(entity: Entity<Termx>, world: &mut World<Termx>),
}

impl Init {
    pub fn new(termx: &Rc<dyn IsTermx>) -> Rc<dyn IsInit> {
        Rc::new(unsafe { Self::new_raw(termx, INIT_VTABLE.as_ptr()) })
    }

    pub unsafe fn new_raw(termx: &Rc<dyn IsTermx>, vtable: Vtable) -> Self {
        Init {
            obj: unsafe { Obj::new_raw(vtable) },
            termx: Rc::downgrade(termx),
        }
    }

    pub fn init_t_button_impl(this: &Rc<dyn IsInit>, entity: Entity<Termx>, world: &mut World<Termx>) {
        let init = this.init();
        let termx = init.termx.upgrade().unwrap();
        let s = termx.termx().systems();
        s.render.set_shadow(entity, world, Thickness::new(0, 0, 1, 1));
    }
}
