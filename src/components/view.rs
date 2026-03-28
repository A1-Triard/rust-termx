use int_vec_2d::{Rect, Point, Vector};
use ooecs::Entity;

pub struct View {
    pub(crate) visual_parent: Option<Entity>,
    pub(crate) real_render_bounds: Rect,
    tree: u16,
    render: u16,
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
        }
    }

    pub fn visual_parent(&self) -> Option<Entity> {
        self.visual_parent
    }

    pub fn tree(&self) -> u16 {
        self.tree
    }

    pub fn render(&self) -> u16 {
        self.render
    }
}

/*
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
                use $crate::view::is_false as tvxaml_view_is_false;
                use $crate::view::serialize_optional_i16 as tvxaml_view_serialize_optional_i16;
                use $crate::view::deserialize_optional_i16 as tvxaml_view_deserialize_optional_i16;
                $(use $path as $import;)*

                #[serde(default)]
                #[serde(skip_serializing_if="tvxaml_view_is_false")]
                pub is_name_scope: bool,
                #[serde(default)]
                #[serde(skip_serializing_if="String::is_empty")]
                pub name: String,
                #[serde(default)]
                #[serde(skip_serializing_if="Vec::is_empty")]
                pub resources: Vec<Box<dyn $crate::template::Template>>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub layout: Option<Box<dyn $crate::template::Template>>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="tvxaml_view_serialize_optional_i16")]
                #[serde(deserialize_with="tvxaml_view_deserialize_optional_i16")]
                pub width: Option<Option<i16>>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="tvxaml_view_serialize_optional_i16")]
                #[serde(deserialize_with="tvxaml_view_deserialize_optional_i16")]
                pub height: Option<Option<i16>>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub min_size: Option<$crate::base::Vector>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="tvxaml_view_serialize_optional_i16")]
                #[serde(deserialize_with="tvxaml_view_deserialize_optional_i16")]
                pub max_width: Option<Option<i16>>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="tvxaml_view_serialize_optional_i16")]
                #[serde(deserialize_with="tvxaml_view_deserialize_optional_i16")]
                pub max_height: Option<Option<i16>>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub h_align: Option<$crate::view::ViewHAlign>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub v_align: Option<$crate::view::ViewVAlign>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub margin: Option<$crate::base::Thickness>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub allow_focus: Option<bool>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub is_enabled: Option<bool>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub secondary_focus_keys: Option<$crate::view::SecondaryFocusKeys>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub visibility: Option<$crate::view::Visibility>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub tab_index: Option<i8>,
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
    ($this:ident, $instance:ident, $names:ident) => {
        {
            use $crate::obj_col::ObjColExt;
            use $crate::view::ViewExt;

            let obj: $crate::alloc_rc_Rc<dyn $crate::view::IsView>
                = $crate::dynamic_cast_dyn_cast_rc($instance.clone()).unwrap();
            for resource in &$this.resources {
                obj.resources().insert(resource.load_content($names));
            }
            $this.layout.as_ref().map(|x|
                obj.set_layout($crate::dynamic_cast_dyn_cast_rc(x.load_content($names)).unwrap())
            );
            $this.width.map(|x| obj.set_width(x));
            $this.height.map(|x| obj.set_height(x));
            $this.min_size.map(|x| obj.set_min_size(x));
            $this.max_width.map(|x| obj.set_max_width(x));
            $this.max_height.map(|x| obj.set_max_height(x));
            $this.h_align.map(|x| obj.set_h_align(x));
            $this.v_align.map(|x| obj.set_v_align(x));
            $this.margin.map(|x| obj.set_margin(x)); 
            $this.allow_focus.map(|x| obj.set_allow_focus(x)); 
            $this.is_enabled.map(|x| obj.set_is_enabled(x));
            $this.secondary_focus_keys.map(|x| obj.set_secondary_focus_keys(x));
            $this.visibility.map(|x| obj.set_visibility(x));
            $this.tab_index.map(|x| obj.set_tab_index(x));
        }
    };
}

view_template! {
    #[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
    #[serde(rename="View")]
    pub struct ViewTemplate in view_template { }
}

#[typetag::serde(name="View")]
impl Template for ViewTemplate {
    fn is_name_scope(&self) -> bool {
        self.is_name_scope
    }

    fn name(&self) -> Option<&String> {
        Some(&self.name)
    }

    fn create_instance(&self) -> Rc<dyn IsObj> {
        View::new()
    }

    fn apply(&self, instance: &Rc<dyn IsObj>, names: &mut NameResolver) {
        let this = self;
        view_apply_template!(this, instance, names);
    }
}
*/

