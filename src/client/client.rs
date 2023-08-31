use super::{
    camera::Camera,
    config::Config,
    keybinds::{default_keybinds, Keybinds},
    player::Player,
    ui::{layout, ui::GameUI},
};
use crate::{
    board::BoardSettings,
    common::{
        interface::WorkerInterface,
        view::{BoardId, BoardView},
    },
    render::Renderer,
    rsc::{FPS, FRAME_TIME},
    tile_render_data,
    util::{point::Point, timer::Timer},
};
use std::time::{Duration, Instant};
use winit::event_loop::EventLoop;

tile_render_data!(TileRenderData, TileUpdateData, [
    0 => connex_numbers:u32,
    1 => stability:f32,
    2 => reactivity:f32,
    3 => energy:f32,
]);

pub const TILE_SHADER: &str = include_str!("./rsc/tile.wgsl");

pub struct Client {
    pub state: ClientState,
    pub renderer: Renderer<TileRenderData>,
    pub ui: GameUI,
    pub keybinds: Keybinds,
    pub frame_time: Duration,
    pub hovered_tile: Option<TileId>,
    pub paused: bool,
    pub timer: Timer,
    pub worker: WorkerInterface,
    pub debug: DebugState,
    pub view_dirty: bool,
    pub exit: bool,
}

impl Client {
    pub async fn new(config: Config, event_loop: &EventLoop<()>, worker: WorkerInterface) -> Self {
        let mut keybinds = default_keybinds();
        if let Some(config_keybinds) = config.keybinds {
            keybinds.extend(config_keybinds);
        }
        let fullscreen = config.fullscreen.unwrap_or(false);

        let width = 50;
        let height = 50;
        let settings = BoardSettings {
            pos: Point::new(-(width as f32) / 2.0, -(height as f32) / 2.0),
            width,
            height,
        };
        let main_id = 0;
        worker.send(crate::common::message::WorkerCommand::CreateBoard(settings));
        let settings = BoardSettings {
            pos: Point::new((width as f32) / 2.0, -(height as f32) / 2.0),
            width,
            height,
        };
        worker.send(crate::common::message::WorkerCommand::CreateBoard(settings));

        Self {
            state: ClientState::new(main_id),
            renderer: Renderer::new(event_loop, TILE_SHADER, fullscreen).await,
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
        }
    }
}

impl Client {
    pub fn get_tile(&self, id: TileId) -> Option<TileView<'_>> {
        if let Some(view) = self.worker.get(id.board_id) {
            Some(TileView {
                board_id: id.board_id,
                view,
                i: (id.pos - view.slice.start).index(view.slice.width),
                pos: id.pos,
            })
        } else {
            None
        }
    }

    pub fn hovered_tile(&self) -> Option<TileView<'_>> {
        self.get_tile(self.hovered_tile?)
    }

    pub fn selected_tile(&self) -> Option<TileView<'_>> {
        self.get_tile(self.state.selected_tile?)
    }
}

pub struct TileView<'a> {
    pub board_id: BoardId,
    pub pos: Point<usize>,
    pub view: &'a BoardView,
    pub i: usize,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct ClientState {
    pub selected_tile: Option<TileId>,
    pub camera: Camera,
    pub camera_scroll: f32,
    pub player: Player,
    pub main_id: BoardId,
}

impl ClientState {
    pub fn new(main_id: BoardId) -> Self {
        Self {
            camera: Camera::default(),
            camera_scroll: 0.0,
            selected_tile: None,
            player: Player::default(),
            main_id,
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct TileId {
    pub board_id: BoardId,
    pub pos: Point<usize>,
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
            show: true,
        }
    }
}
