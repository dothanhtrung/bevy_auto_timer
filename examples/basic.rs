use bevy::prelude::*;
use bevy_auto_timer::{AutoTimer, AutoTimerFinished, AutoTimerPlugin};

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    InGame,
    EndGame,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins).init_state::<GameState>();

    // Add this plugin and specify which state it will run in
    app.add_plugins(AutoTimerPlugin::new(vec![GameState::InGame, GameState::EndGame]));
    // or you can use `AutoTimerPluginAnyState` for running on any states.
    // app.add_plugins(AutoTimerPluginAnyState::any());

    app.add_systems(Startup, setup).run();
}

fn setup(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    // Spawn timer as a component
    commands
        .spawn(AutoTimer::from_seconds(1., TimerMode::Repeating))
        .observe(timeout); // observe event `AutoTimerFinished`

    next_state.set(GameState::InGame);
}

fn timeout(_: Trigger<AutoTimerFinished>) {
    info!("Timer finished!");
}
