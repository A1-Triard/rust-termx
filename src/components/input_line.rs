use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use crate::event_handler::EventHandler;
use crate::line_edit::LineEdit;
use crate::systems::render::RenderExt;
use crate::termx::{IsTermx, Termx};
use ooecs::{Entity, World};

pub struct InputLine {
    pub line_edit: LineEdit,
    pub(crate) text_changed_handler: EventHandler<Option<Box<dyn FnMut(&mut World<Termx>)>>>,
}

impl InputLine {
    pub fn new() -> Self {
        InputLine {
            line_edit: LineEdit::new(),
            text_changed_handler: Default::default(),
        }
    }

    pub fn is_numeric(&self) -> bool { self.line_edit.is_numeric() }

    pub fn get_is_numeric(
        entity: Entity<Termx>,
        world: &World<Termx>,
        termx: &Rc<dyn IsTermx>,
    ) -> bool {
        let c = termx.termx().components();
        entity.get(c.input_line, world).unwrap().is_numeric()
    }

    pub fn set_is_numeric(
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        value: bool,
    ) {
        let c = termx.termx().components();
        let input_line = entity.get_mut(c.input_line, world).unwrap();
        if input_line.line_edit.set_is_numeric(value) {
            let s = termx.termx().systems();
            s.render.invalidate_render(entity, world);
        }
    }

    pub fn text(&self) -> &String { self.line_edit.text() }

    pub fn get_text<'a>(
        entity: Entity<Termx>,
        world: &'a World<Termx>,
        termx: &Rc<dyn IsTermx>,
    ) -> &'a String {
        let c = termx.termx().components();
        entity.get(c.input_line, world).unwrap().text()
    }

    pub fn get_text_mut<T>(
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        f: impl FnOnce(&mut String) -> T,
    ) -> T {
        let c = termx.termx().components();
        let (res, changed) = entity.get_mut(c.input_line, world).unwrap().line_edit.change_text(f);
        if changed {
            let s = termx.termx().systems();
            s.render.invalidate_render(entity, world);
            let mut handler = entity.get_mut(c.input_line, world).unwrap().text_changed_handler.begin_invoke();
            handler.as_mut().map(|f| f(world));
            entity.get_mut(c.input_line, world).unwrap().text_changed_handler.end_invoke(handler);
        }
        res
    }

    pub fn set_text(
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        value: String,
    ) {
        Self::get_text_mut(entity, world, termx, |s| *s = value);
    }

    pub fn on_text_change(
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        handler: Option<Box<dyn FnMut(&mut World<Termx>)>>,
    ) {
        let c = termx.termx().components();
        entity.get_mut(c.input_line, world).unwrap().text_changed_handler.set(handler);
    }
}

#[macro_export]
macro_rules! input_line_template {
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
        $crate::input_element_template! {
            $(#[$attr])*
            $vis struct $name in $mod {
                $(use $path as $import;)*

                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub is_numeric: Option<bool>,
                $($(
                    $(#[$field_attr])*
                    pub $field_name : $field_ty
                ),+)?
            }
        }
    };
}

#[macro_export]
macro_rules! input_line_apply_template {
    ($this:ident, $entity:ident, $world:expr, $termx:expr, $names:ident) => {
        $crate::input_element_apply_template! { $this, $entity, $world, $termx, $names }
        $this.is_numeric.map(|x|
            $crate::components::input_line::InputLine::set_is_numeric($entity, $world, $termx, x)
        );
    };
}
