extern crate amethyst;
use amethyst::{
    core::transform::Transform,
    prelude::*,
    //renderer is used to display a window
    renderer::{
        Camera,
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    //needed for application_root_dir() etc
    utils::application_root_dir,
};

pub const ARENA_HEIGHT: f32 = 100.00;
pub const ARENA_WIDTH: f32 = 100.00;

struct GameplayState {
    lifeforms: u8,
}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    // Camera position
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);
    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

//GameData is the internal shared data between states
impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Number of lifeforms: {}", self.lifeforms);
        let world = data.world;
        initialise_camera(world);
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?;

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir,
                                    GameplayState{lifeforms:0}, game_data)?;
    game.run();
    Ok(())
}