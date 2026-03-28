use crate::property;
use crate::termx::IsTermx;
use int_vec_2d::{HAlign, VAlign, Thickness, Rect, Vector, Point};

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
    h_align: Option<HAlign>,
    v_align: Option<VAlign>,
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
            h_align: None,
            v_align: None,
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
    property!(Termx, view_layout, h_align, Option<HAlign>, @measure);
    property!(Termx, view_layout, v_align, Option<VAlign>, @measure);
}
