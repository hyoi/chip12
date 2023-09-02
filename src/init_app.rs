use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        //assetsロード後に遷移するStateをセットする
        .insert_resource( AfterLoadAssets { state: MyState::InitApp } )

        //ゲーム枠とフッターを表示する
        .add_systems
        (   OnEnter ( MyState::InitApp ),
            (   spawn_screen_frame, //ゲーム枠を表示
                spawn_footer,       //フッターを表示
                set_viewport,       //cameraにviewportを設定
            )
        )
        .add_systems
        (   Update,
            (   goto_state_after_init_app, //無条件遷移
            )
            .run_if( in_state( MyState::InitApp ) )
        )

        //footerにFPSを表示する
        .add_plugins( FrameTimeDiagnosticsPlugin ) //FPSプラグイン
        .add_systems( Update, update_fps )         //FPS表示更新
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//画面デザイン(枠)
counted_array!
(   const DESIGN_SCREEN_FRAME: [ &str; _ ] =
    //   0123456789 123456789 123456789 123456789 123456789
    [   "0123456789012345678901234567890123456789012", //0----
        "1                              #0123456789#", //1
        "2                              #0123456789#", //2
        "3                              #0123456789#", //3
        "4                              #0123456789#", //4
        "5                              #0123456789#", //5
        "6                              #0123456789#", //6
        "7                              #0123456789#", //7
        "8                              #0123456789#", //8
        "9                              #0123456789#", //9
        "0                              #0123456789#", //10---
        "1                              #0123456789#", //11
        "2                              #0123456789#", //12
        "3                              #0123456789#", //13
        "4                              #0123456789#", //14
        "5                              #0123456789#", //15
        "6                              #0123456789#", //16
        "7                              #0123456789#", //17
        "8                              #0123456789#", //18
        "9                              #0123456789#", //19
        "0                              #0123456789#", //20---
        "1                              #0123456789#", //21
        "2####5####0####5####0####5####0####5####0##", //22
        "                                           ", //23
    ]  //0123456789 123456789 123456789 123456789 123456789
);

//表示エリア(viewport)の設定
const VIEWPORT_ADJUST: f32 = PIXELS_PER_GRID / 2.0;
const VIEWPORT_LEFT  : f32 = PIXELS_PER_GRID        - VIEWPORT_ADJUST;
const VIEWPORT_TOP   : f32 = PIXELS_PER_GRID        - VIEWPORT_ADJUST;
const VIEWPORT_WIDTH : f32 = PIXELS_PER_GRID * 30.0 + PIXELS_PER_GRID;
const VIEWPORT_HEIGHT: f32 = PIXELS_PER_GRID * 21.0 + PIXELS_PER_GRID;
const VIEWPORT_ZERO  : Vec2 = Vec2::new( VIEWPORT_LEFT, VIEWPORT_TOP );
const VIEWPORT_SIZE  : Vec2 = Vec2::new( VIEWPORT_WIDTH, VIEWPORT_HEIGHT );

//おまけ(蟹)
const GRID_X_KANI: i32 = SCREEN_GRIDS_WIDTH  - 4;
const GRID_Y_KANI: i32 = SCREEN_GRIDS_HEIGHT - 1;
const MAGNIFY_SPRITE_KANI: f32 = 0.9;
const COLOR_SPRITE_KANI: Color = Color::rgba( 1.0, 1.0, 1.0, 0.6 );

////////////////////////////////////////////////////////////////////////////////

//text UIのメッセージセクションの型
type MessageSect<'a> =
(   &'a str, //表示文字列
    &'a str, //フォントのAssets
    f32,     //フォントのサイズ
    Color,   //フォントの色
);

//フッター(FPS表示)のComponent
#[derive( Component )]
struct FooterUiFps;

//フッターの設定
const NA3_2: &str = "###.##";

counted_array!
(   const TEXT_FOOTER_LEFT: [ MessageSect; _ ] =
    [   ( " FPS ", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.6, Color::TEAL   ),
        ( NA3_2  , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.4, Color::SILVER ),
    ]
);

counted_array!
(   const TEXT_FOOTER_CENTER: [ MessageSect; _ ] =
    [   ( "hyoi 2023 - xxxx", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::TEAL ),
    ]
);

counted_array!
(   const TEXT_FOOTER_RIGHT: [ MessageSect; _ ] =
    [   ( "Powered by ", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::TEAL   ),
        ( "RUST"       , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::SILVER ),
        ( " & "        , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::TEAL   ),
        ( "BEVY "      , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::SILVER ),
    ]
);

////////////////////////////////////////////////////////////////////////////////

