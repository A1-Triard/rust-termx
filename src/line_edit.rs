use alloc::string::String;
use core::iter::once;
use core::ops::{Range, RangeInclusive};
use crate::base::{text_width, graphemes, char_width};
use int_vec_2d::{Vector, Point, Rect, HAlign, VAlign, Thickness};
use unicode_width::UnicodeWidthChar;

pub struct LineEdit {
    text: String,
    view: Option<(i16, RangeInclusive<usize>, i16)>,
    cursor: usize,
    delete_char: bool,
    width: i16,
    is_numeric: bool,
    is_focused: bool,
}

pub struct LineEditRender<'a> {
    pub text: &'a str,
    pub text_start: i16,
    pub left_ellipsis: bool,
    pub right_ellipsis: bool,
    pub cursor: Option<i16>,
}

impl LineEdit {
    pub fn new() -> Self {
        LineEdit {
            text: String::new(),
            cursor: 0,
            view: None,
            delete_char: false,
            is_numeric: false,
            width: 0,
            is_focused: false,
        }
    }

    pub fn text(&self) -> &String { &self.text }

    pub fn change_text<T>(&mut self, f: impl FnOnce(&mut String) -> T) -> (T, bool) {
        let res = f(&mut self.text);
        self.reset_view();
        (res, true)
    }

    pub fn is_numeric(&self) -> bool { self.is_numeric }

    pub fn set_is_numeric(&mut self, value: bool) -> bool {
        self.is_numeric = value;
        self.reset_view();
        true
    }

    pub fn width(&self) -> i16 { self.width }

    pub fn set_width(&mut self, value: i16) -> bool {
        self.width = value;
        self.reset_view();
        true
    }

    pub fn set_is_focused(&mut self, value: bool) -> bool {
        self.is_focused = value;
        self.reset_view();
        true
    }

    fn reset_view(&mut self) {
        self.delete_char = false;
        self.cursor = self.text.len();
        if self.is_focused {
            self.calc_view_start(self.text.len());
        } else if self.is_numeric {
            let view_end = graphemes(&self.text).next_back().map(|(g, _)| g.end - 1);
            if let Some(view_end) = view_end {
                self.calc_view_start(view_end);
            } else {
                self.view = None;
            }
        } else {
            let view_start = graphemes(&self.text).next().map(|(g, _)| g.start);
            if let Some(view_start) = view_start {
                self.calc_view_end(view_start, 0);
            } else {
                self.view = None;
            }
        }
    }

