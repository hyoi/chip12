//external crates
use bevy::
{   prelude::*,
    log::LogPlugin,
    core_pipeline::clear_color::ClearColorConfig,
};
use once_cell::sync::*;

//standard library
use std::ops::Range;
use std::f32::consts::PI;

//internal submodules
mod public;
use public::*;


////////////////////////////////////////////////////////////////////////////////

//メイン関数
fn main()
{   //アプリの生成
    let mut app = App::new();

    //メインウィンドウの設定
    let primary_window = MAIN_WINDOW.clone();
    let log_level = if misc::DEBUG() { LOG_LEVEL_DEV } else { LOG_LEVEL_REL };
    app
    .insert_resource( Msaa::Sample4 ) //アンチエイリアス
    .add_plugins
    (   DefaultPlugins
        .set( WindowPlugin { primary_window, ..default() } ) //メインウィンドウ
        .set( ImagePlugin::default_nearest() ) //ピクセルパーフェクト
        .set( LogPlugin { filter: log_level.into(), ..default() } ) //ロギング
    )
    .add_systems
    (   Startup,
        (   misc::spawn_2d_camera, //2D camera
            misc::spawn_3d_camera, //3D camera
            misc::spawn_3d_light,  //3D light
            debug::spawn_2d_sprites.run_if( misc::DEBUG ), //2D表示テスト
            debug::spawn_3d_objects.run_if( misc::DEBUG ), //3D表示テスト
        )
    );

    //アプリの実行
    app.run();
}

////////////////////////////////////////////////////////////////////////////////

//End of code.