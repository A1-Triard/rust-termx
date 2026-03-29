use alloc::rc::Rc;
use alloc::string::String;
use crate::{property_rw, property_ro};
use crate::systems::layout::LayoutExt;
use crate::termx::{Termx, IsTermx};
use int_vec_2d::{Rect, Point, Vector};
use ooecs::Entity;

pub struct View {
    pub(crate) visual_parent: Option<Entity<Termx>>,
    pub(crate) real_render_bounds: Rect,
    tree: u16,
    render: u16,
    pub name: String,
    layout: Option<Entity<Termx>>,
}

pub const TREE_NONE: u16 = 0;
pub const TREE_DECORATOR: u16 = 1;
pub const TREE_PANEL: u16 = 2;

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
            layout: None,
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
    property_ro!(Termx, view, layout, Option<Entity<Termx>>);

    pub fn set_layout(entity: Entity<Termx>, termx: &Rc<dyn IsTermx>, value: Option<Entity<Termx>>) {
        let termx = termx.termx();
        let component = termx.components().view;
        let view_layout = termx.components().view_layout;
        let mut world = termx.world.borrow_mut();
        let old_layout = entity.get(component, &world).unwrap().layout;
        if let Some(old_layout) = old_layout {
            old_layout.get_mut(view_layout, &mut world).unwrap().owner = None;
        }
        let view = entity.get_mut(component, &mut world).unwrap();
        view.layout = value;
        let parent = view.visual_parent;
        if let Some(new_layout) = value {
            new_layout.get_mut(view_layout, &mut world).unwrap().owner = Some(entity);
        }
        if let Some(parent) = parent {
            termx.systems().layout.invalidate_measure(parent, &mut world);
        }
    }
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
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub layout: Option<$crate::alloc_boxed_Box<dyn $crate::template::Template>>,
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
        $this.layout.as_ref().map(|x| $crate::components::view::View::set_layout(
            $entity,
            $termx,
            Some(x.load_content_inline($termx, $names))
        ));
    };
}
