use alloc::rc::Rc;
use alloc::string::String;
use core::mem::replace;
use crate::base::Visibility;
use crate::{property_rw, property_ro};
use crate::systems::layout::LayoutExt;
use crate::systems::render::RenderExt;
use crate::termx::{Termx, IsTermx};
use int_vec_2d::{Rect, Point, Vector, Thickness};
use ooecs::{Entity, World};

pub struct View {
    pub(crate) visual_parent: Option<Entity<Termx>>,
    pub(crate) real_render_bounds: Rect,
    pub(crate) real_render_bounds_with_shadow: Rect,
    tree: u16,
    render: u16,
    pub name: String,
    layout: Option<Entity<Termx>>,
    visibility: Visibility,
    pub(crate) visual_offset: Vector,
    pub(crate) shadow: Thickness,
}

pub const TREE_NONE: u16 = 0;
pub const TREE_DECORATOR: u16 = 1;
pub const TREE_PANEL: u16 = 2;
pub const TREE_CONTENT_PRESENTER: u16 = 3;
pub const TREE_CONTROL: u16 = 4;

pub const RENDER_NONE: u16 = 0;
pub const RENDER_BACKGROUND: u16 = 1;
pub const RENDER_T_BUTTON: u16 = 2;
pub const RENDER_STATIC_TEXT: u16 = 3;
pub const RENDER_BORDER: u16 = 4;
pub const RENDER_M_BUTTON: u16 = 5;
pub const RENDER_INPUT_LINE: u16 = 6;

impl View {
    pub fn new(tree: u16, render: u16) -> Self {
        View {
            visual_parent: None,
            real_render_bounds: Rect { tl: Point { x: 0, y: 0 }, size: Vector::null() },
            real_render_bounds_with_shadow: Rect { tl: Point { x: 0, y: 0 }, size: Vector::null() },
            tree,
            render,
            name: String::new(),
            layout: None,
            visibility: Visibility::Visible,
            visual_offset: Vector { x: 0, y: 0 },
            shadow: Thickness::all(0),
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

    property_rw!(Termx, view, name, ref String);
    property_ro!(Termx, view, layout, Option<Entity<Termx>>);

    pub fn set_layout(
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        value: Option<Entity<Termx>>
    ) {
        let c = termx.termx().components();
        let old_layout = entity.get(c.view, world).unwrap().layout;
        if let Some(old_layout) = old_layout {
            old_layout.get_mut(c.view_layout, world).unwrap().owner = None;
        }
        let view = entity.get_mut(c.view, world).unwrap();
        view.layout = value;
        let parent = view.visual_parent;
        if let Some(new_layout) = value {
            new_layout.get_mut(c.view_layout, world).unwrap().owner = Some(entity);
        }
        if let Some(parent) = parent {
            let s = termx.termx().systems();
            s.layout.invalidate_measure(parent, world);
        }
    }

    property_ro!(Termx, view, visibility, Visibility);

    pub fn set_visibility(
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        value: Visibility
    ) {
        let c = termx.termx().components();
        let view = entity.get_mut(c.view, world).unwrap();
        let old_visibility = replace(&mut view.visibility, value);
        let (invalidate_measure, invalidate_render) = match (old_visibility, value) {
            (Visibility::Visible, Visibility::Collapsed) => (true, false),
            (Visibility::Visible, Visibility::Hidden) => (false, true),
            (Visibility::Hidden, Visibility::Visible) => (false, true),
            (Visibility::Hidden, Visibility::Collapsed) => (true, false),
            (Visibility::Collapsed, Visibility::Visible) => (true, false),
            (Visibility::Collapsed, Visibility::Hidden) => (true, false),
            _ => (false, false),
        };
        let s = termx.termx().systems();
        if invalidate_measure {
            s.layout.invalidate_measure(entity, world);
        }
        if invalidate_render {
            s.render.invalidate_render(entity, world);
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
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub visibility: Option<$crate::base::Visibility>,
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
    ($this:ident, $entity:ident, $world:expr, $termx:expr, $names:ident) => {
        let _ = $names;
        $crate::components::view::View::set_name($entity, $world, $termx, $this.name.clone());
        $this.layout.as_ref().map(|x| {
            let value = x.load_content_inline($world, $termx, $names);
            $crate::components::view::View::set_layout($entity, $world, $termx, Some(value));
        });
        $this.visibility.map(|x| $crate::components::view::View::set_visibility($entity, $world, $termx, x));
    };
}
