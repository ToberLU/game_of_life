use raylib::color::Color; //use raylib::consts::KeyboardKey;
use raylib::drawing::RaylibDraw;
use raylib::{RaylibHandle, RaylibThread};
use tracing::instrument;

use crate::simulation::{self, Simulation};
//use raylib::{RaylibHandle};

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 800;
const SPACING: i32 = 2;
const BORDER: i32 = 10;
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

    #[instrument(skip(self, simulation, draw))]
    fn draw_grid(&mut self, draw: &mut impl RaylibDraw, simulation: &Simulation) {
        let spaces_width = (simulation.grid.width as i32 - 1) * SPACING;
        let spaces_height = (simulation.grid.height as i32 - 1) * SPACING;
        let window_width = WIDTH - 2 * BORDER;
        let window_height = HEIGHT - 2 * BORDER;

        draw.draw_rectangle(
            0 + BORDER,
            0 + BORDER,
            window_width,
            window_height,
            Color::WHITE,
        );
    }

    #[instrument(skip(self, simulation))]
    pub fn run(&mut self, simulation: &mut Simulation) {
        tracing::info!("start render loop...");

        while !self.raylib_handle.window_should_close() {
            let mut draw = self.raylib_handle.begin_drawing(&self.raylib_thread);
            if !simulation.paused {
                simulation.update();
                self.draw_grid(&draw, simulation);
            }

            draw.clear_background(Color::BLACK);
            draw.draw_text("Game of Life", 12, 12, 20, Color::WHITE);
        }
    }
}
