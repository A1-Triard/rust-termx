use alloc::rc::Rc;
use alloc::string::String;
use crate::property;
use crate::base::{Fg, Bg, ViewHAlign};
use crate::components::border::Border;
use crate::components::background::Background;
use crate::components::content_presenter::ContentPresenter;
use crate::components::layout_view::*;
use crate::components::view::*;
use crate::components::focus_scope::FocusScope;
use crate::components::input_element::*;
use crate::components::control::Control;
use crate::systems::init::InitExt;
use crate::template::{Template, NameResolver};
use crate::termx::{IsTermx, Termx};
use ooecs::{Entity, World};

pub struct GroupBox {
    double: bool,
    color: (Fg, Bg),
    header_align: ViewHAlign,
    content: Option<Entity<Termx>>,
    text: Rc<String>,
    text_color: (Fg, Bg),
    header: Option<Entity<Termx>>,
    header_text: Rc<String>,
    pub(crate) part_border: Option<Entity<Termx>>,
    pub(crate) part_header_background: Option<Entity<Termx>>,
    pub(crate) part_content_presenter: Option<Entity<Termx>>,
    pub(crate) part_header_presenter: Option<Entity<Termx>>,
}

impl GroupBox {
    pub fn new() -> Self {
        GroupBox {
            double: false,
            color: (Fg::LightGray, Bg::None),
            header_align: ViewHAlign::Left,
            content: None,
            text: Rc::new(String::new()),
            text_color: (Fg::LightGray, Bg::None),
            header: None,
            header_text: Rc::new(String::new()),
            part_border: None,
            part_header_background: None,
            part_content_presenter: None,
            part_header_presenter: None,
        }
    }

    pub fn new_entity(world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        let c = termx.termx().components();
        let e = Entity::new(c.group_box, world);
        e.add(c.view, world, View::new(TREE_CONTROL, RENDER_NONE));
        e.add(c.layout_view, world, LayoutView::new(LAYOUT_CONTROL));
        e.add(c.focus_scope, world, FocusScope::new());
        e.add(c.input_element, world, InputElement::new(INPUT_NONE));
        e.add(c.control, world, Control::new());
        e.add(c.group_box, world, GroupBox::new());
        termx.termx().systems().init.init_group_box(e, world);
        e
    }

    property!(Termx, group_box, double, bool, update_double);

    fn update_double(entity: Entity<Termx>, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) {
        let c = termx.termx().components();
        let group_box = entity.get(c.group_box, world).unwrap();
        let value = group_box.double;
        let part_border = group_box.part_border;
        if let Some(part_border) = part_border {
            Border::set_double(part_border, world, termx, value);
        }
    }

    property!(Termx, group_box, color, (Fg, Bg), update_color);

    fn update_color(entity: Entity<Termx>, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) {
        let c = termx.termx().components();
        let group_box = entity.get(c.group_box, world).unwrap();
        let value = group_box.color;
        let part_border = group_box.part_border;
        let part_header_background = group_box.part_header_background;
        let part_header_presenter = group_box.part_header_presenter;
        if let Some(part_border) = part_border {
            Border::set_color(part_border, world, termx, value);
        }
        if let Some(part_header_background) = part_header_background {
            Background::set_color(part_header_background, world, termx, value);
        }
        if let Some(part_header_presenter) = part_header_presenter {
            ContentPresenter::set_text_color(part_header_presenter, world, termx, value);
        }
    }

    property!(Termx, group_box, header_align, ViewHAlign, update_header_align);

    fn update_header_align(entity: Entity<Termx>, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) {
        let c = termx.termx().components();
        let group_box = entity.get(c.group_box, world).unwrap();
        let value = group_box.header_align;
        let part_header_background = group_box.part_header_background;
        if let Some(part_header_background) = part_header_background {
            LayoutView::set_h_align(part_header_background, world, termx, value);
        }
    }

    property!(Termx, group_box, text, ref Rc<String>, update_text);

    fn update_text(entity: Entity<Termx>, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) {
        let c = termx.termx().components();
        let group_box = entity.get(c.group_box, world).unwrap();
        let value = group_box.text.clone();
        let part_content_presenter = group_box.part_content_presenter;
        if let Some(part_content_presenter) = part_content_presenter {
            ContentPresenter::set_text(part_content_presenter, world, termx, value);
        }
    }

    property!(Termx, group_box, text_color, (Fg, Bg), update_text_color);

    fn update_text_color(entity: Entity<Termx>, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) {
        let c = termx.termx().components();
        let group_box = entity.get(c.group_box, world).unwrap();
        let value = group_box.text_color;
        let part_content_presenter = group_box.part_content_presenter;
        if let Some(part_content_presenter) = part_content_presenter {
            ContentPresenter::set_text_color(part_content_presenter, world, termx, value);
        }
    }

    property!(Termx, group_box, content, Option<Entity<Termx>>, update_content);

