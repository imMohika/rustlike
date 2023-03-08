use crate::prelude::{DISPLAY_HEIGHT, DISPLAY_WIDTH, Player, Point};

pub struct Camera {
    pub y_start: i32,
    pub y_end: i32,
    pub x_start: i32,
    pub x_end: i32,
}

impl Camera {
    pub fn new(position: Point) -> Self {
        Self {
            y_start: position.y - DISPLAY_HEIGHT / 2,
            y_end: position.y + DISPLAY_HEIGHT / 2,
            x_start: position.x - DISPLAY_WIDTH / 2,
            x_end: position.x + DISPLAY_WIDTH / 2,
        }
    }

    pub fn update_position(&mut self, new_position: Point) {
        self.y_start = new_position.y - DISPLAY_HEIGHT / 2;
        self.y_end= new_position.y + DISPLAY_HEIGHT / 2;
        self.x_start= new_position.x - DISPLAY_WIDTH / 2;
        self.x_end= new_position.x + DISPLAY_WIDTH / 2;
    }
}

