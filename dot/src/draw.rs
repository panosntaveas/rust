use piston_window::{PistonWindow, WindowSettings};
use piston_window::{Context, G2d};
use piston_window::{rectangle, clear};
use piston_window::types::Color;
use piston_window::color::{BLACK, WHITE};

use crate::ball::Ball;
use crate::terrain::Terrain;

const BLOCK_SIZE:f64 = 25.0;

pub enum Col {
    White,
}

pub fn color(color: Col) -> Color {
    match color {
        Col::White => WHITE,
    }
}

pub fn clear_screen(g: &mut G2d) {
    clear(BLACK, g);
}

pub fn get_window(name: &str, width: u32, height: u32) -> PistonWindow {
    WindowSettings::new(name, [width, height]).exit_on_esc(true).build().unwrap()
}

pub fn draw_rectangle(color: Color, x: f64, y:f64, c: Context, g: &mut G2d) {
    rectangle(color, [x, y, BLOCK_SIZE, BLOCK_SIZE], c.transform, g);
}

impl Ball {
    pub fn draw(&self, color: Color, c: Context, g: &mut G2d) {
        draw_rectangle(color, f64::from(self.x), f64::from(self.y), c, g)
    }
}

impl Terrain {
    pub fn draw(&self, color: Color, c: Context, g: &mut G2d) {
        for w in (self.width_start..self.width+1).step_by(BLOCK_SIZE as usize) {
            draw_rectangle(color, f64::from(w), f64::from(self.height_start), c, g);
            draw_rectangle(color, f64::from(w), f64::from(self.height), c, g);
        }
        for h in (self.height_start..self.height+1).step_by(BLOCK_SIZE as usize) {
            draw_rectangle(color, f64::from(self.width_start), f64::from(h), c, g);
            draw_rectangle(color, f64::from(self.width), f64::from(h), c, g);
        }
    } 
}