use alloc::vec::Vec;
use core::cmp::{max, min};
use core::iter::{self};
use core::mem::{replace, transmute};
use core::slice::{self};
use crate::base::{graphemes, Point, Rect, Vector, HAlign, Range1d, text_width, trim_text, TextWrapping};
use either::{Left, Right};
use iter_identify_first_last::IteratorIdentifyFirstLastExt;
use itertools::Itertools;

pub fn render_text(
    mut r: impl FnMut(Point, &str),
    bounds: Rect,
    align: Option<HAlign>,
    wrapping: TextWrapping,
    text: &str,
) -> Rect {
    let mut rendered = Rect { tl: bounds.tl, size: Vector::null() };
    for block in text.split('\n') {
        let block_bounds = Rect::from_tl_br(rendered.bl(), bounds.br());
        if block_bounds.is_empty() { break; }
        let mut block_rendered = render_block(&mut r, block_bounds, align, wrapping, block);
        block_rendered.size.x = max(block_rendered.size.x as u16, rendered.size.x as u16) as i16;
        rendered = Rect::from_tl_br(bounds.tl, block_rendered.br());
    }
    rendered
}

fn render_block(
    mut r: impl FnMut(Point, &str),
    bounds: Rect,
    align: Option<HAlign>,
    wrapping: TextWrapping,
    text: &str,
) -> Rect {
    if text.is_empty() {
        return Rect { tl: bounds.tl, size: Vector { x: 0, y: 1 } };
    }
    if wrapping != TextWrapping::NoWrap {
        let words = split_words(text);
        let runs = words.identify_first().map(|(f, x)| (!f, x, text_width(x))).flat_map(|(s, x, w)| {
            if wrapping == TextWrapping::WrapWithOverflow || w as u16 <= bounds.w() as u16 {
                Left(iter::once((s, x, min(w as u16, bounds.w() as u16) as i16)))
            } else {
                Right(
                    graphemes(x)
                        .identify_first()
                        .map(move |(first, (g, w))| (
                            if first { s } else { false },
                            &x[g],
                            min(w as u16, bounds.w() as u16) as i16
                        ))
                )
            }
        });
        let lines = wrap(bounds, runs);
        let mut y = bounds.t();
        let mut range = Range1d { start: 0, end: 0 };
        for line in lines {
            let line_range = render_line(
                &mut r,
                Rect { tl: Point { x: bounds.l(), y }, size: Vector { x: bounds.w(), y: 1 } },
                align,
                line,
            );
            range = range.union(line_range).unwrap_or(bounds.h_range());
            y = y.wrapping_add(1);
        }
        Rect::from_h_v_ranges(range, Range1d { start: bounds.t(), end: y })
    } else {
        let words = split_words(text);
        let runs = words.identify_first().map(|(f, x)| (!f, x, text_width(x))).collect();
        let range = render_line(r, bounds, align, runs);
        Rect::from_h_v_ranges(range, Range1d { start: bounds.t(), end: bounds.t().wrapping_add(1) })
    }
}

fn split_words(text: &str) -> impl Iterator<Item=&str> {
    let mut words = text.split(' ').map(trim_text).filter(|x| !x.is_empty());
    let first_word = if let Some(first_word) = words.next() {
        unsafe { transmute::<&[u8], &str>(
            slice::from_ptr_range(text.as_ptr() .. first_word.as_ptr().add(first_word.len()))
        ) }
    } else {
        text
    };
    let last_word = if let Some(last_word) = words.next_back() {
        Some(unsafe { transmute::<&[u8], &str>(
            slice::from_ptr_range(last_word.as_ptr() .. text.as_ptr().add(text.len()))
        ) })
    } else {
        None
    };
    iter::once(first_word).chain(words.chain(last_word.into_iter()))
}

