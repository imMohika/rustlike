use crate::prelude::*;

#[system]
pub fn render_map(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    for y in camera.y_start..camera.y_end {
        for x in camera.x_start..camera.x_end {
            if map.in_bounds(Point::new(x, y)) {
                let index = map.tile_index(Point::new(x, y));
                let glyph = match map.tiles[index] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };

                let new_x = x - camera.x_start;
                let new_y = y - camera.y_start;

                draw_batch.set(
                    Point::new(new_x, new_y),
                    ColorPair::new(WHITE, BLACK),
                    glyph,
                );
            }
        }
    }

    draw_batch.submit(0).expect("Batch error");
}
