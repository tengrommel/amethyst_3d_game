extern crate amethyst;
use amethyst::prelude::*;
use amethyst::input::{VirtualKeyCode, is_key_down};

struct GameplayState {
    // The `State` local data. Usually you will not have anything
    // In this case, we have the number of players here
    play_count: u8,
}

struct PausedState;

// This time around, we are using () instead of GameDate, because we don't have any `System's that need to be updated.
// (They are covered in the dedicated section of the book.)
// Instead of writing `State<(), StateEvent>`, we can instead use `EmptyState`.
impl EmptyState for GameplayState {
    fn handle_event(&mut self, _data: StateData<'_, ()>, event: StateEvent) -> EmptyTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                // Pause the game by going to the `PausedState`
                return Trans::Push(Box::new(PausedState));
            }
        }
        // Escape isn't pressed, so we stay in this `State`
        Trans::None
    }
}

impl EmptyState for PausedState {
    fn handle_event(&mut self, _data: StateDate<()>, event: StateEvent) -> EmptyTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                // Go back to the `GameplayState`.
                return Trans::Pop;
            }
        }
        Trans::None
    }
}


// GameData is the internal shared data between states
impl State<GameData<'static, 'static>> for GameplayState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("Number of players: {}", self.play_count);
    }
}

fn main() {

}
