use std::time::{Instant, SystemTime};

use itertools::Itertools;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    board::{BoardWorker, get_bit},
    common::{
        interface::interface_pair,
        message::{WorkerCommand, WorkerResponse},
    },
};

use super::{client::Client, config::Config, input::Input, TileUpdateData};

impl Client {
    pub async fn run() {
        let worker_thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads((rayon::current_num_threads() - 1).max(1))
            .build()
            .unwrap();

        let event_loop = EventLoop::new();
        let (wi, ci) = interface_pair();
        let mut client = Client::new(Config::load(), &event_loop, wi).await;

        worker_thread_pool.spawn(move || {
            BoardWorker::new(ci).run();
        });

        let mut target = Instant::now();
        let mut last_update = Instant::now();
        let mut input = Input::new();
        let mut resized = false;

        client.renderer.window.set_visible(true);

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event, window_id }
                    if window_id == client.renderer.window.id() =>
                {
                    match event {
                        WindowEvent::CloseRequested => client.exit = true,
                        WindowEvent::Resized(_) => resized = true,
                        _ => input.update(event),
                    }
                }
                Event::RedrawRequested(_) => {
                    client.renderer.start_encoder();
                    client.renderer.draw();
                }
                Event::MainEventsCleared => {
                    let now = Instant::now();
                    if now > target {
                        target += client.frame_time;

                        client.timer.start();

                        let time_delta = now - last_update;
                        last_update = now;

                        client.receive_messages();
                        client.handle_input(&time_delta, &input);
                        input.end();
                        client.update(&input, now);
                        client.render(resized);

                        resized = false;
                        if client.exit {
                            client.worker.send(WorkerCommand::Exit());
                            for _ in client.worker.receiver.iter() {}
                            *control_flow = ControlFlow::Exit;
                        }

                        client.timer.stop();
                    }
                }
                _ => {}
            }
        });
    }

    fn receive_messages(&mut self) {
        for msg in self.worker.receiver.try_iter() {
            match msg {
                WorkerResponse::ViewSwap(mut view) => {
                    std::mem::swap(&mut view, &mut self.worker.view);
                    self.worker.send(WorkerCommand::ViewSwap(view));
                    self.view_dirty = true;
                }
                WorkerResponse::Loaded(state) => {
                    self.state = state;
                    self.paused = true;
                }
            }
        }
    }

    fn render(&mut self, resized: bool) {
        self.renderer.start_encoder();
        let view = &mut self.worker.view;
        self.state.camera.pos = self.state.player.pos;
        if let Some(cam_view) = self.renderer.update(
            if self.view_dirty {
                Some(TileUpdateData {
                    slice: &view.slice,
                    connex_numbers: &view.bufs.connex_numbers,
                    stability: &view.bufs.stability,
                    reactivity: &view.bufs.reactivity,
                    energy: &view.bufs.energy,
                    omega: &view.bufs.omega,
                    gamma: &view.bufs.gamma,
                    delta: &view.bufs.delta,
                })
            } else {
                None
            },
            &self.state.camera,
            resized,
        ) {
            self.worker.send(WorkerCommand::CameraUpdate(cam_view));
        }
        self.view_dirty = false;
        let ui = self.ui.compile(&self);
        self.renderer.update_ui(&ui, resized);
        self.renderer.draw();
    }
}