fn render_line(
    mut r: impl FnMut(Point, &str),
    bounds: Rect,
    align: Option<HAlign>,
    line: Vec<(bool, &str, i16)>,
) -> Range1d {
    match align {
        None => {
            let space_runs_count = line.iter().filter(|x| x.0).count();
            if space_runs_count == 0 {
                let mut x = bounds.l();
                for run in line {
                    r(Point { x, y: bounds.t() }, run.1);
                    x = x.wrapping_add(run.2);
                }
                Range1d { start: bounds.l(), end: x }
            } else {
                let min_width = line.iter().map(|x| x.2).fold(0i16, |s, w| s.wrapping_add(w));
                let spaces_count = ((bounds.w() as u16).saturating_sub(min_width as u16)) as usize;
                let spaces_per_run = min(spaces_count / space_runs_count, 1);
                let spaces_runs_head_len = spaces_count.saturating_sub(spaces_per_run * space_runs_count);
                let mut x = bounds.l();
                for (n, run) in line.into_iter().enumerate() {
                    if n == 0 || !run.0 {
                    } else if n <= spaces_runs_head_len {
                        x = x.wrapping_add((spaces_per_run + 1) as u16 as i16);
                    } else {
                        x = x.wrapping_add(spaces_per_run as u16 as i16);
                    }
                    r(Point { x, y: bounds.t() }, run.1);
                    x = x.wrapping_add(run.2);
                }
                bounds.h_range()
            }
        },
        Some(HAlign::Left) => {
            let mut x = bounds.l();
            for run in line {
                if run.0 {
                    x = x.wrapping_add(1);
                }
                r(Point { x, y: bounds.t() }, run.1);
                x = x.wrapping_add(run.2);
            }
            Range1d { start: bounds.l(), end: x }
        },
        Some(HAlign::Right) => {
            let mut x = bounds.r();
            for run in line.into_iter().rev() {
                x = x.wrapping_sub(run.2);
                r(Point { x, y: bounds.t() }, run.1);
                if run.0 {
                    x = x.wrapping_sub(1);
                }
            }
            Range1d { start: x, end: bounds.r() }
        },
        Some(HAlign::Center) => {
            let line_width = line.iter().map(|x| {
                let space_width = if x.0 { 1i16 } else { 0 };
                space_width.wrapping_add(x.2)
            }).fold(0i16, |s, w| s.wrapping_add(w));
            let start = bounds.l().wrapping_add(
                (((bounds.w() as u16).saturating_sub(line_width as u16)) / 2) as i16
            );
            let mut x = start;
            for run in line {
                if run.0 {
                    x = x.wrapping_add(1);
                }
                r(Point { x, y: bounds.t() }, run.1);
                x = x.wrapping_add(run.2);
            }
            Range1d { start, end: x }
        },
    }
}

fn wrap<'a>(
    bounds: Rect,
    runs: impl Iterator<Item=(bool, &'a str, i16)>
) -> impl Iterator<Item=Vec<(bool, &'a str, i16)>> {
    let mut line = Vec::new();
    let mut p = bounds.tl;
    runs.batching(move |i| {
        if (p.y - bounds.t()) as u16 >= bounds.h() as u16 { return None; }
        loop {
            let Some((space, run, run_width)) = i.next() else {
                let res = replace(&mut line, Vec::new());
                break if res.is_empty() { None } else { Some(res) };
            };
            let space_width = if space { 1i16 } else { 0 };
            if
                   run_width as u16 > (bounds.r() - p.x) as u16
                || space_width.wrapping_add(run_width) as u16 > (bounds.r() - p.x) as u16
            {
                let res = replace(&mut line, Vec::new());
                p.x = bounds.l().wrapping_add(run_width);
                p.y = p.y.wrapping_add(1);
                line.push((false, run, run_width));
                debug_assert!(!res.is_empty());
                break Some(res);
            } else {
                line.push((space, run, run_width));
                p.x = p.x.wrapping_add(space_width.wrapping_add(run_width));
            }
        }
    })
}
