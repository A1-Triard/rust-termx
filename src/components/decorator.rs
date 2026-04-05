use alloc::rc::Rc;
use crate::property_ro;
use crate::systems::layout::LayoutExt;
use crate::systems::render::RenderExt;
use crate::termx::{Termx, IsTermx};
use ooecs::{Entity, World};

pub struct Decorator {
    child: Option<Entity<Termx>>,
}

impl Decorator {
    pub fn new() -> Self {
        Decorator { child: None }
    }

    property_ro!(Termx, decorator, child, Option<Entity<Termx>>);

    pub fn set_child(
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        value: Option<Entity<Termx>>,
    ) {
        let c = termx.termx().components();
        let s = termx.termx().systems();
        let old_child = entity.get(c.decorator, world).unwrap().child;
        if let Some(child) = old_child {
            s.render.remove_visual_child(entity, child, world);
        }
        entity.get_mut(c.decorator, world).unwrap().child = value;
        if let Some(child) = value {
            s.render.add_visual_child(entity, child, world);
        }
        s.layout.invalidate_measure(entity, world);
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
        $crate::focus_scope_template! {
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
    ($this:ident, $entity:ident, $world:expr, $termx:expr, $names:ident) => {
        $crate::focus_scope_apply_template! { $this, $entity, $world, $termx, $names }
        $this.child.as_ref().map(|x| {
            let value = x.load_content_inline($world, $termx, $names);
            $crate::components::decorator::Decorator::set_child($entity, $world, $termx, Some(value));
        });
    };
}
