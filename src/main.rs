use rltk::{GameState, Rltk};

const BARTITLE: &'static str = "Rusty-like game";

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Welcome Adventurer!");
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title(BARTITLE)
        .build()?;
    let gs = State {};
    rltk::main_loop(context, gs)
}
