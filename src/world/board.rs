use std::time::Duration;

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;

use crate::rsc::{CONNEX_NUMBER_RANGE, CONDUCTIVITY_RANGE, REACTIVITY_RANGE, ENERGY_RANGE};

use super::swap_buffer::{SwapBuffer, SwapBufferGen};

const BASE_KERNEL: [[f32; 3]; 3] = [[0.5, 1.0, 0.5], [1.0, 2.0, 1.0], [0.5, 1.0, 0.5]];

pub struct Board {
    pub pos: [f32; 2],
    width: usize,
    height: usize,
    pub connex_numbers: SwapBuffer<u32>,
    pub conductivity: SwapBuffer<f32>,
    pub reactivity: SwapBuffer<f32>,
    pub energy: SwapBuffer<f32>,
    total_energy: f32,
}

impl Board {
    pub fn new(pos: [f32; 2], width: usize, height: usize) -> Board {
        let mut rng = ChaCha8Rng::seed_from_u64(0);
        let mut gen = (&mut rng, width, height);

        let connex_numbers = gen.rand_swap_buf(CONNEX_NUMBER_RANGE);
        let conductivity = gen.rand_swap_buf(CONDUCTIVITY_RANGE);
        let reactivity = gen.rand_swap_buf(REACTIVITY_RANGE);
        let energy = gen.rand_swap_buf(ENERGY_RANGE);

        let total_energy = energy.read().iter().sum();

        Board {
            pos,
            width,
            height,
            connex_numbers,
            conductivity,
            reactivity,
            energy,
            total_energy,
        }
    }

    pub fn update(&mut self, delta: &Duration) {
        let d = delta.as_secs_f32();

        let (er, ew) = self.energy.bufs();
        let (cr, ..) = self.conductivity.bufs();
        self.total_energy = ew
            .par_iter_mut()
            .enumerate()
            .map(|(i, e)| {
                let x = i % self.width;
                let y = i / self.width;
                let mut sum = 0.;
                let mut weights = 0.;
                for dy in 0..=2 {
                    if y + dy >= 1 && y + dy - 1 < self.height {
                        for dx in 0..=2 {
                            if x + dx >= 1 && x + dx - 1 < self.width && !((dx == 0) & (dy == 0)) {
                                let i = (y + dy - 1) * self.width + x + dx - 1;
                                let cond = cr[i];
                                let a = BASE_KERNEL[dx][dy] * cond;
                                sum += a * er[i];
                                weights += a;
                            }
                        }
                    }
                }
                let t = sum / weights;
                let cur = er[i];
                let new = cur + (t - cur) * d;
                *e = new;
                new
            })
            .sum();
        self.energy.swap();
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn total_energy(&self) -> f32 {
        self.total_energy
    }

    pub fn tile_at(&self, pos: [f32; 2]) -> Option<[usize; 2]> {
        let x = pos[0] - self.pos[0] + 0.5;
        let y = pos[1] - self.pos[1] + 0.5;
        if x < 0.0 || y < 0.0 || x >= self.width as f32 || y >= self.height as f32 {
            None
        } else {
            Some([ x as usize, y as usize ])
        }
    }
}
