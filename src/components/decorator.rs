use alloc::rc::Rc;
use crate::property_ro;
use crate::systems::layout::LayoutExt;
use crate::systems::render::RenderExt;
use crate::termx::{Termx, IsTermx};
use ooecs::Entity;

pub struct Decorator {
    child: Option<Entity<Termx>>,
}

impl Decorator {
    pub fn new() -> Self {
        Decorator { child: None }
    }

    property_ro!(Termx, decorator, child, Option<Entity<Termx>>);

    pub fn set_child(entity: Entity<Termx>, termx: &Rc<dyn IsTermx>, value: Option<Entity<Termx>>) {
        let termx = termx.termx();
        let component = termx.components().decorator;
        let mut world = termx.world.borrow_mut();
        let old_child = entity.get(component, &mut world).unwrap().child;
        if let Some(child) = old_child {
            termx.systems().render.remove_visual_child(entity, child, &mut world);
        }
        entity.get_mut(component, &mut world).unwrap().child = value;
        if let Some(child) = value {
            termx.systems().render.add_visual_child(entity, child, &mut world);
        }
        termx.systems().layout.invalidate_measure(entity, &mut world);
    }
}

#[macro_export]
macro_rules! decorator_template {
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
        $crate::layout_view_template! {
            $(#[$attr])*
            $vis struct $name in $mod {
                $(use $path as $import;)*

                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub child: Option<$crate::alloc_boxed_Box<dyn $crate::template::Template>>,
                $($(
                    $(#[$field_attr])*
                    pub $field_name : $field_ty
                ),+)?
            }
        }
    };
}

#[macro_export]
macro_rules! decorator_apply_template {
    ($this:ident, $entity:ident, $termx:expr, $names:ident) => {
        $crate::layout_view_apply_template! { $this, $entity, $termx, $names }
        $this.child.as_ref().map(|x| $crate::components::decorator::Decorator::set_child(
            $entity,
            $termx,
            Some(x.load_content_inline($termx, $names))
        ));
    };
}
