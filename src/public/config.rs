use super::*;

////////////////////////////////////////////////////////////////////////////////

//ウィンドウの定義
pub static MAIN_WINDOW: Lazy<Option<Window>> = Lazy::new
(   ||
    {   let title = format!( "{APP_TITLE} v{APP_VER}" );
        let window = Window
        {   title,
            resolution: ( SCREEN_PIXELS_WIDTH, SCREEN_PIXELS_HEIGHT ).into(),
            resizable: false,
            // fit_canvas_to_parent: true, //不具合が発生した場合コメントアウトする
            ..default()
        };

        Some ( window )
    }
);

////////////////////////////////////////////////////////////////////////////////

//アプリの情報
const _CARGO_TOML_NAME: &str = env!( "CARGO_PKG_NAME"    );
const _CARGO_TOML_VER : &str = env!( "CARGO_PKG_VERSION" );

const APP_TITLE: &str = _CARGO_TOML_NAME; //アプリタイトル
const APP_VER  : &str = _CARGO_TOML_VER;  //アプリのバージョン

////////////////////////////////////////////////////////////////////////////////

//ウィンドウ縦横幅(Pixel)
pub const SCREEN_PIXELS_WIDTH : f32 = PIXELS_PER_GRID * SCREEN_GRIDS_WIDTH  as f32;
pub const SCREEN_PIXELS_HEIGHT: f32 = PIXELS_PER_GRID * SCREEN_GRIDS_HEIGHT as f32;

////////////////////////////////////////////////////////////////////////////////

//単位Gridの縦横幅(Pixel)
const BASE_PIXELS: i32 = 8;
const SCALING: f32 = 4.0;
pub const PIXELS_PER_GRID: f32 = BASE_PIXELS as f32 * SCALING;

//GridのSize(縦横Pixel)
pub const SIZE_GRID: Vec2 = Vec2::new( PIXELS_PER_GRID, PIXELS_PER_GRID );

////////////////////////////////////////////////////////////////////////////////

//ウィンドウ縦横幅(Grid)
pub const SCREEN_GRIDS_WIDTH : i32 = 43; //memo: best 43
pub const SCREEN_GRIDS_HEIGHT: i32 = 24; //memo: best 24

pub const SCREEN_GRIDS_X_RANGE: Range<i32> = 0..SCREEN_GRIDS_WIDTH;
pub const SCREEN_GRIDS_Y_RANGE: Range<i32> = 0..SCREEN_GRIDS_HEIGHT;


////////////////////////////////////////////////////////////////////////////////

//ログレベル
pub const LOG_LEVEL_DEV: &str = "warn,wgpu_hal=error"; //開発
pub const LOG_LEVEL_REL: &str = "error"; //リリース

////////////////////////////////////////////////////////////////////////////////

//Cameraのレンダリングの重なり
pub const CAMERA2D_ORDER: isize = 1; //2Dが上
pub const CAMERA3D_ORDER: isize = 0; //3Dが下

////////////////////////////////////////////////////////////////////////////////

//Cameraの背景色
pub const CAMERA2D_BGCOLOR: ClearColorConfig = CAMERA_BG_TRANSPARENCY;
pub const CAMERA3D_BGCOLOR: ClearColorConfig = CAMERA_BG_COLOR;

const CAMERA_BG_TRANSPARENCY: ClearColorConfig = ClearColorConfig::None;
const CAMERA_BG_COLOR       : ClearColorConfig = ClearColorConfig::Custom( BG_COLOR );
const BG_COLOR: Color = Color::rgb( 0.13, 0.13, 0.18 );

////////////////////////////////////////////////////////////////////////////////

//3Dライトの設定
pub const LIGHT3D_BRIGHTNESS: f32 = 15000.0; //明るさ
pub const LIGHT3D_TRANSFORM: Transform = Transform::from_xyz( 30.0, 100.0, 40.0 ); //位置

////////////////////////////////////////////////////////////////////////////////

//End of code.