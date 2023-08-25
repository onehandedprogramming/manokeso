use crate::{render::primitive::RoundedRectInstance, util::point::Point};

pub struct RoundedRect {
    pub top_left: UIPoint,
    pub bottom_right: UIPoint,
    pub colors: [[f32; 4]; 4],
    pub radius: f32,
    pub inner_radius: f32,
    pub thickness: f32,
}

impl RoundedRect {
    pub fn to_primitive(&self) -> RoundedRectInstance {
        RoundedRectInstance {
            top_left_anchor: self.top_left.anchor,
            top_left_offset: self.top_left.offset,
            bottom_right_anchor: self.bottom_right.anchor,
            bottom_right_offset: self.bottom_right.offset,
            colors: self.colors,
            radius: self.radius,
            inner_radius: self.inner_radius,
            thickness: self.thickness,
        }
    }
}

impl Default for RoundedRect {
    fn default() -> Self {
        Self {
            top_left: UIPoint {
                anchor: Point::zero(),
                offset: Point::zero(),
            },
            bottom_right: UIPoint {
                anchor: Point { x: 1.0, y: 1.0 },
                offset: Point::zero(),
            },
            colors: [
                [0.0, 0.0, 0.0, 1.0],
                [0.0, 0.0, 0.0, 1.0],
                [0.0, 0.0, 0.0, 1.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            radius: 0.0,
            thickness: 0.0,
            inner_radius: 0.0,
        }
    }
}

pub struct UIPoint {
    pub anchor: Point<f32>,
    pub offset: Point<f32>,
}

impl UIPoint {
    pub fn anchor_offset(anchor_x: f32, anchor_y: f32, offset_x: f32, offset_y: f32) -> Self {
        Self {
            anchor: Point::new(anchor_x, anchor_y),
            offset: Point::new(offset_x, offset_y),
        }
    }
}