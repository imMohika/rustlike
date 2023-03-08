use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::zero(),
        };

        // if delta.x == 0 && delta.y == 0 {
        //     return;
        // }

        let mut players = <&mut Point>::query().filter(component::<Player>());

        players.iter_mut(ecs).for_each(|position| {
            let new_position = *position + delta;
            println!("{} {}", new_position.x, new_position.y);
            if map.can_enter_tile(new_position) {
                *position = new_position;
                camera.update_position(new_position);
            }
        })
    }
}
