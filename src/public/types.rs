use super::*;

////////////////////////////////////////////////////////////////////////////////

//glamの型にメソッドを追加する準備
pub trait GridToPixel
{   fn convert_pixel( &self ) -> Vec2;
}

//glamの型にメソッドを追加する
impl GridToPixel for IVec2
{   //Grid座標(IVec2)からスクリーン座標(Vec2)を算出する
    fn convert_pixel( &self ) -> Vec2
    {   let neg_half_w = SCREEN_PIXELS_WIDTH  / -2.0;
        let half_h     = SCREEN_PIXELS_HEIGHT /  2.0;
        let half_grid  = PIXELS_PER_GRID      /  2.0;

        let x = neg_half_w + PIXELS_PER_GRID * self.x as f32 + half_grid;
        let y = half_h     - PIXELS_PER_GRID * self.y as f32 - half_grid;

        Vec2::new( x, y )
    }
}

////////////////////////////////////////////////////////////////////////////////

//極座標の型
pub struct Orbit
{   pub r    : f32, //極座標のr（注目点からカメラまでの距離）
    pub theta: f32, //極座標のΘ（注目点から見たカメラの垂直角度）
    pub phi  : f32, //極座標のφ（注目点から見たカメラの水平角度）
}

impl Orbit
{   //極座標から直交座標へ変換する
    pub fn convert_pixel( &self ) -> Vec3
    {   let x = self.r * self.theta.sin() * self.phi.sin();
        let y = self.r * self.theta.cos() * -1.0;
        let z = self.r * self.theta.sin() * self.phi.cos();

        Vec3::new( x, y, z )
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.