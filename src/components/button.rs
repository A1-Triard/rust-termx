use alloc::string::String;
use alloc::rc::Rc;
use alloc::boxed::Box;
use crate::base::{Fg, Bg};
use crate::components::view::*;
use crate::components::layout_view::*;
use crate::components::focus_scope::FocusScope;
use crate::components::input_element::*;
use crate::event_handler::EventHandler;
use crate::property;
use crate::systems::input::Timer;
use crate::template::{Template, NameResolver};
use crate::termx::{IsTermx, Termx};
use ooecs::{Entity, World};

pub struct Button {
    text: Rc<String>,
    color: (Fg, Bg),
    color_hotkey: (Fg, Bg),
    color_focused: (Fg, Bg),
    color_focused_hotkey: (Fg, Bg),
    color_disabled: (Fg, Bg),
    pub(crate) pressed: Option<Timer>,
    pub(crate) is_mouse_pressed: bool,
    pub(crate) click_handler: EventHandler<Option<Box<dyn FnMut(&mut World<Termx>)>>>,
}

impl Button {
    pub fn new() -> Self {
        Button {
            text: Rc::new(String::new()),
            color: (Fg::Black, Bg::LightGray),
            color_hotkey: (Fg::Blue, Bg::LightGray),
            color_focused: (Fg::Black, Bg::Cyan),
            color_focused_hotkey: (Fg::Cyan, Bg::Blue),
            color_disabled: (Fg::DarkGray, Bg::LightGray),
            pressed: None,
            is_mouse_pressed: false,
            click_handler: Default::default(),
        }
    }

    pub fn new_entity(world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        let c = termx.termx().components();
        let b = Entity::new(c.button, world);
        b.add(c.view, world, View::new(TREE_NONE, RENDER_BUTTON));
        b.add(c.layout_view, world, LayoutView::new(LAYOUT_BUTTON));
        b.add(c.focus_scope, world, FocusScope::new());
        b.add(c.input_element, world, InputElement::new(INPUT_BUTTON));
        b.add(c.button, world, Button::new());
        b
    }

    pub fn is_pressed(&self) -> bool { self.pressed.is_some() || self.is_mouse_pressed }

    property!(Termx, button, text, ref Rc<String>, @measure);
    property!(Termx, button, color, (Fg, Bg), @render);
    property!(Termx, button, color_hotkey, (Fg, Bg), @render);
    property!(Termx, button, color_focused, (Fg, Bg), @render);
    property!(Termx, button, color_focused_hotkey, (Fg, Bg), @render);
    property!(Termx, button, color_disabled, (Fg, Bg), @render);

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
                use $crate::base::serialize_color as components_button_serialize_color;
                use $crate::base::deserialize_color as components_button_deserialize_color;
                $(use $path as $import;)*

                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub text: Option<$crate::alloc_string_String>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_button_serialize_color")]
                #[serde(deserialize_with="components_button_deserialize_color")]
                pub color: Option<($crate::base::Fg, $crate::base::Bg)>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_button_serialize_color")]
                #[serde(deserialize_with="components_button_deserialize_color")]
                pub color_hotkey: Option<($crate::base::Fg, $crate::base::Bg)>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_button_serialize_color")]
                #[serde(deserialize_with="components_button_deserialize_color")]
                pub color_focused: Option<($crate::base::Fg, $crate::base::Bg)>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_button_serialize_color")]
                #[serde(deserialize_with="components_button_deserialize_color")]
                pub color_focused_hotkey: Option<($crate::base::Fg, $crate::base::Bg)>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_button_serialize_color")]
                #[serde(deserialize_with="components_button_deserialize_color")]
                pub color_disabled: Option<($crate::base::Fg, $crate::base::Bg)>,
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
        $this.color.map(|x| $crate::components::button::Button::set_color($entity, $world, $termx, x));
        $this.color_hotkey.map(|x|
            $crate::components::button::Button::set_color_hotkey($entity, $world, $termx, x)
        );
        $this.color_focused.map(
            |x| $crate::components::button::Button::set_color_focused($entity, $world, $termx, x)
        );
        $this.color_focused_hotkey.map(
            |x| $crate::components::button::Button::set_color_focused_hotkey($entity, $world, $termx, x)
        );
        $this.color_disabled.map(
            |x| $crate::components::button::Button::set_color_disabled($entity, $world, $termx, x)
        );
    };
}

button_template! {
    #[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
    #[serde(rename="Button@Text")]
    pub struct ButtonTemplate in template { }
}

#[typetag::serde(name="Button")]
impl Template for ButtonTemplate {
    fn name(&self) -> Option<&String> {
        Some(&self.name)
    }

    fn create_entity(&self, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        Button::new_entity(world, termx)
    }

    fn apply(
        &self,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        names: &mut NameResolver,
    ) {
        let this = self;
        button_apply_template! { this, entity, world, termx, names }
    }
}
