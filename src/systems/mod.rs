mod player_input;
mod render_entity;
mod render_map;

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(render_map::render_map_system())
        .add_system(render_entity::render_entity_system())
        .build()
}
