use alloc::string::String;
use alloc::rc::Rc;
use alloc::boxed::Box;
use crate::base::{Fg, Bg};
use crate::components::view::*;
use crate::components::layout_view::*;
use crate::components::focus_scope::FocusScope;
use crate::components::input_element::*;
use crate::event_handler::EventHandler;
use crate::systems::render::RenderExt;
use int_vec_2d::Thickness;
use crate::property;
use crate::systems::input::Timer;
use crate::template::{Template, NameResolver};
use crate::termx::{IsTermx, Termx};
use ooecs::{Entity, World};

pub struct TButton {
    text: String,
    color: (Fg, Bg),
    color_hotkey: (Fg, Bg),
    color_focused: (Fg, Bg),
    color_focused_hotkey: (Fg, Bg),
    color_disabled: (Fg, Bg),
    pub(crate) pressed: Option<Timer>,
    pub(crate) is_mouse_pressed: bool,
    pub(crate) click_handler: EventHandler<Option<Box<dyn FnMut(&mut World<Termx>)>>>,
}

impl TButton {
    pub fn new() -> Self {
        TButton {
            text: String::new(),
            color: (Fg::Black, Bg::Green),
            color_hotkey: (Fg::Yellow, Bg::Green),
            color_focused: (Fg::White, Bg::Green),
            color_focused_hotkey: (Fg::Yellow, Bg::Green),
            color_disabled: (Fg::DarkGray, Bg::Green),
            pressed: None,
            is_mouse_pressed: false,
            click_handler: Default::default(),
        }
    }

    pub fn init(entity: Entity<Termx>, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) {
        termx.termx().systems().render.set_shadow(entity, world, Thickness::new(0, 0, 1, 1));
    }

    pub fn new_entity(world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        let termx_ = termx.termx();
        let view = termx_.components().view;
        let layout_view = termx_.components().layout_view;
        let focus_scope = termx_.components().focus_scope;
        let input_element = termx_.components().input_element;
        let t_button = termx_.components().t_button;
        let b = Entity::new(t_button, world);
        b.add(view, world, View::new(TREE_NONE, RENDER_T_BUTTON));
        b.add(layout_view, world, LayoutView::new(LAYOUT_T_BUTTON));
        b.add(focus_scope, world, FocusScope::new());
        b.add(input_element, world, InputElement::new(INPUT_T_BUTTON));
        b.add(t_button, world, TButton::new());
        Self::init(b, world, termx);
        b
    }

    property!(Termx, t_button, text, ref String as &str, @measure);
    property!(Termx, t_button, color, (Fg, Bg), @render);
    property!(Termx, t_button, color_hotkey, (Fg, Bg), @render);
    property!(Termx, t_button, color_focused, (Fg, Bg), @render);
    property!(Termx, t_button, color_focused_hotkey, (Fg, Bg), @render);
    property!(Termx, t_button, color_disabled, (Fg, Bg), @render);

    pub fn on_click(
        entity: Entity<Termx>,
        world: &mut World<Termx>, 
        termx: &Rc<dyn IsTermx>,
        handler: Option<Box<dyn FnMut(&mut World<Termx>)>>,
    ) {
        let termx = termx.termx();
        let c = termx.components();
        entity.get_mut(c.t_button, world).unwrap().click_handler.set(handler);
    }
}

#[macro_export]
macro_rules! t_button_template {
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
                use $crate::base::serialize_color as components_t_button_serialize_color;
                use $crate::base::deserialize_color as components_t_button_deserialize_color;
                $(use $path as $import;)*

                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub text: Option<$crate::alloc_string_String>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_t_button_serialize_color")]
                #[serde(deserialize_with="components_t_button_deserialize_color")]
                pub color: Option<($crate::base::Fg, $crate::base::Bg)>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_t_button_serialize_color")]
                #[serde(deserialize_with="components_t_button_deserialize_color")]
                pub color_hotkey: Option<($crate::base::Fg, $crate::base::Bg)>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_t_button_serialize_color")]
                #[serde(deserialize_with="components_t_button_deserialize_color")]
                pub color_focused: Option<($crate::base::Fg, $crate::base::Bg)>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_t_button_serialize_color")]
                #[serde(deserialize_with="components_t_button_deserialize_color")]
                pub color_focused_hotkey: Option<($crate::base::Fg, $crate::base::Bg)>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_t_button_serialize_color")]
                #[serde(deserialize_with="components_t_button_deserialize_color")]
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
macro_rules! t_button_apply_template {
    ($this:ident, $entity:ident, $world:expr, $termx:expr, $names:ident) => {
        $crate::input_element_apply_template! { $this, $entity, $world, $termx, $names }
        $this.text.as_ref().map(|x|
            $crate::components::t_button::TButton::set_text($entity, $world, $termx, x.clone())
        );
        $this.color.map(|x| $crate::components::t_button::TButton::set_color($entity, $world, $termx, x));
        $this.color_hotkey.map(|x|
            $crate::components::t_button::TButton::set_color_hotkey($entity, $world, $termx, x)
        );
        $this.color_focused.map(
            |x| $crate::components::t_button::TButton::set_color_focused($entity, $world, $termx, x)
        );
        $this.color_focused_hotkey.map(
            |x| $crate::components::t_button::TButton::set_color_focused_hotkey($entity, $world, $termx, x)
        );
        $this.color_disabled.map(
            |x| $crate::components::t_button::TButton::set_color_disabled($entity, $world, $termx, x)
        );
    };
}

t_button_template! {
    #[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
    #[serde(rename="TButton@Text")]
    pub struct TButtonTemplate in template { }
}

#[typetag::serde(name="TButton")]
impl Template for TButtonTemplate {
    fn name(&self) -> Option<&String> {
        Some(&self.name)
    }

    fn create_entity(&self, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        TButton::new_entity(world, termx)
    }

    fn apply(
        &self,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        names: &mut NameResolver,
    ) {
        let this = self;
        t_button_apply_template! { this, entity, world, termx, names }
    }
}