    fn calc_view_start(&mut self, view_end: usize) {
        let view = 'r: {
            let with_end = view_end == self.text.len();
            let text = if with_end { &self.text[.. view_end] } else { &self.text[..= view_end] };
            let mut w = if with_end { 1i16 } else { 0i16 };
            let mut prev_g: Option<Range<usize>> = None;
            for (g, g_w) in graphemes(text).rev() {
                if w.wrapping_add(g_w) as u16 > self.width as u16 {
                    break 'r prev_g.map(|x| (self.width.wrapping_sub(w), x.start ..= view_end));
                }
                w = w.wrapping_add(g_w);
                prev_g = Some(g);
            }
            prev_g.map(|x| (0, x.start ..= view_end))
        };
        self.view = view.map(|(p, x)| (p, x, 0));
    }

    fn calc_view_end(&mut self, view_start: usize, left_p: i16) {
        let view = 'r: {
            let mut w = left_p;
            let mut prev_g: Option<usize> = None;
            for
                (g, g_w)
            in
                graphemes(&self.text[view_start .. ])
                    .map(|(g, g_w)| (view_start + g.end - 1, g_w))
                    .chain(once((self.text.len(), 1)))
            {
                if w.wrapping_add(g_w) as u16 > self.width as u16 {
                    break 'r prev_g.map(|x| (view_start ..= x, self.width.wrapping_sub(w)));
                }
                w = w.wrapping_add(g_w);
                prev_g = Some(g);
            }
            prev_g.map(|x| (view_start ..= x, 0))
        };
        self.view = view.map(|(x, p)| (left_p, x, p));
    }

    pub fn render(&self) -> Option<LineEditRender<'_>> {
        let (left_p, view, right_p) = self.view.clone()?;
        let show_text_end = view.contains(&self.text.len());
        let text = if show_text_end {
            &self.text[*view.start() .. *view.end()]
        } else {
            &self.text[view.clone()]
        };
        let the_text_width = text_width(text)
            .wrapping_add(left_p)
            .wrapping_add(right_p)
            .wrapping_add(if show_text_end { 1 } else { 0 })
        ;
        let align = Thickness::align(
            Vector { x: the_text_width, y: 1 },
            Vector { x: self.width, y: 1 },
            if self.is_numeric { HAlign::Right } else { HAlign::Left },
            VAlign::Top
        );
        let text_start = align
            .shrink_rect(Rect { tl: Point { x: 0, y: 0 }, size: Vector { x: self.width, y: 1 } })
            .tl
            .offset(Vector { x: left_p, y: 0 })
            .x
        ;
        let left_ellipsis = graphemes(&self.text[.. *view.start()]).next_back().is_some();
        let right_ellipsis = !show_text_end && graphemes(&self.text[*view.end() + 1 .. ]).next().is_some();
        let cursor = if self.is_focused && view.contains(&self.cursor) {
            let cursor_x = text_width(&self.text[*view.start() .. self.cursor]);
            Some(text_start.wrapping_add(cursor_x))
        } else {
            None
        };
        Some(LineEditRender {
            text,
            text_start,
            left_ellipsis,
            right_ellipsis,
            cursor,
        })
    }

    pub fn cursor_left(&mut self) -> bool {
        let view_start = {
            let Some((g, _)) = graphemes(&self.text[.. self.cursor]).next_back() else { return false; };
            self.delete_char = false;
            self.cursor = g.start;
            if let Some((_, view, _)) = self.view.clone() && view.contains(&self.cursor) {
                None
            } else {
                Some(self.cursor)
            }
        };
        view_start.map(|x| self.calc_view_end(x, 0));
        true
    }

    pub fn cursor_right(&mut self) -> bool {
        let view_end = {
            let mut graphemes = graphemes(&self.text[self.cursor ..]);
            if graphemes.next().is_none() { return false; }
            let cursor_end = if let Some((g, _)) = graphemes.next() {
                let cursor_end = self.cursor + g.end - 1;
                self.cursor += g.start;
                cursor_end
            } else {
                self.cursor = self.text.len();
                self.text.len()
            };
            self.delete_char = false;
            if let Some((_, view, _)) = self.view.clone() && view.contains(&self.cursor) {
                None
            } else {
                Some(cursor_end)
            }
        };
        view_end.map(|x| self.calc_view_start(x));
        true
    }

    pub fn cursor_home(&mut self) -> bool {
        let view_start = {
            self.delete_char = false;
            let view_start = graphemes(&self.text).next().map(|(g, _)| g.start);
            self.cursor = view_start.unwrap_or(0);
            view_start
        };
        if let Some(view_start) = view_start {
            self.calc_view_end(view_start, 0);
        } else {
            self.view = None;
        }
        true
    }

    pub fn cursor_end(&mut self) -> bool {
        let text_len = {
            self.delete_char = false;
            self.cursor = self.text.len();
            self.text.len()
        };
        self.calc_view_start(text_len);
        true
    }

    pub fn type_char(&mut self, c: char) -> bool {
        if c == '\0' { return false; }
        let Some(c_w) = c.width() else { return false; };
        let (view_start, left_p) = {
            let prev_gr = if c_w == 0 {
                let Some((g, _)) = graphemes(&self.text[.. self.cursor]).next_back() else { return false; };
                Some(g)
            } else {
                None
            };
            self.delete_char = true;
            let cursor = self.cursor;
            self.text.insert(cursor, c);
            self.cursor += c.len_utf8();
            if let Some((left_p, view, _)) = self.view.clone() && *view.start() < cursor {
                (*view.start(), left_p) 
            } else {
                if let Some(prev_gr) = prev_gr {
                    (prev_gr.start, 0)
                } else {
                    (cursor, 0)
                }
            }
        };
        self.calc_view_end(view_start, left_p);
        let view_end = {
            if let Some((_, view, _)) = self.view.clone() && view.contains(&self.cursor) {
                None
            } else {
                let cursor_end = self.cursor
                    + graphemes(&self.text[self.cursor ..]).next().map_or(0, |(g, _)| g.end - 1);
                Some(cursor_end)
            }
        };
        if let Some(view_end) = view_end {
            self.calc_view_start(view_end);
        }
        true
    }

    pub fn delete_before_cursor(&mut self) -> bool {
        let (view_start, left_p) = {
            if self.delete_char {
                let c = self.text[.. self.cursor].chars().next_back().unwrap();
                self.cursor -= c.len_utf8();
                let cursor = self.cursor;
                self.text.remove(cursor);
                if char_width(c) != 0 {
                    self.delete_char = false;
                }
            } else {
                let Some((g, _)) = graphemes(&self.text[.. self.cursor]).next_back() else { return false; };
                let cursor = self.cursor;
                self.text.replace_range(g.start .. cursor, "");
                self.cursor = g.start;
            };
            if let Some((left_p, view, _)) = self.view.clone() && *view.start() <= self.cursor {
                (*view.start(), left_p)
            } else {
                (self.cursor, 0)
            }
        };
        self.calc_view_end(view_start, left_p);
        let view_end = {
            if let Some((left_p, view, right_p)) = self.view.clone() {
                let with_end = view.contains(&self.text.len());
                if with_end {
                    let text = &self.text[*view.start() .. *view.end()];
                    let text_width = text_width(text).wrapping_add(1).wrapping_add(left_p).wrapping_add(right_p);
                    if text_width < self.width {
                        Some(self.text.len())
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        };
        view_end.map(|x| self.calc_view_start(x));
        true
    }
}
