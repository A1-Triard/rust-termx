use crate::{property_ro, property_rw};
use crate::termx::IsTermx;

pub struct InputElement {
    pub focusable: bool,
    pub(crate) is_focused: bool,
}

impl InputElement {
    pub fn new() -> Self {
        InputElement {
            focusable: true,
            is_focused: false,
        }
    }

    property_rw!(Termx, input_element, focusable, bool);
    property_ro!(Termx, input_element, is_focused, bool);
}

#[macro_export]
macro_rules! input_element_template {
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
                pub focusable: Option<bool>,
                $($(
                    $(#[$field_attr])*
                    pub $field_name : $field_ty
                ),+)?
            }
        }
    };
}

#[macro_export]
macro_rules! input_element_apply_template {
    ($this:ident, $entity:ident, $termx:expr, $names:ident) => {
        $crate::focus_scope_apply_template! { $this, $entity, $termx, $names }
        $this.focusable.map(|x|
            $crate::components::input_element::InputElement::set_focusable($entity, $termx, x)
        );
    };
}
