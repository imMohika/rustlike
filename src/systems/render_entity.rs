use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn render_entity(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut entities = <(&Point, &Render)>::query();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    entities.iter(ecs).for_each(|(position, render)| {
        let new_position = *position - Point::new(camera.x_start, camera.y_start);

        draw_batch.set(new_position, render.color, render.glyph);
    });

    draw_batch.submit(80 * 50 + 1000).expect("Batch error");
}
