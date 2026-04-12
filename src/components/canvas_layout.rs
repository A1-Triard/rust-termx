use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use crate::components::view_layout::ViewLayout;
use crate::layout_property;
use crate::template::{Template, NameResolver};
use crate::termx::{Termx, IsTermx};
use int_vec_2d::Point;
use ooecs::{Entity, World};

pub struct CanvasLayout {
    tl: Point,
}

impl CanvasLayout {
    pub fn new() -> Self {
        CanvasLayout {
            tl: Point { x: 0, y: 0 },
        }
    }

    pub fn new_entity(world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        let termx = termx.termx();
        let view_layout = termx.components().view_layout;
        let canvas_layout = termx.components().canvas_layout;
        let cl = Entity::new(canvas_layout, world);
        cl.add(view_layout, world, ViewLayout::new());
        cl.add(canvas_layout, world, CanvasLayout::new());
        cl
    }

    layout_property!(Termx, canvas_layout, tl, Point, @arrange);
}

#[macro_export]
macro_rules! canvas_layout_template {
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
        $crate::view_layout_template! {
            $(#[$attr])*
            $vis struct $name in $mod {
                $(use $path as $import;)*

                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub tl: Option<$crate::base::Point>,
                $($(
                    $(#[$field_attr])*
                    pub $field_name : $field_ty
                ),+)?
            }
        }
    };
}

#[macro_export]
macro_rules! canvas_layout_apply_template {
    ($this:ident, $entity:ident, $world:expr, $termx:expr, $names:ident) => {
        $crate::view_layout_apply_template! { $this, $entity, $world, $termx, $names }
        $this.tl.map(|x| $crate::components::canvas_layout::CanvasLayout::set_tl($entity, $world, $termx, x));
    };
}

canvas_layout_template! {
    #[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
    #[serde(rename="CanvasLayout")]
    pub struct CanvasLayoutTemplate in template { }
}

#[typetag::serde(name="CanvasLayout")]
impl Template for CanvasLayoutTemplate {
    fn name(&self) -> Option<&String> {
        None
    }

    fn create_entity(&self, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        CanvasLayout::new_entity(world, termx)
    }

    fn apply_resources<'a>(
        &self,
        _entity: Entity<Termx>,
        _world: &'a mut World<Termx>,
        _termx: &Rc<dyn IsTermx>,
    ) -> Option<&'a Box<dyn Template>> {
        None
    }

    fn apply(
        &self,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        names: &mut NameResolver,
    ) {
        let this = self;
        canvas_layout_apply_template! { this, entity, world, termx, names }
    }
}
