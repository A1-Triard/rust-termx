use alloc::rc::Rc;
use alloc::vec::Vec;
use crate::property_ro;
use crate::systems::layout::LayoutExt;
use crate::systems::render::RenderExt;
use crate::termx::{Termx, IsTermx};
use ooecs::{Entity, World};

pub struct Panel {
    children: Vec<Entity<Termx>>,
}

impl Panel {
    pub fn new() -> Self {
        Panel { children: Vec::new() }
    }

    property_ro!(Termx, panel, children, ref as &[Entity<Termx>]);

    pub fn get_children_mut<T>(
        entity: Entity<Termx>,
        world: &mut World<Termx>, 
        termx: &Rc<dyn IsTermx>,
        f: impl FnOnce(&mut Vec<Entity<Termx>>) -> T,
    ) -> T {
        let termx = termx.termx();
        let component = termx.components().panel;
        let elements = entity.get(component, world).unwrap().children.clone();
        for element in elements {
            termx.systems().render.remove_visual_child(entity, element, world);
        }
        let res = f(&mut entity.get_mut(component, world).unwrap().children);
        let elements = entity.get(component, world).unwrap().children.clone();
        for element in elements {
            termx.systems().render.add_visual_child(entity, element, world);
        }
        termx.systems().layout.invalidate_measure(entity, world);
        res
    }

    pub fn set_children(
        entity: Entity<Termx>,
        world: &mut World<Termx>, 
        termx: &Rc<dyn IsTermx>,
        value: &Vec<Entity<Termx>>,
    ) {
        let termx = termx.termx();
        let component = termx.components().panel;
        let elements = entity.get(component, world).unwrap().children.clone();
        for element in elements {
            termx.systems().render.remove_visual_child(entity, element, world);
        }
        entity.get_mut(component, world).unwrap().children = value.clone();
        for &element in value {
            termx.systems().render.add_visual_child(entity, element, world);
        }
        termx.systems().layout.invalidate_measure(entity, world);
    }
}

#[doc(hidden)]
pub fn vec_is_empty<T>(x: &Vec<T>) -> bool {
    x.is_empty()
}

#[macro_export]
macro_rules! panel_template {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident in $mod:ident {
            $(use $path:path as $import:ident;)*
            $($(
                $(#[$field_attr:meta])*
                pub $field_name:ident : $field_ty:ty
            ),+ $(,)?)?
        }
    ) => {
        $crate::focus_scope_template! {
            $(#[$attr])*
            $vis struct $name in $mod {
                use $crate::components::panel::vec_is_empty as components_panel_vec_is_empty;
                $(use $path as $import;)*

                #[serde(default)]
                #[serde(skip_serializing_if="components_panel_vec_is_empty")]
                pub children: $crate::alloc_vec_Vec<$crate::alloc_boxed_Box<dyn $crate::template::Template>>,
                $($(
                    $(#[$field_attr])*
                    pub $field_name : $field_ty
                ),+)?
            }
        }
    };
}

#[macro_export]
macro_rules! panel_apply_template {
    ($this:ident, $entity:ident, $world:expr, $termx:expr, $names:ident) => {
        $crate::focus_scope_apply_template! { $this, $entity, $world, $termx, $names }
        let children = $this.children.iter().map(|x| x.load_content_inline($world, $termx, $names)).collect();
        $crate::components::panel::Panel::set_children($entity, $world, $termx, &children);
    };
}
