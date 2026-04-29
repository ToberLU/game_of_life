use raylib::color::Color; //use raylib::consts::KeyboardKey;
use raylib::drawing::RaylibDraw;
use raylib::{RaylibHandle, RaylibThread};
use tracing::instrument;
//use raylib::{RaylibHandle};

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 800;
const FPS: u32 = 100;

pub struct RenderContext {
    raylib_handle: RaylibHandle,
    raylib_thread: RaylibThread,
}

impl RenderContext {
    #[instrument]
    pub fn new() -> Self {
        tracing::info!("init raylib");
        let (mut rl, thread) = raylib::init()
            .size(WIDTH, HEIGHT)
            .title("Game of Life")
            .build();

        tracing::info!("set fps {FPS}");
        rl.set_target_fps(FPS);
        Self {
            raylib_handle: rl,
            raylib_thread: thread,
        }
    }

    #[instrument(skip(self))]
    pub fn run(&mut self) {
        tracing::info!("start render loop...");

        while !self.raylib_handle.window_should_close() {
            let mut draw = self.raylib_handle.begin_drawing(&self.raylib_thread);

            draw.clear_background(Color::BLACK);
            draw.draw_text("Game of Life", 12, 12, 20, Color::WHITE);
        }
    }
}
