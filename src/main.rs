//external crates
use bevy::
{   prelude::*,
    log::LogPlugin,
};
use once_cell::sync::*;

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
    // .add_systems( Startup, misc::spawn_2d_camera ) //2D camera
    // .add_systems( Startup, misc::spawn_3d_camera ) //3D camera
    // .add_systems( Startup, misc::spawn_3d_light )  //3D light
    ;

    //アプリの実行
    app.run();
}

////////////////////////////////////////////////////////////////////////////////

//End of code.