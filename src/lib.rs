#![feature(macro_metavar_expr_concat)]
#![feature(slice_from_ptr_range)]

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
pub use alloc::vec::Vec as alloc_vec_Vec;
#[doc(hidden)]
pub use basic_oop::obj::IsObj as basic_oop_obj_IsObj;
#[doc(hidden)]
pub use ooecs::Entity as ooecs_Entity;
#[doc(hidden)]
pub use ooecs::World as ooecs_World;

pub mod base;
pub mod components;
pub mod event_handler;
pub mod systems;
pub mod termx;
pub mod render_port;
pub mod template;
pub mod xaml;
pub mod text_renderer;
pub mod line_edit;

#[macro_export]
macro_rules! property_ro {
    ($Termx:ident, $component:ident, $name:ident, $ty:ty) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> $ty {
                self.$name
            }

            pub fn [< get_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &$crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> $ty {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get(c.$component, world).unwrap().$name
            }
        }
    };
    ($Termx:ident, $component:ident, $name:ident, ref $ty:ty) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> &$ty {
                &self.$name
            }

            pub fn [< get_ $name >] <'a> (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &'a $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> &'a $ty {
                let c = termx. [< $Termx:snake >] ().components();
                &entity.get(c.$component, world).unwrap().$name
            }
        }
    };
}

#[macro_export]
macro_rules! property_rw {
    ($Termx:ident, $component:ident, $name:ident, $ty:ty) => {
        $crate::paste_paste! {
            pub fn [< get_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &$crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> $ty {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get(c.$component, world).unwrap().$name
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $ty
            ) {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get_mut(c.$component, world).unwrap().$name = value;
            }
        }
    };
    ($Termx:ident, $component:ident, $name:ident, ref $ty:ty) => {
        $crate::paste_paste! {
            pub fn [< get_ $name >] <'a> (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &'a $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> &'a $ty {
                let c = termx. [< $Termx:snake >] ().components();
                &entity.get(c.$component, world).unwrap().$name
            }

            pub fn [< get_ $name _mut >] <T> (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                f: impl FnOnce(&mut $ty) -> T
            ) -> T {
                let c = termx. [< $Termx:snake >] ().components();
                f(&mut entity.get_mut(c.$component, world).unwrap().$name)
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $ty
            ) {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get_mut(c.$component, world).unwrap().$name = value;
            }
        }
    };
}

