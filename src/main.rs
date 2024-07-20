//#![windows_subsystem = "windows"]

#[macro_use]
mod macros;

use bevy::{prelude::*, window::*};
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin};
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
fn main() {
    cmd!(utf8); //UTF-8を有効化
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy".into(),                                          //タイトル
                        resolution: (1280.0, 720.0).into(), //ウィンドウサイズ
                        position: WindowPosition::Centered(MonitorSelection::Primary), //ウィンドウの生成座標を中心に設定
                        present_mode: PresentMode::AutoNoVsync, //Vsyncを無効化
                        resizable: false,                       //サイズ変更不可
                        enabled_buttons: bevy::window::EnabledButtons {
                            minimize: false, //最小化無効
                            maximize: false, //最大化無効
                            close: true,     //閉じる有効
                        },
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()), //デフォルトの画像処理をピクセルパーフェクトに設定
        )
        .insert_resource(ClearColor(Color::NONE)) //デフォルトの背景色を設定
        .insert_resource(Msaa::Off) //MSAAを無効化
        .add_plugins((
            ScreenDiagnosticsPlugin::default(), //診断情報を表示
            ScreenFrameDiagnosticsPlugin,       //フレームレートを表示
            InfiniteGridPlugin,                 //グリッドを表示
        ))
        //以上は固定
        .add_systems(Startup, summon)
        .add_systems(Update, (move_player, move_camera))
        .run();
}

#[derive(Component)]
pub struct Player;

pub fn summon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(InfiniteGridBundle::default());
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Player
    let default_fov = 70.53_f32.to_radians(); //デフォルトの垂直視野角70.53度(水平視野角は103度)
    commands
        .spawn((
            InheritedVisibility::default(), //描画オブジェクト
            Player,
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 4.0, 40.0),
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: default_fov,
                    ..default()
                }),
                ..default()
            },
            FogSettings {
                color: Color::srgba(0., 0., 0., 1.0),
                falloff: FogFalloff::Linear {
                    start: 20.0,
                    end: 48.0,
                },
                ..default()
            },
            SpatialListener {
                left_ear_offset: Vec3::new(0.1, 0.0, 0.0),
                right_ear_offset: Vec3::new(-0.1, 0.0, 0.0),
            },
        ))
        .with_children(|interacter| {
            interacter.spawn((PbrBundle {
                mesh: meshes.add(Cuboid::new(0.05, 0.05, 0.05)),
                material: materials.add(Color::srgb(1., 1., 1.)),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, -4.0),
                    ..default()
                },
                ..default()
            },));
        });
}

fn move_player(
    mut query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let speed = 12.;
    for mut transform in query.iter_mut() {
        let rotation = transform.rotation;
        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.translation -= rotation * Vec3::Z * speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            transform.translation += rotation * Vec3::Z * speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.translation -= rotation * Vec3::X * speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.translation += rotation * Vec3::X * speed * time.delta_seconds();
        }
    }
}

fn move_camera(
    mut query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let sensitivity = 2.;
    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.rotation *= Quat::from_rotation_y(-sensitivity * time.delta_seconds());
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.rotation *= Quat::from_rotation_y(sensitivity * time.delta_seconds());
        }
    }
}
