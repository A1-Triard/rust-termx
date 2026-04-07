use alloc::string::{String, ToString};
use core::str::FromStr;
use crate::base::{ViewHAlign, ViewVAlign};
use crate::property;
use crate::termx::IsTermx;
use int_vec_2d::{Thickness, Rect, Vector, Point};
use serde::de::{self};
use serde::de::Error as de_Error;
use serde::{Deserializer, Serializer, Serialize, Deserialize};

pub struct LayoutView {
    layout: u16,
    pub(crate) measure_size: Option<(Option<i16>, Option<i16>)>,
    pub(crate) desired_size: Vector,
    pub(crate) arrange_size: Option<Vector>,
    pub(crate) render_bounds: Rect,
    min_size: Vector,
    max_width: Option<i16>,
    max_height: Option<i16>,
    width: Option<i16>,
    height: Option<i16>,
    margin: Thickness,
    h_align: ViewHAlign,
    v_align: ViewVAlign,
}

pub const LAYOUT_NONE: u16 = 0;
pub const LAYOUT_BACKGROUND: u16 = 1;
pub const LAYOUT_STACK_PANEL: u16 = 2;
pub const LAYOUT_CANVAS: u16 = 3;
pub const LAYOUT_T_BUTTON: u16 = 4;
pub const LAYOUT_STATIC_TEXT: u16 = 5;
pub const LAYOUT_CONTENT_PRESENTER: u16 = 6;
pub const LAYOUT_BORDER: u16 = 7;
pub const LAYOUT_ADORNERS_PANEL: u16 = 8;
pub const LAYOUT_CONTROL: u16 = 9;
pub const LAYOUT_BUTTON: u16 = 10;

impl LayoutView {
    pub fn new(layout: u16) -> Self {
        LayoutView {
            layout,
            measure_size: None,
            desired_size: Vector::null(),
            arrange_size: None,
            render_bounds: Rect { tl: Point { x: 0, y: 0 }, size: Vector::null() },
            min_size: Vector::null(),
            max_width: None,
            max_height: None,
            width: None,
            height: None,
            margin: Thickness::all(0),
            h_align: ViewHAlign::Stretch,
            v_align: ViewVAlign::Stretch,
        }
    }

    pub fn layout(&self) -> u16 {
        self.layout
    }

    pub fn desired_size(&self) -> Vector {
        self.desired_size
    }

    pub fn render_bounds(&self) -> Rect {
        self.render_bounds
    }

    property!(Termx, layout_view, min_size, Vector, @measure);
    property!(Termx, layout_view, max_width, Option<i16>, @measure);
    property!(Termx, layout_view, max_height, Option<i16>, @measure);
    property!(Termx, layout_view, width, Option<i16>, @measure);
    property!(Termx, layout_view, height, Option<i16>, @measure);
    property!(Termx, layout_view, margin, Thickness, @measure);
    property!(Termx, layout_view, h_align, ViewHAlign, @measure);
    property!(Termx, layout_view, v_align, ViewVAlign, @measure);
}

#[derive(Serialize, Deserialize)]
#[serde(rename="OptionalI16")]
enum OptionalI16NHRSurrogate {
    None,
    Some(i16)
}

impl From<Option<i16>> for OptionalI16NHRSurrogate {
    fn from(value: Option<i16>) -> OptionalI16NHRSurrogate {
        match value {
            None => OptionalI16NHRSurrogate::None,
            Some(x) => OptionalI16NHRSurrogate::Some(x),
        }
    }
}

impl From<OptionalI16NHRSurrogate> for Option<i16> {
    fn from(value: OptionalI16NHRSurrogate) -> Option<i16> {
        match value {
            OptionalI16NHRSurrogate::None => None,
            OptionalI16NHRSurrogate::Some(x) => Some(x),
        }
    }
}

