//external crates
use bevy::
{   prelude::*,
    log::LogPlugin,
    core_pipeline::clear_color::ClearColorConfig,
    window::WindowMode,
    asset::LoadState,
    render::camera,
    diagnostic::DiagnosticsStore,
    diagnostic::FrameTimeDiagnosticsPlugin,
    input::mouse,
};
use once_cell::sync::Lazy;
use counted_array::counted_array;
use rand::prelude::*;

//standard library
use std::ops::Range;
use std::f32::consts::{ PI, TAU };

//internal submodules
mod public;
use public::*;

mod load_assets;
mod init_app;
mod play_game;

////////////////////////////////////////////////////////////////////////////////

//メイン関数
fn main()
{   //アプリの生成
    let mut app = App::new();

    //メインウィンドウの設定
    let primary_window = MAIN_WINDOW.clone();
    let log_level = if misc::DEBUG() { LOG_LEVEL_DEV } else { LOG_LEVEL_REL };
    let filter = log_level.into();
    app
    .insert_resource( Msaa::Sample4 ) //アンチエイリアス
    .add_plugins
    (   DefaultPlugins
        .set( WindowPlugin { primary_window, ..default() } ) //メインウィンドウ
        .set( ImagePlugin::default_nearest() ) //ピクセルパーフェクト
        .set( LogPlugin { filter, ..default() } ) //ロギング
    )
    .add_systems
    (   Startup,
        (   misc::spawn_2d_camera, //2D camera
            misc::spawn_3d_camera, //3D camera
            misc::spawn_3d_light,  //3D light
            debug::spawn_2d_sprites.run_if( misc::DEBUG ), //2D表示テスト
            debug::spawn_3d_objects.run_if( misc::DEBUG ), //3D表示テスト
        )
    )
    .add_systems
    (   Update,
        (   (   bevy::window::close_on_esc, //[ESC]で終了
                misc::toggle_window_mode,   //フルスクリーン切換
            )
            .run_if( not( misc::WASM ) ),

            (   (   misc::catch_input_keyboard, //極座標を更新(キー入力)
                    misc::catch_input_mouse,    //極座標を更新(マウス)
                    misc::catch_input_gamepad,  //極座標を更新(ゲームパッド)
                ),

                #[cfg( debug_assertions )] //Updateで.run_ifよりover head少ない？
                misc::move_orbit_camera::<misc::AppDefault3dCamera>, //Camera操作テスト
            )
            .chain() //実行順の固定
        )
    );

    //メイン処理
    app
    .add_state::<MyState>() //Stateを初期化する。enumの#[default]で初期値指定
    .add_plugins( load_assets::Schedule ) //assetsの事前ロード
    .add_plugins( init_app::Schedule )    //ゲーム枠とFPSの表示
    .add_plugins( play_game::Schedule )   //ゲームロジック
    ;

    //アプリの実行
    app.run();
}

////////////////////////////////////////////////////////////////////////////////

//End of code.