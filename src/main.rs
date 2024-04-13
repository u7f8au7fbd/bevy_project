//#![windows_subsystem = "windows"]

use bevy::{prelude::*, window::*};
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin};
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
mod initialize;
fn main() {
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
                        visible: false, //非表示
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()), //デフォルトの画像処理をピクセルパーフェクトに設定
        )
        .insert_resource(ClearColor(Color::NONE)) //デフォルトの背景色を設定
        .insert_resource(Msaa::Off) //MSAAを無効化
        .add_plugins((
            initialize::InitializePlugin,       //初期処理
            ScreenDiagnosticsPlugin::default(), //診断情報を表示
            ScreenFrameDiagnosticsPlugin,       //フレームレートを表示
            InfiniteGridPlugin,                 //グリッドを表示
        ))
        //以上は固定
        .add_systems(Startup, set_camera) //カメラを生成
        .run();
}

#[derive(Component)]
struct MainCamera;
fn set_camera(mut commands: Commands) {
    //グリッドを生成
    commands.spawn(InfiniteGridBundle::default());
    //ライトを生成
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    //カメラを生成
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(16.0, 4.0, 16.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
