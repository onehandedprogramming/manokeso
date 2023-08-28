use crate::{render::{Renderer, tile::data::TileData, primitive::RoundedRectInstance}, util::point::Point, client::ui::element::UIPoint, rsc::{DEFAULT_PLAYER_SPEED, DEFAULT_PLAYER_SIZE}};

use super::ui::element::RoundedRect;

pub struct Player {
    pub pos: Point<f32>,
    pub size: f32,
    pub colors: [[f32; 4]; 4],
    pub speed: f32,
    pub creative: bool,
    pub admin: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            pos: Point { x: 0.0, y: 0.0 },
            size: DEFAULT_PLAYER_SIZE,
            colors: [
                [1.0, 1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0, 1.0],
            ],
            speed: DEFAULT_PLAYER_SPEED,
            creative: true,
            admin: true,
        }
    }
}

impl Player {
    pub fn to_primitives<T: TileData>(
        &self,
        renderer: &Renderer<T>,
    ) -> Vec<RoundedRectInstance> {
        let radius = self.size / 2.0;
        let point_rad = Point {
            x: radius,
            y: radius,
        };
        let start = self.pos - point_rad;
        let end = self.pos + point_rad;
        let mut start = renderer.world_to_pixel(start);
        let mut end = renderer.world_to_pixel(end);
        let y = end.y;
        end.y = start.y;
        start.y = y;

        vec![RoundedRect {
            top_left: UIPoint {
                anchor: Point::zero(),
                offset: start,
            },
            bottom_right: UIPoint {
                anchor: Point::zero(),
                offset: end,
            },
            radius: (end - start).x / 2.0,
            colors: self.colors,
            ..Default::default()
        }.to_primitive()]
    }
}
