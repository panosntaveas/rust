pub struct Terrain {
    pub width_start: u32,
    pub width: u32,
    pub height_start: u32,
    pub height: u32,
}

impl Terrain {
    pub fn new(width: u32, height: u32) -> Terrain {
        Terrain {
            width_start: 0,
            height_start: 0,
            width,
            height,
        }
    }
}