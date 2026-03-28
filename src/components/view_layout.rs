use alloc::string::{String, ToString};
use core::str::FromStr;
use crate::base::{ViewHAlign, ViewVAlign};
use crate::property;
use crate::termx::IsTermx;
use int_vec_2d::{Thickness, Rect, Vector, Point};
use serde::de::{self};
use serde::de::Error as de_Error;
use serde::{Deserializer, Serializer, Serialize, Deserialize};

pub struct ViewLayout {
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

impl ViewLayout {
    pub fn new(layout: u16) -> Self {
        ViewLayout {
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

    property!(Termx, view_layout, min_size, Vector, @measure);
    property!(Termx, view_layout, max_width, Option<i16>, @measure);
    property!(Termx, view_layout, max_height, Option<i16>, @measure);
    property!(Termx, view_layout, width, Option<i16>, @measure);
    property!(Termx, view_layout, height, Option<i16>, @measure);
    property!(Termx, view_layout, margin, Thickness, @measure);
    property!(Termx, view_layout, h_align, ViewHAlign, @measure);
    property!(Termx, view_layout, v_align, ViewVAlign, @measure);
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
macro_rules! view_layout_template {
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
                $(use $path as $import;)*
                use $crate::components::view::layout::serialize_optional_i16
                    as termx_components_view_layout_serialize_optional_i16;
                use $crate::components::view::layout::deserialize_optional_i16
                    as termx_components_view_layout_deserialize_optional_i16;

                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="termx_components_view_layout_serialize_optional_i16")]
                #[serde(deserialize_with="termx_components_view_layout_deserialize_optional_i16")]
                pub width: Option<Option<i16>>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="termx_components_view_layout_serialize_optional_i16")]
                #[serde(deserialize_with="termx_components_view_layout_deserialize_optional_i16")]
                pub height: Option<Option<i16>>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub min_size: Option<$crate::base::Vector>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="termx_components_view_layout_serialize_optional_i16")]
                #[serde(deserialize_with="termx_components_view_layout_deserialize_optional_i16")]
                pub max_width: Option<Option<i16>>,
                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                #[serde(serialize_with="termx_components_view_layout_serialize_optional_i16")]
                #[serde(deserialize_with="termx_components_view_layout_deserialize_optional_i16")]
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
macro_rules! view_layout_apply_template {
    ($this:ident, $entity:ident, $termx:ident, $names:ident) => {
        $crate::view_apply_template! { $this, $entity, $termx, $names }
        $this.width.map(|x| $crate::components::view::View::set_width($entity, $termx, x));
        $this.height.map(|x| $crate::components::view::View::set_height($entity, $termx, x));
        $this.min_size.map(|x| $crate::components::view::View::set_min_size($entity, $termx, x));
        $this.max_width.map(|x| $crate::components::view::View::set_max_width($entity, $termx, x));
        $this.max_height.map(|x| $crate::components::view::View::set_max_height($entity, $termx, x));
        $this.h_align.map(|x| $crate::components::view::View::set_h_align($entity, $termx, x));
        $this.v_align.map(|x| $crate::components::view::View::set_v_align($entity, $termx, x));
        $this.margin.map(|x| $crate::components::view::View::set_margin($entity, $termx, x)); 
    };
}
