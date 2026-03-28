#![feature(macro_metavar_expr_concat)]

#![deny(warnings)]
#![doc(test(attr(deny(warnings))))]
#![doc(test(attr(allow(dead_code))))]
#![doc(test(attr(allow(unused_variables))))]

#![no_std]

extern crate alloc;

#[doc(hidden)]
pub use paste::paste as paste_paste;
#[doc(hidden)]
pub use alloc::boxed::Box as alloc_boxed_Box;
#[doc(hidden)]
pub use alloc::rc::Rc as alloc_rc_Rc;
#[doc(hidden)]
pub use alloc::string::String as alloc_string_String;
#[doc(hidden)]
pub use basic_oop::obj::IsObj as basic_oop_obj_IsObj;
#[doc(hidden)]
pub use ooecs::Entity as ooecs_Entity;

pub mod base;
pub mod components;
pub mod systems;
pub mod termx;
pub mod render_port;
pub mod template;

#[macro_export]
macro_rules! property_ro {
    ($Termx:ident, $component:ident, $name:ident, $get:ty) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> $get {
                self.$name
            }

            pub fn [< get_ $name >] (
                entity: $crate::ooecs_Entity,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> $get {
                let xtermx = termx. [< $Termx:snake >] ();
                let termx = termx.termx();
                let component = xtermx.components().$component;
                let world = termx.world.borrow();
                entity.get::<Self>(component, &world).unwrap().$name
            }
        }
    };
}

