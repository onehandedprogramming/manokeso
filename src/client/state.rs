use super::{
    camera::Camera,
    config::Config,
    keybinds::{default_keybinds, Keybinds},
    player::Player,
    ui::{layout, ui::GameUI},
};
use crate::{
    message::{ClientMessage, WorldMessage},
    render::Renderer,
    rsc::{FPS, FRAME_TIME},
    sync::{BoardView, TileInfo, WorldInterface},
    tile_render_data,
    util::timer::Timer,
};
use std::{
    sync::mpsc::{Receiver, Sender},
    time::{Duration, Instant},
};
use winit::event_loop::EventLoop;

tile_render_data!(TileRenderData, TileUpdateData, [
    0 => connex_numbers:u32,
    1 => stability:f32,
    2 => reactivity:f32,
    3 => energy:f32,
]);

pub const TILE_SHADER: &str = include_str!("./rsc/tile.wgsl");

pub struct ClientState {
    pub renderer: Renderer<TileRenderData>,
    pub ui: GameUI,
    pub keybinds: Keybinds,
    pub frame_time: Duration,
    pub camera: Camera,
    pub camera_scroll: f32,
    pub selected_tile: Option<TileInfo>,
    pub hovered_tile: Option<TileInfo>,
    pub paused: bool,
    pub timer: Timer,
    pub world: WorldInterface,
    pub player: Player,
    pub debug_stats: DebugStats,
    pub last_debug: Instant,
    pub exit: bool,
    pub debug: bool,
}

impl ClientState {
    pub async fn new(
        config: Config,
        event_loop: &EventLoop<()>,
        sender: Sender<ClientMessage>,
        receiver: Receiver<WorldMessage>,
    ) -> Self {
        let mut keybinds = default_keybinds();
        if let Some(config_keybinds) = config.keybinds {
            keybinds.extend(config_keybinds);
        }
        let camera = Camera::default();
        let view = BoardView::empty();
        let fullscreen = config.fullscreen.unwrap_or(false);
        Self {
            renderer: Renderer::new(event_loop, TILE_SHADER, fullscreen).await,
            keybinds,
            frame_time: FRAME_TIME,
            camera,
            camera_scroll: 0.0,
            selected_tile: None,
            hovered_tile: None,
            paused: true,
            timer: Timer::new(Duration::from_secs(1), FPS as usize),
            world: WorldInterface {
                sender,
                receiver,
                view,
            },
            ui: layout::board(),
            debug_stats: DebugStats::new(),
            player: Player::default(),
            last_debug: Instant::now(),
            debug: true,
            exit: false,
        }
    }
}

pub struct DebugStats {
    pub period: Duration,
    pub client_update_time: f32,
    pub world_update_time: f32,
}

impl DebugStats {
    pub fn new() -> Self {
        Self {
            period: Duration::from_secs_f32(0.5),
            client_update_time: 0.0,
            world_update_time: 0.0,
        }
    }
}
