use alloc::string::String;
use crate::property_rw;
use crate::termx::{Termx, IsTermx};
use int_vec_2d::{Rect, Point, Vector};
use ooecs::Entity;

pub struct View {
    pub(crate) visual_parent: Option<Entity<Termx>>,
    pub(crate) real_render_bounds: Rect,
    tree: u16,
    render: u16,
    pub name: String,
}

pub const TREE_NONE: u16 = 0;
pub const TREE_DECORATOR: u16 = 1;

pub const RENDER_NONE: u16 = 0;
pub const RENDER_BACKGROUND: u16 = 1;

impl View {
    pub fn new(tree: u16, render: u16) -> Self {
        View {
            visual_parent: None,
            real_render_bounds: Rect { tl: Point { x: 0, y: 0 }, size: Vector::null() },
            tree,
            render,
            name: String::new(),
        }
    }

    pub fn visual_parent(&self) -> Option<Entity<Termx>> {
        self.visual_parent
    }

    pub fn tree(&self) -> u16 {
        self.tree
    }

    pub fn render(&self) -> u16 {
        self.render
    }

    property_rw!(Termx, view, name, ref String as &str);
}

#[doc(hidden)]
pub fn string_is_empty(x: &String) -> bool {
    x.is_empty()
}

#[macro_export]
macro_rules! view_template {
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
                use $crate::components::view::string_is_empty as components_view_string_is_empty;
                $(use $path as $import;)*

                #[serde(default)]
                #[serde(skip_serializing_if="components_view_string_is_empty")]
                pub name: $crate::alloc_string_String,
                $($(
                    $(#[$field_attr])*
                    pub $field_name : $field_ty
                ),+)?
            }
        }
    };
}

#[macro_export]
macro_rules! view_apply_template {
    ($this:ident, $entity:ident, $termx:expr, $names:ident) => {
        $crate::components::view::View::set_name($entity, $termx, $this.name.clone());
    };
}
