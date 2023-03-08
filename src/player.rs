use crate::camera::Camera;
use crate::map::Map;
use crate::prelude::{BLACK, BTerm, Point, to_cp437, VirtualKeyCode, YELLOW};

pub struct Player {
    pub position: Point
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self {
            position
        }
    }

    pub fn walk(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1,0),
                VirtualKeyCode::Right => Point::new(1,0),
                VirtualKeyCode::Up => Point::new(0,-1),
                VirtualKeyCode::Down => Point::new(0,1),
                _ => Point::zero()
            };
            let new_position = self.position + delta;

            if map.can_enter_tile(new_position) {
                self.position = new_position;
            }
        }
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map, camera: &mut Camera) {
        self.walk(ctx, &map);
        camera.update_position(self.position);
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(1);

        let x = self.position.x - camera.x_start;
        let y = self.position.y - camera.y_start;

        ctx.set(x,y, YELLOW,BLACK, to_cp437('@'))
    }
}