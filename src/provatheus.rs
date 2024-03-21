use bevy::{
    app::PluginGroupBuilder,
    core::*,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use std::process::Command;

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

pub fn summon_grid(mut commands: Commands, asset_server: Res<AssetServer>) {
    let grid_size = 16.;
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., 0., -256.),
                scale: Vec3::new(16., 16., 0.),
                ..default()
            },
            texture: asset_server.load("provatheus/grid.png"),
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 1. / grid_size,
        },
    ));
}

#[derive(Component)]
pub struct FpsText;

pub fn set_text(mut commands: Commands) {
    //テキストの生成
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font_size: 20.0,
                    ..default()
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font_size: 20.0,
                    ..default()
                },
            ),
            TextSection::new(
                "\nFT: ",
                TextStyle {
                    font_size: 20.0,
                    ..default()
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font_size: 20.0,
                    ..default()
                },
            ),
        ]),
        FpsText,
    ));
}

pub fn update_text(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        let fps_date = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS);
        if let Some(fps) = fps_date.unwrap().smoothed() {
            text.sections[1].value = format!("{:.2}", fps);
        }

        let ft_date = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME);
        if let Some(ft) = ft_date.unwrap().smoothed() {
            text.sections[3].value = format!("{:.2}", ft);
        }
    }
}

pub struct ProvatheusPlugin;

impl PluginGroup for ProvatheusPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(UpdatePlugin)
            .add(StartupPlugin)
    }
}

pub struct UpdatePlugin;

impl Plugin for UpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (enable_visible, update_text));
    }
}

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (set_utf8, set_text, summon_grid));
    }
}