#[macro_export]
macro_rules! property {
    ($Termx:ident, $component:ident, $name:ident, $ty:ty, $callback:ident) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> $ty {
                self.$name
            }

            pub fn [< get_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &$crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> $ty {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get(c.$component, world).unwrap().$name
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $ty
            ) {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get_mut(c.$component, world).unwrap().$name = value;
                Self::$callback(entity, world, termx);
            }
        }
    };
    ($Termx:ident, $component:ident, $name:ident, $ty:ty, @measure) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> $ty {
                self.$name
            }

            pub fn [< get_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &$crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> $ty {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get(c.$component, world).unwrap().$name
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $ty
            ) {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get_mut(c.$component, world).unwrap().$name = value;
                let s = termx.termx().systems();
                $crate::systems::layout::LayoutExt::invalidate_measure(
                    &s.layout, entity, world
                );
            }
        }
    };
    ($Termx:ident, $component:ident, $name:ident, $ty:ty, @arrange) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> $ty {
                self.$name
            }

            pub fn [< get_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &$crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> $ty {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get(c.$component, world).unwrap().$name
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $ty
            ) {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get_mut(c.$component, world).unwrap().$name = value;
                let s = termx.termx().systems();
                $crate::systems::layout::LayoutExt::invalidate_arrange(
                    &s.layout, entity, world
                );
            }
        }
    };
    ($Termx:ident, $component:ident, $name:ident, $ty:ty, @render) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> $ty {
                self.$name
            }

            pub fn [< get_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &$crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> $ty {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get(c.$component, world).unwrap().$name
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $ty
            ) {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get_mut(c.$component, world).unwrap().$name = value;
                let s = termx.termx().systems();
                $crate::systems::render::RenderExt::invalidate_render(
                    &s.render, entity, world
                );
            }
        }
    };
    ($Termx:ident, $component:ident, $name:ident, $ty:ty, @render+measure) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> $ty {
                self.$name
            }

            pub fn [< get_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &$crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> $ty {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get(c.$component, world).unwrap().$name
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $ty
            ) {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get_mut(c.$component, world).unwrap().$name = value;
                let s = termx.termx().systems();
                $crate::systems::render::RenderExt::invalidate_render(
                    &s.render, entity, world
                );
                $crate::systems::layout::LayoutExt::invalidate_measure(
                    &s.layout, entity, world
                );
            }
        }
    };
    ($Termx:ident, $component:ident, $name:ident, $ty:ty, @render+arrange) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> $ty {
                self.$name
            }

            pub fn [< get_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &$crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> $ty {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get(c.$component, world).unwrap().$name
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $ty
            ) {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get_mut(c.$component, world).unwrap().$name = value;
                let s = termx.termx().systems();
                $crate::systems::render::RenderExt::invalidate_render(
                    &s.render, entity, world
                );
                $crate::systems::layout::LayoutExt::invalidate_arrange(
                    &s.layout, entity, world
                );
            }
        }
    };
    ($Termx:ident, $component:ident, $name:ident, ref $ty:ty, $callback:ident) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> &$ty {
                &self.$name
            }

            pub fn [< get_ $name >] <'a> (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &'a $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> &'a $ty {
                let c = termx. [< $Termx:snake >] ().components();
                &entity.get(c.$component, world).unwrap().$name
            }

            pub fn [< get_ $name _mut >] <T> (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                f: impl FnOnce(&mut $ty) -> T
            ) -> T {
                let c = termx. [< $Termx:snake >] ().components();
                let res = f(&mut entity.get_mut(c.$component, world).unwrap().$name);
                Self::$callback(entity, world, termx);
                res
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $ty
            ) {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get_mut(c.$component, world).unwrap().$name = value;
                Self::$callback(entity, world, termx);
            }
        }
    };
    ($Termx:ident, $component:ident, $name:ident, ref $ty:ty, @measure) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> &$ty {
                &self.$name
            }

            pub fn [< get_ $name >] <'a> (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &'a $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> &'a $ty {
                let c = termx. [< $Termx:snake >] ().components();
                &entity.get(c.$component, world).unwrap().$name
            }

            pub fn [< get_ $name _mut >] <T> (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                f: impl FnOnce(&mut $ty) -> T
            ) -> T {
                let c = termx. [< $Termx:snake >] ().components();
                let res = f(&mut entity.get_mut(c.$component, world).unwrap().$name);
                let s = termx.termx().systems();
                $crate::systems::layout::LayoutExt::invalidate_measure(
                    &s.layout, entity, world
                );
                res
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $ty
            ) {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get_mut(c.$component, world).unwrap().$name = value;
                let s = termx.termx().systems();
                $crate::systems::layout::LayoutExt::invalidate_measure(
                    &s.layout, entity, world
                );
            }
        }
    };
    ($Termx:ident, $component:ident, $name:ident, ref $ty:ty, @arrange) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> &$ty {
                &self.$name
            }

            pub fn [< get_ $name >] <'a> (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &'a $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> &'a $ty {
                let c = termx. [< $Termx:snake >] ().components();
                &entity.get(c.$component, world).unwrap().$name
            }

            pub fn [< get_ $name _mut >] <T> (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                f: impl FnOnce(&mut $ty) -> T
            ) -> T {
                let c = termx. [< $Termx:snake >] ().components();
                let res = f(&mut entity.get_mut(c.$component, world).unwrap().$name);
                let s = termx.termx().systems();
                $crate::systems::layout::LayoutExt::invalidate_arrange(
                    &s.layout, entity, world
                );
                res
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $ty
            ) {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get_mut(c.$component, world).unwrap().$name = value;
                let s = termx.termx().systems();
                $crate::systems::layout::LayoutExt::invalidate_arrange(
                    &s.layout, entity, world
                );
            }
        }
    };
    ($Termx:ident, $component:ident, $name:ident, ref $ty:ty, @render) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> &$ty {
                &self.$name
            }

            pub fn [< get_ $name >] <'a> (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &'a $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> &'a $ty {
                let c = termx. [< $Termx:snake >] ().components();
                &entity.get(c.$component, world).unwrap().$name
            }

            pub fn [< get_ $name _mut >] <T> (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                f: impl FnOnce(&mut $ty) -> T
            ) -> T {
                let c = termx. [< $Termx:snake >] ().components();
                let res = f(&mut entity.get_mut(c.$component, world).unwrap().$name);
                let s = termx.termx().systems();
                $crate::systems::render::RenderExt::invalidate_render(
                    &s.render, entity, world
                );
                res
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $ty
            ) {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get_mut(c.$component, world).unwrap().$name = value;
                let s = termx.termx().systems();
                $crate::systems::render::RenderExt::invalidate_render(
                    &s.render, entity, world
                );
            }
        }
    };
    ($Termx:ident, $component:ident, $name:ident, ref $ty:ty, @render+measure) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> &$ty {
                &self.$name
            }

            pub fn [< get_ $name >] <'a> (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &'a $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> &'a $ty {
                let c = termx. [< $Termx:snake >] ().components();
                &entity.get(c.$component, world).unwrap().$name
            }

            pub fn [< get_ $name _mut >] <T> (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                f: impl FnOnce(&mut $ty) -> T
            ) -> T {
                let c = termx. [< $Termx:snake >] ().components();
                let res = f(&mut entity.get_mut(c.$component, world).unwrap().$name);
                let s = termx.termx().systems();
                $crate::systems::render::RenderExt::invalidate_render(
                    &s.render, entity, world
                );
                $crate::systems::layout::LayoutExt::invalidate_measure(
                    &s.layout, entity, world
                );
                res
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $ty
            ) {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get_mut(c.$component, world).unwrap().$name = value;
                let s = termx.termx().systems();
                $crate::systems::render::RenderExt::invalidate_render(
                    &s.render, entity, world
                );
                $crate::systems::layout::LayoutExt::invalidate_measure(
                    &s.layout, entity, world
                );
            }
        }
    };
    ($Termx:ident, $component:ident, $name:ident, ref $ty:ty, @render+arrange) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> &$ty {
                &self.$name
            }

            pub fn [< get_ $name >] <'a> (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &'a $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> &'a $ty {
                let c = termx. [< $Termx:snake >] ().components();
                &entity.get(c.$component, world).unwrap().$name
            }

            pub fn [< get_ $name _mut >] <T> (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                f: impl FnOnce(&mut $ty) -> T
            ) -> T {
                let c = termx. [< $Termx:snake >] ().components();
                let res = f(&mut entity.get_mut(c.$component, world).unwrap().$name);
                let s = termx.termx().systems();
                $crate::systems::render::RenderExt::invalidate_render(
                    &s.render, entity, world
                );
                $crate::systems::layout::LayoutExt::invalidate_arrange(
                    &s.layout, entity, world
                );
                res
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $ty
            ) {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get_mut(c.$component, world).unwrap().$name = value;
                let s = termx.termx().systems();
                $crate::systems::render::RenderExt::invalidate_render(
                    &s.render, entity, world
                );
                $crate::systems::layout::LayoutExt::invalidate_arrange(
                    &s.layout, entity, world
                );
            }
        }
    };
}

