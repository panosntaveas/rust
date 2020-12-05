pub struct Ball {
    pub x: i32,
    pub y: i32,
    x_dir: i8,
    y_dir: i8,
}

impl Ball {
    pub fn new(x_start: u32, y_start:u32) -> Ball {
        Ball {
            x: x_start as i32,
            x_dir: 1,
            y: y_start as i32,
            y_dir: 1,
        }
    }

    pub fn update(&mut self, width_start: u32, width_end: u32, height_start: u32, height_end: u32) {
        if      self.x < width_start as i32 { self.x_dir =  1; }
        else if self.x > width_end as i32   { self.x_dir = -1; }
        self.x += self.x_dir as i32;
        if      self.y < height_start as i32 { self.y_dir =  1; }
        else if self.y > height_end as i32   { self.y_dir = -1; }
        self.y += self.y_dir as i32;
    }
}