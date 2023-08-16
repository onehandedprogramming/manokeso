use std::time::Duration;

use crate::{util::point::Point, world::BoardSlice};

pub struct BoardView {
    pub pos: Point<f32>,
    pub slice: BoardSlice,
    pub connex_numbers: Vec<u32>,
    pub stability: Vec<f32>,
    pub reactivity: Vec<f32>,
    pub energy: Vec<f32>,
    pub alpha: Vec<f32>,
    pub beta: Vec<f32>,
    pub gamma: Vec<f32>,
    pub delta: Vec<f32>,
    pub omega: Vec<f32>,
    pub dirty: bool,
    pub total_energy: f32,
    pub time_taken: Duration,
}

impl BoardView {
    pub fn empty() -> Self {
        Self {
            pos: Point { x: 0.0, y: 0.0 },
            slice: BoardSlice::default(),
            connex_numbers: Vec::new(),
            stability: Vec::new(),
            reactivity: Vec::new(),
            energy: Vec::new(),
            alpha: Vec::new(),
            beta: Vec::new(),
            gamma: Vec::new(),
            delta: Vec::new(),
            omega: Vec::new(),
            dirty: false,
            total_energy: 0.0,
            time_taken: Duration::ZERO
        }
    }
}

#[derive(Clone, Copy)]
pub struct ClientView {
    pub pos: Point<f32>,
    pub width: f32,
    pub height: f32,
    pub paused: bool,
}

impl ClientView {
    pub fn new() -> Self {
        Self {
            pos: Point::new(0.0, 0.0),
            width: 0.0,
            height: 0.0,
            paused: false,
        }
    }
}