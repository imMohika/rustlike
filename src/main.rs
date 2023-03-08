mod camera;
mod components;
mod map;
mod spawner;
mod systems;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 100;
    pub const SCREEN_HEIGHT: i32 = 80;
    pub const DISPLAY_WIDTH: i32 = 25;
    pub const DISPLAY_HEIGHT: i32 = 20;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();

        let map = Map::new();
        let camera = Camera::new(Point::new(10, 10));

        spawn_player(&mut ecs, Point::new(10, 10));

        resources.insert(map);
        resources.insert(camera);

        Self {
            ecs,
            resources,
            systems: build_scheduler(),
        }
    }

    fn clear_layers(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.clear_layers(ctx);

        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error")
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_fps_cap(30.0)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resourses/")
        .with_font("spritesheet.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "spritesheet.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "spritesheet.png")
        .build()?;
    main_loop(context, State::new())
}
