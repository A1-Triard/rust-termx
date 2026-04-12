use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use crate::property_ro;
use crate::base::{Fg, Bg, TextWrapping};
use crate::components::layout_view::*;
use crate::components::focus_scope::FocusScope;
use crate::components::static_text::StaticText;
use crate::components::view::*;
use crate::systems::layout::LayoutExt;
use crate::systems::render::RenderExt;
use crate::template::{Template, NameResolver};
use crate::termx::{IsTermx, Termx};
use ooecs::{Entity, World};

pub struct ContentPresenter {
    content: Option<Entity<Termx>>,
    text: Rc<String>,
    text_color: (Fg, Bg),
    text_wrapping: TextWrapping,
    pub(crate) actual_child: Option<Entity<Termx>>,
}

impl ContentPresenter {
    pub fn new() -> Self {
        ContentPresenter {
            content: None,
            text: Rc::new(String::new()),
            text_color: (Fg::LightGray, Bg::None),
            text_wrapping: TextWrapping::NoWrap,
            actual_child: None,
        }
    }

    pub fn new_entity(world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        let termx = termx.termx();
        let c = termx.components();
        let e = Entity::new(c.content_presenter, world);
        e.add(c.view, world, View::new(TREE_CONTENT_PRESENTER, RENDER_NONE));
        e.add(c.layout_view, world, LayoutView::new(LAYOUT_CONTENT_PRESENTER));
        e.add(c.focus_scope, world, FocusScope::new());
        e.add(c.content_presenter, world, ContentPresenter::new());
        e
    }

    property_ro!(Termx, content_presenter, content, Option<Entity<Termx>>);

    pub fn set_content(
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        value: Option<Entity<Termx>>,
    ) {
        let c = termx.termx().components();
        entity.get_mut(c.content_presenter, world).unwrap().content = value;
        Self::update_actual_child(entity, world, termx);
    }

    property_ro!(Termx, content_presenter, text, ref Rc<String>);

    pub fn get_text_mut<T>(
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        f: impl FnOnce(&mut Rc<String>) -> T
    ) -> T {
        let c = termx.termx().components();
        let content_presenter = entity.get_mut(c.content_presenter, world).unwrap();
        let res = f(&mut content_presenter.text);
        if content_presenter.content.is_some() { return res; }
        if
               let Some(actual_child) = content_presenter.actual_child
            && !content_presenter.text.is_empty()
        {
            let text = content_presenter.text.clone();
            StaticText::set_text(actual_child, world, termx, text);
            return res;
        }
        Self::update_actual_child(entity, world, termx);
        res
    }

    pub fn set_text(
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        value: Rc<String>,
    ) {
        let c = termx.termx().components();
        let content_presenter = entity.get_mut(c.content_presenter, world).unwrap();
        content_presenter.text = value;
        if content_presenter.content.is_some() { return; }
        if
               let Some(actual_child) = content_presenter.actual_child
            && !content_presenter.text.is_empty()
        {
            let text = content_presenter.text.clone();
            StaticText::set_text(actual_child, world, termx, text);
            return;
        }
        Self::update_actual_child(entity, world, termx);
    }

    property_ro!(Termx, content_presenter, text_color, (Fg, Bg));

    pub fn set_text_color(
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        value: (Fg, Bg),
    ) {
        let c = termx.termx().components();
        let content_presenter = entity.get_mut(c.content_presenter, world).unwrap();
        content_presenter.text_color = value;
        if
               let Some(actual_child) = content_presenter.actual_child
            && content_presenter.content.is_none()
        {
            StaticText::set_color(actual_child, world, termx, value);
        }
    }

    property_ro!(Termx, content_presenter, text_wrapping, TextWrapping);

    pub fn set_text_wrapping(
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        value: TextWrapping,
    ) {
        let c = termx.termx().components();
        let content_presenter = entity.get_mut(c.content_presenter, world).unwrap();
        content_presenter.text_wrapping = value;
        if
               let Some(actual_child) = content_presenter.actual_child
            && content_presenter.content.is_none()
        {
            StaticText::set_text_wrapping(actual_child, world, termx, value);
        }
    }

