extern crate amethyst;
use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

struct GameplayState {
    play_count: u8,
}

struct PausedState;

// GameData is the internal shared data between states
impl State<GameData<'static, 'static>, ()> for GameplayState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("Number of players: {}", self.play_count);
    }
}

fn main() -> amethyst::Result<()> {
    // amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    Ok(())
}
