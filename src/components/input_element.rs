use crate::{property_ro, property_rw};
use crate::termx::IsTermx;

pub struct InputElement {
    input: u16,
    pub focusable: bool,
    pub(crate) is_focused: bool,
}

pub const INPUT_NONE: u16 = 0;
pub const INPUT_BUTTON: u16 = 1;
pub const INPUT_INPUT_LINE: u16 = 2;

impl InputElement {
    pub fn new(input: u16) -> Self {
        InputElement {
            input,
            focusable: true,
            is_focused: false,
        }
    }

    pub fn input(&self) -> u16 {
        self.input
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
    ($this:ident, $entity:ident, $world:expr, $termx:expr, $names:ident) => {
        $crate::focus_scope_apply_template! { $this, $entity, $world, $termx, $names }
        $this.focusable.map(|x|
            $crate::components::input_element::InputElement::set_focusable($entity, $world, $termx, x)
        );
    };
}
