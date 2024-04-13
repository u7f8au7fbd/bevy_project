use std::process::Command;

use bevy::{
    app::{App, Plugin, PluginGroup, PluginGroupBuilder, Startup, Update},
    core::FrameCount,
    ecs::system::{Query, Res},
    window::Window,
};

pub fn set_utf8() {
    Command::new("cmd")
        .args(["/C", "chcp 65001"])
        .output()
        .expect("UTF-8に設定できませんでした");
}

pub fn enable_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == 3 {
        window.single_mut().visible = true;
    }
}

pub struct InitializePlugin;

impl PluginGroup for InitializePlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(UpdatePlugin)
            .add(StartupPlugin)
    }
}

pub struct UpdatePlugin;

impl Plugin for UpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enable_visible);
    }
}

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_utf8);
    }
}
