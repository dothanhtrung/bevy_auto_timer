use bevy::log::info;
use bevy::prelude::{App, Commands, Startup, Trigger};
use bevy::time::TimerMode;
use bevy::DefaultPlugins;
use bevy_auto_timer::{AutoTimer, AutoTimerFinished, AutoTimerPluginAnyState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AutoTimerPluginAnyState::any())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn(AutoTimer::from_seconds(1., TimerMode::Repeating))
        .observe(timeout);
}

fn timeout(_: Trigger<AutoTimerFinished>) {
    info!("Timer finished!");
}
