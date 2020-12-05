use crate::terrain::Terrain;
use crate::ball::Ball;

pub struct Game {
    pub terrain: Terrain,
    pub ball: Ball,
}

impl Game {
    pub fn new(width: u32, height: u32, pos_x: u32, pos_y: u32) -> Game {
        Game {
            terrain: Terrain::new(width, height),
            ball: Ball::new(pos_x, pos_y),
        }
    }

    pub fn update(&mut self) {
        self.ball.update(self.terrain.width_start, 
                         self.terrain.width, 
                         self.terrain.height_start, 
                         self.terrain.height);
    }
}