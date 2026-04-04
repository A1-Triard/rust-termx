use alloc::format;
use alloc::string::String;
use core::str::FromStr;
use iter_identify_first_last::IteratorIdentifyFirstLastExt;
use serde::{Serialize, Deserialize, Deserializer, Serializer};
use serde::de::{self};

pub use int_vec_2d::*;
pub use termx_screen_base::*;
pub use timer_no_std::*;

pub fn label_width(text: &str) -> i16 {
    let mut width = 0i16;
    let mut hotkey = false;
    for (first, last, text) in text.split('~').identify_first_last() {
        if !first && !text.is_empty() {
            hotkey = !hotkey;
        }
        let actual_text = if !first && !last && text.is_empty() { "~" } else { text };
        width = width.wrapping_add(text_width(actual_text));
        if !first && text.is_empty() {
            hotkey = !hotkey;
        }
    }
    width
}

pub fn label(text: &str) -> Option<char> {
    let mut hotkey = false;
    for (first, last, text) in text.split('~').identify_first_last() {
        if !first && !text.is_empty() {
            hotkey = !hotkey;
        }
        let actual_text = if !first && !last && text.is_empty() { "~" } else { text };
        if hotkey && !actual_text.is_empty() {
            return Some(actual_text.chars().next().unwrap().to_lowercase().next().unwrap());
        }
        if !first && text.is_empty() {
            hotkey = !hotkey;
        }
    }
    None
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Serialize, Deserialize)]
pub enum TextWrapping {
    NoWrap,
    Wrap,
    WrapWithOverflow,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Serialize, Deserialize)]
pub enum TextAlign { Left, Center, Right, Justify }

impl From<Option<HAlign>> for TextAlign {
    fn from(value: Option<HAlign>) -> Self {
        match value {
            Some(HAlign::Left) => TextAlign::Left,
            Some(HAlign::Center) => TextAlign::Center,
            Some(HAlign::Right) => TextAlign::Right,
            None => TextAlign::Justify,
        }
    }
}

impl From<TextAlign> for Option<HAlign> {
    fn from(value: TextAlign) -> Self {
        match value {
            TextAlign::Left => Some(HAlign::Left),
            TextAlign::Center => Some(HAlign::Center),
            TextAlign::Right => Some(HAlign::Right),
            TextAlign::Justify => None,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Serialize, Deserialize)]
pub enum ViewHAlign { Left, Center, Right, Stretch }

impl From<ViewHAlign> for Option<HAlign> {
    fn from(a: ViewHAlign) -> Option<HAlign> {
        match a {
            ViewHAlign::Left => Some(HAlign::Left),
            ViewHAlign::Center => Some(HAlign::Center),
            ViewHAlign::Right => Some(HAlign::Right),
            ViewHAlign::Stretch => None,
        }
    }
}

impl From<Option<HAlign>> for ViewHAlign {
    fn from(a: Option<HAlign>) -> ViewHAlign {
        match a {
            Some(HAlign::Left) => ViewHAlign::Left,
            Some(HAlign::Center) => ViewHAlign::Center,
            Some(HAlign::Right) => ViewHAlign::Right,
            None => ViewHAlign::Stretch,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Serialize, Deserialize)]
pub enum ViewVAlign { Top, Center, Bottom, Stretch }

impl From<ViewVAlign> for Option<VAlign> {
    fn from(a: ViewVAlign) -> Option<VAlign> {
        match a {
            ViewVAlign::Top => Some(VAlign::Top),
            ViewVAlign::Center => Some(VAlign::Center),
            ViewVAlign::Bottom => Some(VAlign::Bottom),
            ViewVAlign::Stretch => None,
        }
    }
}

impl From<Option<VAlign>> for ViewVAlign {
    fn from(a: Option<VAlign>) -> ViewVAlign {
        match a {
            Some(VAlign::Top) => ViewVAlign::Top,
            Some(VAlign::Center) => ViewVAlign::Center,
            Some(VAlign::Bottom) => ViewVAlign::Bottom,
            None => ViewVAlign::Stretch,
        }
    }
}

pub fn deserialize_color<'de, D: Deserializer<'de>>(
    deserializer: D
) -> Result<Option<(Fg, Bg)>, D::Error> {
    if deserializer.is_human_readable() {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() { return Ok(None); };
        let mut parts = s.split('/');
        let fg = parts.next().ok_or_else(|| de::Error::custom("invalid color"))?;
        let bg = parts.next().ok_or_else(|| de::Error::custom("invalid color"))?;
        if parts.next().is_some() { return Err(de::Error::custom("invalid color")); }
        let fg = Fg::from_str(fg).map_err(|_| de::Error::custom("invalid color"))?;
        let bg = Bg::from_str(bg).map_err(|_| de::Error::custom("invalid color"))?;
        Ok(Some((fg, bg)))
    } else {
        <Option<(Fg, Bg)>>::deserialize(deserializer)
    }
}

pub fn serialize_color<S: Serializer>(value: &Option<(Fg, Bg)>, serializer: S) -> Result<S::Ok, S::Error> {
    if serializer.is_human_readable() {
        if let Some((fg, bg)) = value {
            format!("{fg}/{bg}").serialize(serializer)
        } else {
            "".serialize(serializer)
        }
    } else {
        value.serialize(serializer)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Serialize, Deserialize)]
pub enum Visibility { Visible, Hidden, Collapsed }
