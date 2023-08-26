use std::f32::consts::PI;

use ggez::event::EventHandler;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::{DrawMode, DrawParam};
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyMods;
use ggez::miniquad::GraphicsContext;
use ggez::Context;
use ggez::GameResult;
use good_web_game::mint::Point2;

use crate::color;
use crate::debug::draw_debug_text;

const COLORS: &[Color] = &[
    color!(RED),
    color!(GREEN),
    color!(BLUE),
    // color!(CYAN),
    // color!(YELLOW),
    // color!(MAGENTA),
];
const WIDTH: f32 = 3.0;
const HEIGHT: f32 = 30.0;
const SPEED_EXPONENT: f32 = 1.3;
const SPEED_MULTIPLY: f32 = 0.01;

#[derive(Default)]
pub struct App {
    frame_count: u32,
    show_debug: bool,
}

impl App {
    pub fn new(_ctx: &mut Context, _quad_ctx: &mut GraphicsContext) -> GameResult<Self> {
        Ok(Self::default())
    }
}

impl EventHandler for App {
    fn update(&mut self, _ctx: &mut Context, _quad_ctx: &mut GraphicsContext) -> GameResult {
        self.frame_count += 1;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut GraphicsContext) -> GameResult {
        graphics::clear(ctx, quad_ctx, color!(BLACK));

        let canvas = graphics::Canvas::with_window_size(ctx, quad_ctx)?;

        // Center of canvas
        let mut point = Point2 {
            x: canvas.width() as f32 / 2.0,
            y: canvas.height() as f32 / 2.0,
        };

        for (i, color) in COLORS.into_iter().enumerate() {
            // Amount of arms from center, as float
            let alpha = (COLORS.len() - i) as f32;
            // Amount of arms from end, as float
            let omega = (i + 1) as f32;

            // Arm properties
            let width = alpha * WIDTH;
            let length = alpha * HEIGHT;
            let rotation = self.frame_count as f32 * omega.powf(SPEED_EXPONENT) * SPEED_MULTIPLY;

            // Points for arm line
            let points = [Point2 { x: 0.0, y: 0.0 }, Point2 { x: 0.0, y: length }];
            // Use rotation transformation
            let param = DrawParam::new().dest(point).rotation(rotation - PI / 2.0);

            // Draw arm line
            let mesh = graphics::MeshBuilder::new()
                .line(&points, width, *color)?
                .build(ctx, quad_ctx)?;
            graphics::draw(ctx, quad_ctx, &mesh, param)?;

            // Draw circle (line cap) at start of arm
            let mesh = graphics::Mesh::new_circle(
                ctx,
                quad_ctx,
                DrawMode::fill(),
                point,
                width / 2.0,
                0.1,
                *color,
            )?;
            graphics::draw(ctx, quad_ctx, &mesh, DrawParam::default())?;

            // Move point to end of arm
            point.x += length * rotation.cos();
            point.y += length * rotation.sin();

            // Draw circle (line cap) at end of arm
            let mesh = graphics::Mesh::new_circle(
                ctx,
                quad_ctx,
                DrawMode::fill(),
                point,
                width / 2.0,
                0.1,
                *color,
            )?;
            graphics::draw(ctx, quad_ctx, &mesh, DrawParam::default())?;
        }

        // Display debug information
        if self.show_debug {
            draw_debug_text(
                ctx,
                quad_ctx,
                [format!("Total frames: {}", self.frame_count)],
            )?;
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _quad_ctx: &mut GraphicsContext,
        keycode: KeyCode,
        keymod: KeyMods,
        _repeat: bool,
    ) {
        use KeyCode::*;

        match (keymod, keycode) {
            (KeyMods::NONE, F3) => self.show_debug ^= true,
            _ => (),
        }
    }
}