//ゲームの枠を表示する
fn spawn_screen_frame
(   mut cmds : Commands,
    asset_svr: Res<AssetServer>,
)
{   let custom_size = Some ( SIZE_GRID );
    let alpha = if misc::DEBUG() { 0.5 } else { 1.0 };
    let color = Color::rgba( 1.0, 1.0, 1.0, alpha );

    for ( y, line ) in DESIGN_SCREEN_FRAME.iter().enumerate()
    {   for ( x, char ) in line.chars().enumerate()
        {   if char == ' ' { continue }
            let vec2 = IVec2::new( x as i32, y as i32 ).convert_pixel();
            let vec3 = vec2.extend( DEPTH_SPRITE_GAME_FRAME );

            cmds.spawn( SpriteBundle::default() )
            .insert( Sprite { custom_size, color, ..default() } )
            .insert( Transform::from_translation( vec3 ) )
            .insert( asset_svr.load( ASSETS_SPRITE_BRICK_WALL ) as Handle<Image> )
            ;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//3D Cameraにviewport(表示エリア)をセットする
fn set_viewport
(   mut q_camera: Query<&mut Camera, With<Camera3d>>,
)
{   let Ok( mut camera ) = q_camera.get_single_mut() else { return };

    camera.viewport = Some
    (   camera::Viewport
        {   physical_position: VIEWPORT_ZERO.as_uvec2(),
            physical_size    : VIEWPORT_SIZE.as_uvec2(),
            ..default()
        }
    );
}

////////////////////////////////////////////////////////////////////////////////

//フッターを配置する
fn spawn_footer
(   mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   //レイアウト用の隠しフレームの準備
    let per100 = Val::Percent( 100.0 );
    let style = Style
    {   width          : per100,
        height         : per100,
        position_type  : PositionType::Absolute,
        flex_direction : FlexDirection::Column,
        justify_content: JustifyContent::FlexEnd, //画面の下端
        ..default()
    };
    let background_color = BackgroundColor ( Color::NONE );
    let hidden_frame = NodeBundle { style, background_color, ..default() };

    //フッターの準備
    let mut footer_left   = text_ui( &TEXT_FOOTER_LEFT  , &asset_svr );
    let mut footer_center = text_ui( &TEXT_FOOTER_CENTER, &asset_svr );
    let mut footer_right  = text_ui( &TEXT_FOOTER_RIGHT , &asset_svr );
    footer_left.style.align_self   = AlignSelf::FlexStart;
    footer_center.style.align_self = AlignSelf::Center;
    footer_right.style.align_self  = AlignSelf::FlexEnd;

    //隠しフレームの中に子要素を作成する
    cmds.spawn( hidden_frame ).with_children
    (   | cmds |
        {   cmds.spawn( ( footer_left, FooterUiFps ) );
            cmds.spawn(   footer_center              );
            cmds.spawn(   footer_right               );
        }
    );

    //おまけ(蟹)
    let custom_size = Some ( SIZE_GRID * MAGNIFY_SPRITE_KANI );
    let color = COLOR_SPRITE_KANI;
    let vec2 = IVec2::new( GRID_X_KANI, GRID_Y_KANI ).convert_pixel();
    let vec3 = vec2.extend( DEPTH_SPRITE_KANI_DOTOWN );

    cmds
    .spawn( SpriteBundle::default() )
    .insert( Sprite { custom_size, color, ..default() } )
    .insert( Transform::from_translation( vec3 ) )
    .insert( asset_svr.load( ASSETS_SPRITE_KANI_DOTOWN ) as Handle<Image> )
    ;
}

//TextBundleを作る
fn text_ui
(   message: &[ MessageSect ],
    asset_svr: &Res<AssetServer>,
) -> TextBundle
{   let mut sections = Vec::new();
    for ( line, file, size, color ) in message.iter()
    {   let value = line.to_string();
        let style = TextStyle
        {   font     : asset_svr.load( *file ),
            font_size: *size,
            color    : *color
        };
        sections.push( TextSection { value, style } );
    }
    let alignment = TextAlignment::Center;
    let position_type = PositionType::Absolute;

    let text  = Text { sections, alignment, ..default() };
    let style = Style { position_type, ..default() };
    TextBundle { text, style, ..default() }
}

////////////////////////////////////////////////////////////////////////////////

//フッターを更新する(FPS)
fn update_fps
(   mut q_text: Query<&mut Text, With<FooterUiFps>>,
    diag_store: Res<DiagnosticsStore>,
)
{   let Ok( mut text ) = q_text.get_single_mut() else { return };

    let fps_avr =
    diag_store
    .get( FrameTimeDiagnosticsPlugin::FPS )
    .map_or
    (   NA3_2.to_string(),
        | fps |
        fps
        .average()
        .map_or
        (   NA3_2.to_string(),
            | avg |
            format!( "{avg:06.2}" )
        )
    );

    text.sections[ 1 ].value = fps_avr;
}

////////////////////////////////////////////////////////////////////////////////

//ResourceにセットされたStateへ無条件に遷移する
fn goto_state_after_init_app
(   o_after_init_app: Option<Res<AfterInitApp<MyState>>>,
    mut next_state: ResMut<NextState<MyState>>,
)
{   let Some ( after_init_app ) = o_after_init_app else { return };
    next_state.set( after_init_app.state );
}

////////////////////////////////////////////////////////////////////////////////

//End of code.