    fn update_content(entity: Entity<Termx>, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) {
        let c = termx.termx().components();
        let group_box = entity.get(c.group_box, world).unwrap();
        let value = group_box.content;
        let part_content_presenter = group_box.part_content_presenter;
        if let Some(part_content_presenter) = part_content_presenter {
            ContentPresenter::set_content(part_content_presenter, world, termx, value);
        }
    }

    property!(Termx, group_box, header, Option<Entity<Termx>>, update_header);

    fn update_header(entity: Entity<Termx>, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) {
        let c = termx.termx().components();
        let group_box = entity.get(c.group_box, world).unwrap();
        let value = group_box.header;
        let part_header_presenter = group_box.part_header_presenter;
        if let Some(part_header_presenter) = part_header_presenter {
            ContentPresenter::set_content(part_header_presenter, world, termx, value);
        }
    }

    property!(Termx, group_box, header_text, ref Rc<String>, update_header_text);

    fn update_header_text(entity: Entity<Termx>, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) {
        let c = termx.termx().components();
        let group_box = entity.get(c.group_box, world).unwrap();
        let value = group_box.header_text.clone();
        let part_header_presenter = group_box.part_header_presenter;
        if let Some(part_header_presenter) = part_header_presenter {
            ContentPresenter::set_text(part_header_presenter, world, termx, value);
        }
    }
}

#[macro_export]
macro_rules! group_box_template {
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
        $crate::control_template! {
            $(#[$attr])*
            $vis struct $name in $mod {
                use $crate::base::serialize_color as components_group_box_serialize_color;
                use $crate::base::deserialize_color as components_group_box_deserialize_color;
                $(use $path as $import;)*

                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub double: Option<bool>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_group_box_serialize_color")]
                #[serde(deserialize_with="components_group_box_deserialize_color")]
                pub color: Option<($crate::base::Fg, $crate::base::Bg)>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub header_align: Option<$crate::base::ViewHAlign>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub content: Option<$crate::alloc_boxed_Box<dyn $crate::template::Template>>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub text: Option<$crate::alloc_string_String>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_group_box_serialize_color")]
                #[serde(deserialize_with="components_group_box_deserialize_color")]
                pub text_color: Option<($crate::base::Fg, $crate::base::Bg)>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub header: Option<$crate::alloc_boxed_Box<dyn $crate::template::Template>>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub header_text: Option<$crate::alloc_string_String>,
                $($(
                    $(#[$field_attr])*
                    pub $field_name : $field_ty
                ),+)?
            }
        }
    };
}

#[macro_export]
macro_rules! group_box_apply_template {
    ($this:ident, $entity:ident, $world:expr, $termx:expr, $names:ident) => {
        $crate::control_apply_template! { $this, $entity, $world, $termx, $names }
        $this.double.map(|x|
            $crate::components::group_box::GroupBox::set_double($entity, $world, $termx, x)
        );
        $this.color.map(|x|
            $crate::components::group_box::GroupBox::set_color($entity, $world, $termx, x)
        );
        $this.header_align.map(|x|
            $crate::components::group_box::GroupBox::set_header_align($entity, $world, $termx, x)
        );
        $this.content.as_ref().map(|x| {
            let value = x.begin_load_content_inline($world, $termx, $names);
            $crate::components::group_box::GroupBox::set_content(
                $entity, $world, $termx, Some(value)
            );
            x.end_load_content_inline(value, $world, $termx, $names);
        });
        $this.text.as_ref().map(|x|
            $crate::components::group_box::GroupBox::set_text(
                $entity, $world, $termx, $crate::alloc_rc_Rc::new(x.clone())
            )
        );
        $this.text_color.map(|x|
            $crate::components::group_box::GroupBox::set_text_color($entity, $world, $termx, x)
        );
        $this.header.as_ref().map(|x| {
            let value = x.begin_load_content_inline($world, $termx, $names);
            $crate::components::group_box::GroupBox::set_header(
                $entity, $world, $termx, Some(value)
            );
            x.end_load_content_inline(value, $world, $termx, $names);
        });
        $this.header_text.as_ref().map(|x|
            $crate::components::group_box::GroupBox::set_header_text(
                $entity, $world, $termx, $crate::alloc_rc_Rc::new(x.clone())
            )
        );
    };
}

group_box_template! {
    #[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
    #[serde(rename="GroupBox@Content")]
    pub struct GroupBoxTemplate in template { }
}

#[typetag::serde(name="GroupBox")]
impl Template for GroupBoxTemplate {
    fn name(&self) -> Option<&String> {
        Some(&self.name)
    }

    fn create_entity(&self, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        GroupBox::new_entity(world, termx)
    }

    fn apply_resources<'a>(
        &self,
        entity: Entity<Termx>,
        world: &'a mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
    ) -> Option<&'a Rc<dyn Template>> {
        View::apply_resources(
            self.resources.clone(), entity, world, termx, &self.style_key, "IMPLICIT_GroupBox"
        )
    }

    fn apply(
        &self,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        names: &mut NameResolver,
    ) {
        let this = self;
        group_box_apply_template! { this, entity, world, termx, names }
    }
}
