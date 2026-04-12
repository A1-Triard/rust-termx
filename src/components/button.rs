use alloc::boxed::Box;
use alloc::string::String;
use alloc::rc::Rc;
use crate::event_handler::EventHandler;
use crate::property;
use crate::systems::input::Timer;
use crate::termx::{IsTermx, Termx};
use ooecs::{Entity, World};

pub struct Button {
    text: Rc<String>,
    pub(crate) pressed: Option<Timer>,
    pub(crate) is_mouse_pressed: bool,
    pub(crate) click_handler: EventHandler<Option<Box<dyn FnMut(&mut World<Termx>)>>>,
}

impl Button {
    pub fn new() -> Self {
        Button {
            text: Rc::new(String::new()),
            pressed: None,
            is_mouse_pressed: false,
            click_handler: Default::default(),
        }
    }

    pub fn is_pressed(&self) -> bool { self.pressed.is_some() || self.is_mouse_pressed }

    property!(Termx, button, text, ref Rc<String>, @measure);

    pub fn on_click(
        entity: Entity<Termx>,
        world: &mut World<Termx>, 
        termx: &Rc<dyn IsTermx>,
        handler: Option<Box<dyn FnMut(&mut World<Termx>)>>,
    ) {
        let termx = termx.termx();
        let c = termx.components();
        entity.get_mut(c.button, world).unwrap().click_handler.set(handler);
    }
}

#[macro_export]
macro_rules! button_template {
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
                pub text: Option<$crate::alloc_string_String>,
                $($(
                    $(#[$field_attr])*
                    pub $field_name : $field_ty
                ),+)?
            }
        }
    };
}

#[macro_export]
macro_rules! button_apply_template {
    ($this:ident, $entity:ident, $world:expr, $termx:expr, $names:ident) => {
        $crate::input_element_apply_template! { $this, $entity, $world, $termx, $names }
        $this.text.as_ref().map(|x|
            $crate::components::button::Button::set_text(
                $entity,
                $world,
                $termx,
                $crate::alloc_rc_Rc::new(x.clone())
            )
        );
    };
}