#[doc(hidden)]
pub fn serialize_optional_i16<S>(
    value: &Option<Option<i16>>, serializer: S
) -> Result<S::Ok, S::Error> where S: Serializer {
    if serializer.is_human_readable() {
        let s = value.map(|x| x.map_or_else(|| "None".to_string(), |x| x.to_string()));
        s.serialize(serializer)
    } else {
        value.map(OptionalI16NHRSurrogate::from).serialize(serializer)
    }
}

#[doc(hidden)]
pub fn deserialize_optional_i16<'de, D>(
    deserializer: D
) -> Result<Option<Option<i16>>, D::Error> where D: Deserializer<'de> {
    if deserializer.is_human_readable() {
        let s = <Option<String>>::deserialize(deserializer)?;
        let Some(s) = s else { return Ok(None); };
        if s == "None" { return Ok(Some(None)); }
        Ok(Some(Some(i16::from_str(&s).map_err(|_| D::Error::invalid_value(de::Unexpected::Str(&s), &"i16"))?)))
    } else {
        let v = <Option<OptionalI16NHRSurrogate>>::deserialize(deserializer)?;
        Ok(v.map(|x| x.into()))
    }
}

#[macro_export]
macro_rules! layout_view_template {
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
        $crate::view_template! {
            $(#[$attr])*
            $vis struct $name in $mod {
                use $crate::components::layout_view::serialize_optional_i16
                    as termx_components_layout_view_serialize_optional_i16;
                use $crate::components::layout_view::deserialize_optional_i16
                    as termx_components_layout_view_deserialize_optional_i16;
                $(use $path as $import;)*

                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="termx_components_layout_view_serialize_optional_i16")]
                #[serde(deserialize_with="termx_components_layout_view_deserialize_optional_i16")]
                pub width: Option<Option<i16>>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="termx_components_layout_view_serialize_optional_i16")]
                #[serde(deserialize_with="termx_components_layout_view_deserialize_optional_i16")]
                pub height: Option<Option<i16>>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub min_size: Option<$crate::base::Vector>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="termx_components_layout_view_serialize_optional_i16")]
                #[serde(deserialize_with="termx_components_layout_view_deserialize_optional_i16")]
                pub max_width: Option<Option<i16>>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="termx_components_layout_view_serialize_optional_i16")]
                #[serde(deserialize_with="termx_components_layout_view_deserialize_optional_i16")]
                pub max_height: Option<Option<i16>>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub h_align: Option<$crate::base::ViewHAlign>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub v_align: Option<$crate::base::ViewVAlign>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub margin: Option<$crate::base::Thickness>,
                $($(
                    $(#[$field_attr])*
                    pub $field_name : $field_ty
                ),+)?
            }
        }
    };
}

#[macro_export]
macro_rules! layout_view_apply_template {
    ($this:ident, $entity:ident, $world:expr, $termx:expr, $names:ident) => {
        $crate::view_apply_template! { $this, $entity, $world, $termx, $names }
        $this.width.map(|x| $crate::components::layout_view::LayoutView::set_width($entity, $world, $termx, x));
        $this.height.map(|x|
            $crate::components::layout_view::LayoutView::set_height($entity, $world, $termx, x)
        );
        $this.min_size.map(|x|
            $crate::components::layout_view::LayoutView::set_min_size($entity, $world, $termx, x)
        );
        $this.max_width.map(|x|
            $crate::components::layout_view::LayoutView::set_max_width($entity, $world, $termx, x)
        );
        $this.max_height.map(|x|
            $crate::components::layout_view::LayoutView::set_max_height($entity, $world, $termx, x)
        );
        $this.h_align.map(|x|
            $crate::components::layout_view::LayoutView::set_h_align($entity, $world, $termx, x)
        );
        $this.v_align.map(|x|
            $crate::components::layout_view::LayoutView::set_v_align($entity, $world, $termx, x)
        );
        $this.margin.map(|x|
            $crate::components::layout_view::LayoutView::set_margin($entity, $world, $termx, x)
        ); 
    };
}
