use crate::property_ro;
use crate::termx::{Termx, IsTermx};
use ooecs::Entity;

pub struct ViewLayout {
    pub(crate) owner: Option<Entity<Termx>>,
}

impl ViewLayout {
    pub fn new() -> Self {
        ViewLayout {
            owner: None,
        }
    }

    property_ro!(Termx, view_layout, owner, Option<Entity<Termx>>);
}

#[macro_export]
macro_rules! view_layout_template {
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
        $crate::template! {
            $(#[$attr])*
            $vis struct $name in $mod {
                $(use $path as $import;)*

                $($(
                    $(#[$field_attr])*
                    pub $field_name : $field_ty
                ),+)?
            }
        }
    };
}

#[macro_export]
macro_rules! view_layout_apply_template {
    ($this:ident, $entity:ident, $termx:expr, $names:ident) => {
        let _ = $this;
        let _ = $entity;
        let _ = $termx;
        let _ = $names;
    };
}
