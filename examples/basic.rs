use bevy::log::info;
use bevy::prelude::{App, AppExtStates, Commands, NextState, ResMut, Startup, States, Trigger};
use bevy::time::TimerMode;
use bevy::DefaultPlugins;
use bevy_auto_timer::{AutoTimer, AutoTimerFinished, AutoTimerPlugin};

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(AutoTimerPlugin::new(vec![GameState::InGame]))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    commands
        .spawn(AutoTimer::from_seconds(1., TimerMode::Repeating))
        .observe(timeout);

    next_state.set(GameState::InGame);
}

fn timeout(_: Trigger<AutoTimerFinished>) {
    info!("Timer finished!");
}
