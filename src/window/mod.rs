mod buffer;
mod init;
mod render;
mod uniform;

use crate::Camera;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use self::{
    buffer::Instance,
    init::{init_renderer, init_surface},
    uniform::CameraUniform,
};

pub const CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.1,
    g: 0.1,
    b: 0.1,
    a: 1.0,
};

pub struct GameWindow {
    // window & device stuff
    pub window: Window,
    pub(super) surface: wgpu::Surface,
    pub(super) device: wgpu::Device,
    pub(super) queue: wgpu::Queue,
    // render stuff
    pub(super) render_pipeline: wgpu::RenderPipeline,
    pub(super) vertex_buffer: wgpu::Buffer,
    pub(super) index_buffer: wgpu::Buffer,
    pub(super) instances: Vec<Instance>,
    pub(super) instance_buffer: wgpu::Buffer,
    // camera
    pub(super) camera_uniform: CameraUniform,
    pub(super) camera_buffer: wgpu::Buffer,
    pub(super) camera_bind_group: wgpu::BindGroup,
}

impl GameWindow {
    pub async fn new(event_loop: &EventLoop<()>, camera: &Camera) -> GameWindow {
        let window = WindowBuilder::new()
            .with_visible(false)
            .build(&event_loop)
            .unwrap();

        let (surface, device, queue, config) = init_surface(&window).await;

        let (
            render_pipeline,
            vertex_buffer,
            index_buffer,
            instances,
            instance_buffer,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
        ) = init_renderer(&device, &config, &camera);

        Self {
            window,
            surface,
            device,
            queue,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            instances,
            instance_buffer,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
        }
    }
}