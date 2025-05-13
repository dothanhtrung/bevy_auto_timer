<div align="center">

bevy_auto_timer
===============

[![crates.io](https://img.shields.io/crates/v/bevy_auto_timer)](https://crates.io/crates/bevy_auto_timer)
[![docs.rs](https://docs.rs/bevy_auto_timer/badge.svg)](https://docs.rs/bevy_auto_timer)
[![dependency status](https://deps.rs/repo/gitlab/kimtinh/bevy_auto_timer/status.svg)](https://deps.rs/repo/gitlab/kimtinh/bevy_auto_timer)
[![pipeline status](https://gitlab.com/kimtinh/bevy_auto_timer/badges/master/pipeline.svg)](https://gitlab.com/kimtinh/bevy_auto_timer/-/commits/master)


[![Gitlab](https://img.shields.io/badge/gitlab-%23181717.svg?style=for-the-badge&logo=gitlab&logoColor=white)](https://gitlab.com/kimtinh/bevy_auto_timer)
[![Github](https://img.shields.io/badge/github-%23121011.svg?style=for-the-badge&logo=github&logoColor=white)](https://github.com/dothanhtrung/bevy_auto_timer)

</div>

A convenient timer which ticks automatically.

Quickstart
----------

```rust
use bevy::prelude::*;
use bevy_auto_timer::{AutoTimer, AutoTimerFinished, AutoTimerPlugin, AutoTimerPluginAnyState};

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    InGame,
    EndGame,
}

fn main() {
    let mut app = App::new().add_plugins(DefaultPlugins).init_state::<GameState>();

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
```

License
-------

Please see [LICENSE](./LICENSE).


Compatible Bevy Versions
------------------------

| bevy | bevy_auto_timer |
|------|-----------------|
| 0.16 | 0.1             |
