use bevy::{
    app::App,
    prelude::{Plugin, Res, ResMut, Resource},
    time::{Time, Timer},
    DefaultPlugins,
};

#[derive(Resource)]
struct InitTimer(Timer);

fn init(time: Res<Time>, mut timer: ResMut<InitTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("initialized");
    }
}
pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(init);
    }
}

fn main() {
    App::new()
        .insert_resource(InitTimer(Timer::from_seconds(
            2.0,
            bevy::time::TimerMode::Repeating,
        )))
        .add_plugins(DefaultPlugins)
        .add_system(init)
        .run();
}
