use int_vec_2d::{HAlign, VAlign, Thickness, Rect, Vector, Point};

pub struct ViewLayout {
    pub(crate) measure_size: Option<(Option<i16>, Option<i16>)>,
    pub(crate) desired_size: Vector,
    pub(crate) arrange_size: Option<Vector>,
    pub(crate) render_bounds: Rect,
    pub(crate) min_size: Vector,
    pub(crate) max_width: Option<i16>,
    pub(crate) max_height: Option<i16>,
    pub(crate) width: Option<i16>,
    pub(crate) height: Option<i16>,
    pub(crate) margin: Thickness,
    pub(crate) h_align: Option<HAlign>,
    pub(crate) v_align: Option<VAlign>,
}

impl ViewLayout {
    pub fn new() -> Self {
        ViewLayout {
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

    pub fn desired_size(&self) -> Vector {
        self.desired_size
    }

    pub fn render_bounds(&self) -> Rect {
        self.render_bounds
    }

    pub fn min_size(&self) -> Vector {
        self.min_size
    }

    pub fn max_width(&self) -> Option<i16> {
        self.max_width
    }

    pub fn max_height(&self) -> Option<i16> {
        self.max_height
    }

    pub fn width(&self) -> Option<i16> {
        self.width
    }

    pub fn height(&self) -> Option<i16> {
        self.height
    }

    pub fn margin(&self) -> Thickness {
        self.margin
    }

    pub fn h_align(&self) -> Option<HAlign> {
        self.h_align
    }

    pub fn v_align(&self) -> Option<VAlign> {
        self.v_align
    }
}
