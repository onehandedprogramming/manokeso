use super::{
    camera::Camera,
    config::Config,
    input::Input,
    keybinds::{default_keybinds, Keybinds},
    player::Player,
    ui::{layout, ui::GameUI},
};
use crate::{
    common::interface::WorkerInterface,
    render::Renderer,
    rsc::{FPS, FRAME_TIME},
    tile_render_data,
    util::{point::Point, timer::Timer},
};
use std::time::{Duration, Instant};
use winit::event_loop::ActiveEventLoop;

tile_render_data!(TileRenderData, TileUpdateData, [
    0 => connex_numbers:u32:Uint32,
    1 => stability:f32:Float32,
    2 => reactivity:f32:Float32,
    3 => energy:f32:Float32,
    4 => omega:f32:Float32,
    5 => gamma:f32:Float32,
    6 => delta:u64:Uint32x2,
]);

pub const TILE_SHADER: &str = include_str!("./rsc/tile.wgsl");

pub struct Client<'a> {
    pub state: ClientState,
    pub renderer: Renderer<'a, TileRenderData>,
    pub ui: GameUI,
    pub keybinds: Keybinds,
    pub frame_time: Duration,
    pub hovered_tile: Option<TileInfo>,
    pub paused: bool,
    pub timer: Timer,
    pub worker: WorkerInterface,
    pub debug: DebugState,
    pub view_dirty: bool,
    pub exit: bool,
    pub input: Input,
    pub target: Instant,
    pub last_update: Instant,
    pub resized: bool,
}

impl Client<'_> {
    pub fn new(config: Config, event_loop: &ActiveEventLoop, worker: WorkerInterface) -> Self {
        let mut keybinds = default_keybinds();
        if let Some(config_keybinds) = config.keybinds {
            keybinds.extend(config_keybinds);
        }
        let fullscreen = config.fullscreen.unwrap_or(false);
        Self {
            state: ClientState::new(),
            renderer: Renderer::new(event_loop, TILE_SHADER, fullscreen),
            keybinds,
            frame_time: FRAME_TIME,
            hovered_tile: None,
            paused: true,
            timer: Timer::new(Duration::from_secs(1), FPS as usize),
            worker,
            ui: layout::board(),
            debug: DebugState::new(),
            view_dirty: false,
            exit: false,
            input: Input::new(),
            last_update: Instant::now(),
            target: Instant::now(),
            resized: false,
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct ClientState {
    pub selected_tile: Option<TileInfo>,
    pub camera: Camera,
    pub camera_scroll: f32,
    pub player: Player,
}

impl ClientState {
    pub fn new() -> Self {
        Self {
            camera: Camera::default(),
            camera_scroll: 0.0,
            selected_tile: None,
            player: Player::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct TileInfo {
    pub pos: Point<usize>,
    pub connex_number: u32,
    pub stability: f32,
    pub reactivity: f32,
    pub energy: f32,
    pub alpha: u64,
    pub beta: u64,
    pub gamma: f32,
    pub delta: u64,
    pub omega: f32,
}

pub struct DebugState {
    pub last_update: Instant,
    pub period: Duration,
    pub client_update_time: f32,
    pub board_update_time: f32,
    pub show: bool,
}

impl DebugState {
    pub fn new() -> Self {
        Self {
            last_update: Instant::now(),
            period: Duration::from_secs_f32(0.5),
            client_update_time: 0.0,
            board_update_time: 0.0,
            show: false,
        }
    }
}
