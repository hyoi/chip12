use super::*;

////////////////////////////////////////////////////////////////////////////////

//glamの型にメソッドを追加する準備
pub trait GridToPixel
{   fn to_screen_pixel( &self ) -> Vec2;
    fn to_3dxz( &self ) -> Vec3;
}

//glamの型にメソッドを追加する
impl GridToPixel for IVec2
{   //平面座標(IVec2)からスクリーン用の座標(Vec2)を算出する
    fn to_screen_pixel( &self ) -> Vec2
    {   let neg_half_w = SCREEN_PIXELS_WIDTH  / -2.0;
        let half_h     = SCREEN_PIXELS_HEIGHT /  2.0;
        let half_grid  = PIXELS_PER_GRID      /  2.0;

        let x = neg_half_w + PIXELS_PER_GRID * self.x as f32 + half_grid;
        let y = half_h     - PIXELS_PER_GRID * self.y as f32 - half_grid;

        Vec2::new( x, y )
    }

    //平面座標(IVec2)から3D直交座標(Vec3)へ変換する
    fn to_3dxz( &self ) -> Vec3
    {   let x = self.x as f32;
        let y = 0.0; //xz平面上
        let z = self.y as f32;
        Vec3::new( x, y, z )
    }

}

////////////////////////////////////////////////////////////////////////////////

//極座標の型
#[derive( Clone, Copy )]
pub struct Orbit
{   pub r    : f32, //極座標のr（注目点からカメラまでの距離）
    pub theta: f32, //極座標のΘ（注目点から見たカメラの垂直角度）
    pub phi  : f32, //極座標のφ（注目点から見たカメラの水平角度）
}

impl Orbit
{   //極座標から直交座標へ変換する
    pub fn convert_vec3( &self ) -> Vec3
    {   let x = self.r * self.theta.sin() * self.phi.sin();
        let y = self.r * self.theta.cos() * -1.0;
        let z = self.r * self.theta.sin() * self.phi.cos();

        Vec3::new( x, y, z )
    }
}

////////////////////////////////////////////////////////////////////////////////

//極座標カメラのResource
#[derive( Resource, Clone, Copy )]
pub struct OrbitCamera
{   pub orbit: Orbit,  //カメラ自身の極座標
    pub look_at: Vec3, //カメラの注視点の直交座標
}

impl Default for OrbitCamera
{   fn default() -> Self
    {   Self
        {   orbit: Orbit
            {   r    : ORBIT_CAMERA_INIT_R,
                theta: ORBIT_CAMERA_INIT_THETA,
                phi  : ORBIT_CAMERA_INIT_PHI,
            },
            look_at: Vec3::ZERO,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//ゲームの状態
#[derive( Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States )]
pub enum MyState
{   #[default] LoadAssets,
    InitApp,
    GameStart,
}

//LoadAssetsの次のState登録用Resource
#[derive( Resource )]
pub struct AfterLoadAssets<T: States> { pub state: T }

//InitAppの次のState登録用Resource
#[derive( Resource )]
pub struct AfterInitApp<T: States> { pub state: T }

////////////////////////////////////////////////////////////////////////////////

//画面デザイン(枠)
pub struct ScreenFrame<'a>
{   pub design: Vec<&'a str>,
    pub zero  : Vec2,
    pub size  : Vec2,
}

////////////////////////////////////////////////////////////////////////////////

//四方を表す列挙型
#[derive( Default, Clone, Copy )]
pub enum News { #[default] North, East, West, South }

//IVec2 = IVec2 + News
impl Add<News> for IVec2
{   type Output = IVec2;
    fn add( mut self, news: News ) -> IVec2
    {   match news
        {   News::North => { self.y -= 1; }
            News::East  => { self.x += 1; }
            News::West  => { self.x -= 1; }
            News::South => { self.y += 1; }
        }
        self
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.