    fn update_actual_child(entity: Entity<Termx>, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) {
        let c = termx.termx().components();
        let s = termx.termx().systems();
        if let Some(old_child) = entity.get(c.content_presenter, world).unwrap().actual_child {
            s.render.remove_visual_child(entity, old_child, world);
        }
        let content_presenter = entity.get(c.content_presenter, world).unwrap();
        let new_child = if let Some(content) = content_presenter.content {
            Some(content)
        } else if !content_presenter.text.is_empty() {
            let text = content_presenter.text.clone();
            let text_color = content_presenter.text_color;
            let text_wrapping = content_presenter.text_wrapping;
            let child = StaticText::new_entity(world, termx);
            StaticText::set_text(child, world, termx, text);
            StaticText::set_color(child, world, termx, text_color);
            StaticText::set_text_wrapping(child, world, termx, text_wrapping);
            Some(child)
        } else {
            None
        };
        entity.get_mut(c.content_presenter, world).unwrap().actual_child = new_child;
        if let Some(new_child) = new_child {
            s.render.add_visual_child(entity, new_child, world);
        }
        s.layout.invalidate_measure(entity, world);
    }
}

#[macro_export]
macro_rules! content_presenter_template {
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
                use $crate::base::serialize_color as components_content_presenter_serialize_color;
                use $crate::base::deserialize_color as components_content_presenter_deserialize_color;
                $(use $path as $import;)*

                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub content: Option<$crate::alloc_boxed_Box<dyn $crate::template::Template>>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub text: Option<$crate::alloc_string_String>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="components_content_presenter_serialize_color")]
                #[serde(deserialize_with="components_content_presenter_deserialize_color")]
                pub text_color: Option<($crate::base::Fg, $crate::base::Bg)>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub text_wrapping: Option<$crate::base::TextWrapping>
                $(,$(
                    $(#[$field_attr])*
                    pub $field_name : $field_ty
                ),+)?
            }
        }
    };
}

#[macro_export]
macro_rules! content_presenter_apply_template {
    ($this:ident, $entity:ident, $world:expr, $termx:expr, $names:ident) => {
        $crate::focus_scope_apply_template! { $this, $entity, $world, $termx, $names }
        $this.content.as_ref().map(|x| {
            let value = x.begin_load_content_inline($world, $termx, $names);
            $crate::components::content_presenter::ContentPresenter::set_content(
                $entity, $world, $termx, Some(value)
            );
            x.end_load_content_inline(value, $world, $termx, $names);
        });
        $this.text.as_ref().map(|x|
            $crate::components::content_presenter::ContentPresenter::set_text(
                $entity, $world, $termx, $crate::alloc_rc_Rc::new(x.clone())
            )
        );
        $this.text_color.map(|x|
            $crate::components::content_presenter::ContentPresenter::set_text_color($entity, $world, $termx, x)
        );
        $this.text_wrapping.map(|x|
            $crate::components::content_presenter::ContentPresenter::set_text_wrapping(
                $entity, $world, $termx, x
            )
        );
    };
}

content_presenter_template! {
    #[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
    #[serde(rename="ContentPresenter@Content")]
    pub struct ContentPresenterTemplate in template { }
}

#[typetag::serde(name="ContentPresenter")]
impl Template for ContentPresenterTemplate {
    fn name(&self) -> Option<&String> {
        Some(&self.name)
    }

    fn create_entity(&self, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        ContentPresenter::new_entity(world, termx)
    }

    fn apply_resources<'a>(
        &self,
        entity: Entity<Termx>,
        world: &'a mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
    ) -> Option<&'a Box<dyn Template>> {
        View::apply_resources(
            &self.resources, entity, world, termx, &self.style_key, "IMPLICIT_ContentPresenter"
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
        content_presenter_apply_template! { this, entity, world, termx, names }
    }
}
