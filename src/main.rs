use bevy::{prelude::*, window::*};
use bevy_screen_diagnostics::*;
mod provatheus;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy".into(),                                          //タイトル
                resolution: (1280.0, 720.0).into(),                            //ウィンドウサイズ
                position: WindowPosition::Centered(MonitorSelection::Primary), //ウィンドウの生成座標を中心に設定
                present_mode: PresentMode::AutoVsync,                          //Vsync有効
                resizable: false,                                              //サイズ変更不可
                enabled_buttons: bevy::window::EnabledButtons {
                    minimize: false, //最小化無効
                    maximize: false, //最大化無効
                    close: true,     //閉じる有効
                },
                visible: false, //非表示
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::NONE)) //デフォルトの背景色を設定
        .add_systems(Update, (provatheus::enable_visible, provatheus::gizmos_xyz)) //Provatheus用の開発用ライブラリ
        .add_plugins(ScreenDiagnosticsPlugin::default()) //FPS表示プラグイン//
        .add_plugins(ScreenFrameDiagnosticsPlugin) //FPS表示プラグイン
        .run();
}
