use cgmath::{ElementWise, Point2, Vector2};
use sdl2::rect::Rect;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SubRect {
    outer: Rect,
    ratio: Vector2<f32>,
    align: Vector2<Align>,
}

impl SubRect {
    pub fn of(
        outer: Rect,
        ratio: (f32, f32),
        align: Option<(Align, Align)>,
    ) -> Self {
        Self {
            outer,
            ratio: ratio.into(),
            align: align.unwrap_or((Align::Center, Align::Center)).into(),
        }
    }

    /// creates a new `sub_rect` consuming `self` as the parent.
    pub fn sub_rect(
        &self,
        ratio: (f32, f32),
        align: Option<(Align, Align)>,
    ) -> Self {
        Self::of(Rect::from(self), ratio, align)
    }

    /// Makes a sub rect with absolute dimensions instead of a ratio.
    pub fn absolute(
        outer: Rect,
        ratio: (f32, f32),
        align: Option<(Align, Align)>,
    ) -> Self {
        let Vector2 { x, y } = Vector2::from(outer.size())
            .cast::<f32>()
            .expect("Never fails");

        let aspect_correction = Vector2::from(if x > y {
            (y / x, 1.0) // landscape
        } else {
            (1.0, x / y) // portrait
        });

        let ratio = Vector2::from(ratio)
            .mul_element_wise(aspect_correction)
            .into();

        Self::of(outer, ratio, align)
    }

    pub fn size(&self) -> Vector2<u32> {
        let outer_size = Vector2::from(self.outer.size())
            .cast::<f32>()
            .expect("Never fails");

        outer_size
            .mul_element_wise(self.ratio)
            .map(f32::trunc)
            .cast()
            .expect("Never fails")
    }

    pub fn top_left(&self) -> Point2<i32> {
        let outer_top_left: (i32, i32) = self.outer.top_left().into();
        let margin = self
            .total_margin()
            .mul_element_wise(self.align.map(Align::front_margin));

        Point2::from(outer_top_left) + margin.cast().expect("Never fails")
    }

    pub fn bottom_left(&self) -> Point2<i32> {
        let outer_bottom_left: (i32, i32) = self.outer.bottom_left().into();
        let margin = self
            .total_margin()
            .mul_element_wise(self.align.map(Align::back_margin))
            .mul_element_wise(Vector2::new(1.0, -1.0));

        Point2::from(outer_bottom_left) + margin.cast().expect("Never fails")
    }

    fn total_margin(&self) -> Vector2<f32> {
        Vector2::from(self.outer.size())
            .cast()
            .expect("Never fails")
            .mul_element_wise(Vector2::new(1.0, 1.0) - self.ratio)
    }
}

impl From<SubRect> for Rect {
    fn from(sub_rect: SubRect) -> Self {
        Self::from(&sub_rect)
    }
}

impl From<&SubRect> for Rect {
    fn from(region: &SubRect) -> Self {
        let Point2 { x, y } = region.top_left();
        let Vector2 { x: w, y: h } = region.size();

        Rect::new(x, y, w, h)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Align {
    Near,
    Center,
    Far,
}

impl Align {
    pub fn front_margin(self) -> f32 {
        match self {
            Self::Near => 0.0,
            Self::Center => 0.5,
            Self::Far => 1.0,
        }
    }

    pub fn back_margin(self) -> f32 {
        1.0 - self.front_margin()
    }
}
