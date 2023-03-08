use bracket_lib::prelude::{BError, BTerm, BTermBuilder, GameState, main_loop};


struct State{}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1,1,"num num");
    }
}
fn main() -> BError {
    let context = BTermBuilder::simple80x50().build()?;
    main_loop(context, State {})
}