#[macro_export]
macro_rules! layout_property {
    ($Termx:ident, $component:ident, $name:ident, $ty:ty, @measure) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> $ty {
                self.$name
            }

            pub fn [< get_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &$crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> $ty {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get(c.$component, world).unwrap().$name
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $ty
            ) {
                let xc = termx. [< $Termx:snake >] ().components();
                entity.get_mut(xc.$component, world).unwrap().$name = value;
                let c = termx.termx().components();
                let owner = entity.get(c.view_layout, world).unwrap().owner();
                if let Some(owner) = onwer {
                    let owner_parent = owner.get(c.view, world).unwrap().visual_parent();
                    if let Some(owner_parent) = owner_parent {
                        let s = termx.termx().systems();
                        $crate::systems::layout::LayoutExt::invalidate_measure(
                            &s.layout, owner_parent, world
                        );
                    }
                }
            }
        }
    };
    ($Termx:ident, $component:ident, $name:ident, $ty:ty, @arrange) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> $ty {
                self.$name
            }

            pub fn [< get_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &$crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> $ty {
                let c = termx. [< $Termx:snake >] ().components();
                entity.get(c.$component, world).unwrap().$name
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $ty
            ) {
                let xc = termx. [< $Termx:snake >] ().components();
                entity.get_mut(xc.$component, world).unwrap().$name = value;
                let c = termx.termx().components();
                let owner = entity.get(c.view_layout, world).unwrap().owner();
                if let Some(owner) = owner {
                    let owner_parent = owner.get(c.view, world).unwrap().visual_parent();
                    if let Some(owner_parent) = owner_parent {
                        let s = termx.termx().systems();
                        $crate::systems::layout::LayoutExt::invalidate_arrange(
                            &s.layout, owner_parent, world
                        );
                    }
                }
            }
        }
    };
    ($Termx:ident, $component:ident, $name:ident, ref $ty:ty, @measure) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> &$ty {
                &self.$name
            }

            pub fn [< get_ $name >] <'a> (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &'a $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> &'a $ty {
                let c = termx. [< $Termx:snake >] ().components();
                &entity.get(c.$component, world).unwrap().$name
            }

            pub fn [< get_ $name _mut >] <T> (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                f: impl FnOnce(&mut $ty) -> T
            ) -> T {
                let xc = termx. [< $Termx:snake >] ().components();
                let res = f(&mut entity.get_mut(xc.$component, world).unwrap().$name);
                let c = termx.termx().components();
                let owner = entity.get(c.view_layout, world).unwrap().owner();
                if let Some(owner) = owner {
                    let owner_parent = owner.get(c.view, world).unwrap().visual_parent();
                    if let Some(owner_parent) = owner_parent {
                        let s = termx.termx().systems();
                        $crate::systems::layout::LayoutExt::invalidate_measure(
                            &s.layout, owner_parent, world
                        );
                    }
                }
                res
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $ty
            ) {
                let xc = termx. [< $Termx:snake >] ().components();
                entity.get_mut(xc.$component, world).unwrap().$name = value;
                let c = termx.termx().components();
                let owner = entity.get(c.view_layout, world).unwrap().owner();
                if let Some(owner) = owner {
                    let owner_parent = owner.get(c.view, world).unwrap().visual_parent();
                    if let Some(owner_parent) = owner_parent {
                        let s = termx.termx().systems();
                        $crate::systems::layout::LayoutExt::invalidate_measure(
                            &s.layout, owner_parent, world
                        );
                    }
                }
            }
        }
    };
    ($Termx:ident, $component:ident, $name:ident, ref $ty:ty, @arrange) => {
        $crate::paste_paste! {
            pub fn $name(&self) -> &$ty {
                &self.$name
            }

            pub fn [< get_ $name >] <'a> (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &'a $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
            ) -> &'a $ty {
                let c = termx. [< $Termx:snake >] ().components();
                &entity.get(c.$component, world).unwrap().$name
            }

            pub fn [< get_ $name _mut >] <T> (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                f: impl FnOnce(&mut $ty) -> T
            ) -> T {
                let xc = termx. [< $Termx:snake >] ().components();
                let res = f(&mut entity.get_mut(xc.$component, world).unwrap().$name);
                let c = termx.termx().components();
                let owner = entity.get(c.view_layout, world).unwrap().owner();
                if let Some(owner) = owner {
                    let owner_parent = owner.get(c.view, world).unwrap().visual_parent();
                    if let Some(owner_parent) = owner_parent {
                        let s = termx.termx().systems();
                        $crate::systems::layout::LayoutExt::invalidate_arrange(
                            &s.layout, owner_parent, world
                        );
                    }
                }
                res
            }

            pub fn [< set_ $name >] (
                entity: $crate::ooecs_Entity<$crate::termx::Termx>,
                world: &mut $crate::ooecs_World<$crate::termx::Termx>,
                termx: &$crate::alloc_rc_Rc<dyn [< Is $Termx >] >,
                value: $ty
            ) {
                let xc = termx. [< $Termx:snake >] ().components();
                entity.get_mut(xc.$component, world).unwrap().$name = value;
                let c = termx.termx().components();
                let owner = entity.get(c.view_layout, world).unwrap().owner();
                if let Some(owner) = owner {
                    let owner_parent = owner.get(c.view, world).unwrap().visual_parent();
                    if let Some(owner_parent) = owner_parent {
                        let s = termx.termx().systems();
                        $crate::systems::layout::LayoutExt::invalidate_arrange(
                            &s.layout, owner_parent, world
                        );
                    }
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use crate::components::background::{Background, BackgroundTemplate};
    use crate::components::layout_view::LayoutView;
    use crate::systems::layout::LayoutExt;
    use crate::template::Template;
    use crate::termx::Termx;
    use int_vec_2d::Vector;
    use ooecs::World;

    #[test]
    fn create_view_perform_layout_change_property() {
        let world = &mut World::new();
        let termx = Termx::new(world);
        let bg = Background::new_entity(world, &termx);
        termx.termx().systems().layout.perform(bg, world, Vector { x: 80, y: 25 });
        Background::set_pattern(bg, world, &termx, "x".to_string());
        assert_eq!(&Background::get_pattern(bg, world, &termx, |x| x.to_string()), "x");
    }

    #[test]
    fn create_background_with_template() {
        let world = &mut World::new();
        let termx = Termx::new(world);
        let (bg, _) = BackgroundTemplate {
            pattern: Some("x".to_string()),
            width: Some(Some(20)),
            .. Default::default()
        }.load_content(world, &termx);
        assert_eq!(&Background::get_pattern(bg, world, &termx, |x| x.to_string()), "x");
        assert_eq!(LayoutView::get_width(bg, world, &termx), Some(20));
    }
}