#[macro_export]
macro_rules! property_rw {
    ($Termx:ident, $component:ident, $name:ident, $get_set:ty) => {
        $crate::paste_paste! {
            pub fn [< get_ $name >] (
                entity: $crate::ooecs_Entity,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> $get_set {
                let xtermx = termx. [< $Termx:snake >] ();
                let termx = termx.termx();
                let component = xtermx.components().$component;
                let world = termx.world.borrow();
                entity.get::<Self>(component, &world).unwrap().$name
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $get_set
            ) {
                let xtermx = termx. [< $Termx:snake >] ();
                let termx = termx.termx();
                let component = xtermx.components().$component;
                let mut world = termx.world.borrow_mut();
                entity.get_mut::<Self>(component, &mut world).unwrap().$name = value;
            }
        }
    };
    ($Termx:ident, $component:ident, $name:ident, ref $set:ty as $get:ty) => {
        $crate::paste_paste! {
            pub fn [< get_ $name >] <T> (
                entity: $crate::ooecs_Entity,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                f: impl FnOnce($get) -> T
            ) -> T {
                let xtermx = termx. [< $Termx:snake >] ();
                let termx = termx.termx();
                let component = xtermx.components().$component;
                let world = termx.world.borrow();
                f(&entity.get::<Self>(component, &world).unwrap().$name)
            }

            pub fn [< get_ $name _mut >] <T> (
                entity: $crate::ooecs_Entity,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                f: impl FnOnce(&mut $set) -> T
            ) -> T {
                let xtermx = termx. [< $Termx:snake >] ();
                let termx = termx.termx();
                let component = xtermx.components().$component;
                let mut world = termx.world.borrow_mut();
                f(&mut entity.get_mut::<Self>(component, &mut world).unwrap().$name)
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $set
            ) {
                let xtermx = termx. [< $Termx:snake >] ();
                let termx = termx.termx();
                let component = xtermx.components().$component;
                let mut world = termx.world.borrow_mut();
                entity.get_mut::<Self>(component, &mut world).unwrap().$name = value;
            }
        }
    };
}

#[macro_export]
macro_rules! property {
    ($Termx:ident, $component:ident, $name:ident, $get_set:ty, @measure) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> $get_set {
                self.$name
            }

            pub fn [< get_ $name >] (
                entity: $crate::ooecs_Entity,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> $get_set {
                let xtermx = termx. [< $Termx:snake >] ();
                let termx = termx.termx();
                let component = xtermx.components().$component;
                let world = termx.world.borrow();
                entity.get::<Self>(component, &world).unwrap().$name
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $get_set
            ) {
                let xtermx = termx. [< $Termx:snake >] ();
                let termx = termx.termx();
                let component = xtermx.components().$component;
                let mut world = termx.world.borrow_mut();
                entity.get_mut::<Self>(component, &mut world).unwrap().$name = value;
                $crate::systems::layout::LayoutExt::invalidate_measure(
                    &termx.systems().layout, entity, &mut world
                );
            }
        }
    };
    ($Termx:ident, $component:ident, $name:ident, $get_set:ty, @arrange) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> $get_set {
                self.$name
            }

            pub fn [< get_ $name >] (
                entity: $crate::ooecs_Entity,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> $get_set {
                let xtermx = termx. [< $Termx:snake >] ();
                let termx = termx.termx();
                let component = xtermx.components().$component;
                let world = termx.world.borrow();
                entity.get::<Self>(component, &world).unwrap().$name
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $get_set
            ) {
                let xtermx = termx. [< $Termx:snake >] ();
                let termx = termx.termx();
                let component = xtermx.components().$component;
                let mut world = termx.world.borrow_mut();
                entity.get_mut::<Self>(component, &mut world).unwrap().$name = value;
                $crate::systems::layout::LayoutExt::invalidate_arrange(
                    &termx.systems().layout, entity, &mut world
                );
            }
        }
    };
    ($Termx:ident, $component:ident, $name:ident, $get_set:ty, @render) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> $get_set {
                self.$name
            }

            pub fn [< get_ $name >] (
                entity: $crate::ooecs_Entity,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> $get_set {
                let xtermx = termx. [< $Termx:snake >] ();
                let termx = termx.termx();
                let component = xtermx.components().$component;
                let world = termx.world.borrow();
                entity.get::<Self>(component, &world).unwrap().$name
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $get_set
            ) {
                let xtermx = termx. [< $Termx:snake >] ();
                let termx = termx.termx();
                let component = xtermx.components().$component;
                let mut world = termx.world.borrow_mut();
                entity.get_mut::<Self>(component, &mut world).unwrap().$name = value;
                $crate::systems::render::RenderExt::invalidate_render(
                    &termx.systems().render, entity, &mut world
                );
            }
        }
    };
    ($Termx:ident, $component:ident, $name:ident, ref $set:ty as $get:ty, @measure) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> $get {
                &self.$name
            }

            pub fn [< get_ $name >] <T> (
                entity: $crate::ooecs_Entity,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                f: impl FnOnce($get) -> T
            ) -> T {
                let xtermx = termx. [< $Termx:snake >] ();
                let termx = termx.termx();
                let component = xtermx.components().$component;
                let world = termx.world.borrow();
                f(&entity.get::<Self>(component, &world).unwrap().$name)
            }

            pub fn [< get_ $name _mut >] <T> (
                entity: $crate::ooecs_Entity,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                f: impl FnOnce(&mut $set) -> T
            ) -> T {
                let xtermx = termx. [< $Termx:snake >] ();
                let termx = termx.termx();
                let component = xtermx.components().$component;
                let mut world = termx.world.borrow_mut();
                let res = f(&mut entity.get_mut::<Self>(component, &mut world).unwrap().$name);
                $crate::systems::layout::LayoutExt::invalidate_measure(
                    &termx.systems().layout, entity, &mut world
                );
                res
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $set
            ) {
                let xtermx = termx. [< $Termx:snake >] ();
                let termx = termx.termx();
                let component = xtermx.components().$component;
                let mut world = termx.world.borrow_mut();
                entity.get_mut::<Self>(component, &mut world).unwrap().$name = value;
                $crate::systems::layout::LayoutExt::invalidate_measure(
                    &termx.systems().layout, entity, &mut world
                );
            }
        }
    };
    ($Termx:ident, $component:ident, $name:ident, ref $set:ty as $get:ty, @arrange) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> $get {
                &self.$name
            }

            pub fn [< get_ $name >] <T> (
                entity: $crate::ooecs_Entity,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                f: impl FnOnce($get) -> T
            ) -> T {
                let xtermx = termx. [< $Termx:snake >] ();
                let termx = termx.termx();
                let component = xtermx.components().$component;
                let world = termx.world.borrow();
                f(&entity.get::<Self>(component, &world).unwrap().$name)
            }

            pub fn [< get_ $name _mut >] <T> (
                entity: $crate::ooecs_Entity,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                f: impl FnOnce(&mut $set) -> T
            ) -> T {
                let xtermx = termx. [< $Termx:snake >] ();
                let termx = termx.termx();
                let component = xtermx.components().$component;
                let mut world = termx.world.borrow_mut();
                let res = f(&mut entity.get_mut::<Self>(component, &mut world).unwrap().$name);
                $crate::systems::layout::LayoutExt::invalidate_arrange(
                    &termx.systems().layout, entity, &mut world
                );
                res
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $set
            ) {
                let xtermx = termx. [< $Termx:snake >] ();
                let termx = termx.termx();
                let component = xtermx.components().$component;
                let mut world = termx.world.borrow_mut();
                entity.get_mut::<Self>(component, &mut world).unwrap().$name = value;
                $crate::systems::layout::LayoutExt::invalidate_arrange(
                    &termx.systems().layout, entity, &mut world
                );
            }
        }
    };
    ($Termx:ident, $component:ident, $name:ident, ref $set:ty as $get:ty, @render) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> $get {
                &self.$name
            }

            pub fn [< get_ $name >] <T> (
                entity: $crate::ooecs_Entity,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                f: impl FnOnce($get) -> T
            ) -> T {
                let xtermx = termx. [< $Termx:snake >] ();
                let termx = termx.termx();
                let component = xtermx.components().$component;
                let world = termx.world.borrow();
                f(&entity.get::<Self>(component, &world).unwrap().$name)
            }

            pub fn [< get_ $name _mut >] <T> (
                entity: $crate::ooecs_Entity,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                f: impl FnOnce(&mut $set) -> T
            ) -> T {
                let xtermx = termx. [< $Termx:snake >] ();
                let termx = termx.termx();
                let component = xtermx.components().$component;
                let mut world = termx.world.borrow_mut();
                let res = f(&mut entity.get_mut::<Self>(component, &mut world).unwrap().$name);
                $crate::systems::layout::LayoutExt::invalidate_measure(
                    &termx.systems().layout, entity, &mut world
                );
                res
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $set
            ) {
                let xtermx = termx. [< $Termx:snake >] ();
                let termx = termx.termx();
                let component = xtermx.components().$component;
                let mut world = termx.world.borrow_mut();
                entity.get_mut::<Self>(component, &mut world).unwrap().$name = value;
                $crate::systems::render::RenderExt::invalidate_render(
                    &termx.systems().render, entity, &mut world
                );
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::alloc::string::ToString;
    use crate::components::background::Background;
    use crate::systems::layout::LayoutExt;
    use crate::termx::Termx;
    use int_vec_2d::Vector;

    #[test]
    fn create_view_perform_layout_change_property() {
        let termx = Termx::new();
        let bg = Background::new_entity(&termx);
        {
            let termx = termx.termx();
            let mut world = termx.world.borrow_mut();
            termx.systems().layout.perform(bg, &mut world, Vector { x: 80, y: 25 });
        }
        Background::set_pattern(bg, &termx, "x".to_string());
        assert_eq!(&Background::get_pattern(bg, &termx, |x| x.to_string()), "x");
    }
}
