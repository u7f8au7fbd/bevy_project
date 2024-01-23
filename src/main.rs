//#![windows_subsystem = "windows"]
use bevy::{prelude::*, window::*};
use bevy_screen_diagnostics::*;
use bevy_simple_tilemap::prelude::*;
use provatheus::*;
mod provatheus;
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy".into(),                                          //タイトル
                        resolution: (1280.0, 960.0).into(), //ウィンドウサイズ
                        position: WindowPosition::Centered(MonitorSelection::Primary), //ウィンドウの生成座標を中心に設定
                        present_mode: PresentMode::Fifo,                               //Vsync有効
                        resizable: false, //サイズ変更不可
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
            ProvatheusPlugin,                   //プロヴァテウスプラグイン
            SimpleTileMapPlugin,                //タイルマッププラグイン
            ScreenDiagnosticsPlugin::default(), //診断データプラグイン
            ScreenFrameDiagnosticsPlugin,       //FPS表示プラグイン
        ))
        //以上は固定
        .add_systems(Startup, set_camera) //カメラを生成
        .run();
}

#[derive(Component)]
struct MainCamera;
fn set_camera(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2dBundle { ..default() }));
}
