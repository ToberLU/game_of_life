use std::ops::Mul;

use raylib::color::Color; //use raylib::consts::KeyboardKey;
use raylib::drawing::RaylibDraw;
use raylib::ffi::KeyboardKey;
use raylib::{RaylibHandle, RaylibThread};
use tracing::instrument;

use crate::simulation::Simulation;
//use raylib::{RaylibHandle};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;
const SPACING: i32 = 0;
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

    #[instrument(skip(simulation, draw))]
    fn draw_grid(draw: &mut impl RaylibDraw, simulation: &Simulation) {
        let spaces_width = (simulation.grid.width as i32 - 1) * SPACING;
        let spaces_height = (simulation.grid.height as i32 - 1) * SPACING;
        let window_width = WIDTH - 2 * BORDER;
        let window_height = HEIGHT - 2 * BORDER;
        let cell_width = (window_width - spaces_width) / simulation.grid.width as i32;
        let cell_height = (window_height - spaces_height) / simulation.grid.height as i32;

        draw.draw_rectangle_lines(
            BORDER - 1,
            BORDER - 1,
            window_width + 1,
            window_height + 1,
            Color::WHITE,
        );

        for row in 0..simulation.grid.height {
            for col in 0..simulation.grid.width {
                //let color = match simulation.grid.cells[row * simulation.grid.width + col] {
                //    true => Color::BLUE,
                //    _ => Color::BLACK,
                //}
                let color = match (simulation.grid.cells[row * simulation.grid.width + col], simulation.grid.count_live_neighbors(row, col)) {
                    (false, 3) => Color::DARKGRAY,
                    (true, 2 | 3) => Color::BLUE,
                    (true, _) => Color::DARKBLUE,
                    (false, _) => Color::BLACK
                };
                draw.draw_rectangle(
                    BORDER + (cell_width + SPACING) * col as i32,
                    BORDER + (cell_height + SPACING) * row as i32,
                    cell_width,
                    cell_height,
                    color,
                );
            }
        }
    }

    #[instrument(skip(self, simulation))]
    pub fn run(&mut self, simulation: &mut Simulation) {
        tracing::info!("start render loop...");

        let mut time = std::time::Instant::now();
        while !self.raylib_handle.window_should_close() {
            if self.raylib_handle.is_key_pressed(KeyboardKey::KEY_SPACE) {
                simulation.paused = !simulation.paused;
            }
            if self.raylib_handle.is_key_pressed(KeyboardKey::KEY_R) {
                simulation.grid.randomize(simulation.grid.width * simulation.grid.height / 20);
            }
            if self.raylib_handle.is_key_pressed(KeyboardKey::KEY_MINUS) {
                simulation.delay_ms = simulation.delay_ms.mul(2);
            }
            if self.raylib_handle.is_key_pressed(KeyboardKey::KEY_EQUAL) {
                simulation.delay_ms = simulation.delay_ms.div_f32(2.);
            }
            let mut draw = self.raylib_handle.begin_drawing(&self.raylib_thread);
            if !simulation.paused && time.elapsed() > simulation.delay_ms {
                simulation.update();
                time = std::time::Instant::now();
            }
            draw.clear_background(Color::BLACK);
            RenderContext::draw_grid(&mut draw, simulation);
        }
    }
}
