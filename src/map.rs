use crate::prelude::*;
use std::ops::{Index, IndexMut};

const TILES_COUNT: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
}

pub fn create_bounded_tiles(width: i32, height: i32) -> Vec<TileType> {
    let total_tiles = (width * height) as usize;
    let mut tiles = vec![TileType::Floor; total_tiles];

    for x in 0..width {
        tiles[x as usize] = TileType::Wall;
        tiles[total_tiles - (x as usize) - 1] = TileType::Wall;
    }

    for y in 0..height {
        let index = y * width;
        tiles[index as usize] = TileType::Wall;
        tiles[total_tiles - (index as usize) - 1] = TileType::Wall;
    }

    tiles
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: create_bounded_tiles(SCREEN_WIDTH, SCREEN_HEIGHT),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
        }
    }

    pub fn tile_index(&self, point: Point) -> usize {
        ((point.y * self.width) + point.x) as usize
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.width && point.y < self.height
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[self.tile_index(point)] == TileType::Floor
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);

        for y in camera.y_start..camera.y_end {
            for x in camera.x_start..camera.x_end {
                if self.in_bounds(Point::new(x,y)) {
                    let index = self.tile_index(Point::new(x, y));
                    let new_x = x - camera.x_start;
                    let new_y = y - camera.y_start;

                    match self.tiles[index] {
                        TileType::Floor => {
                            ctx.set(new_x, new_y, GRAY, BLACK, to_cp437('.'));
                        }
                        TileType::Wall => {
                            ctx.set(new_x, new_y, YELLOW, BLACK, to_cp437('#'));
                        }
                    }
                }
            }
        }
    }
}
