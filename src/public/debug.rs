#![allow( dead_code )]

use super::*;

////////////////////////////////////////////////////////////////////////////////

//スプライトの設定
const COLOR_SPRITE_DEBUG_GRID: Color = Color::rgba( 1.0, 1.0, 1.0, 0.01 );

//マス目状にスプライトを敷き詰める
pub fn spawn_2d_sprites
(   mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   let color = COLOR_SPRITE_DEBUG_GRID;
    let custom_size = Some ( SIZE_GRID );

    for x in SCREEN_GRIDS_X_RANGE
    {   for y in SCREEN_GRIDS_Y_RANGE
        {   let vec2 = IVec2::new( x, y ).convert_pixel();
            let vec3 = vec2.extend( DEPTH_SPRITE_DEBUG_GRID );

            cmds.spawn( SpriteBundle::default() )
            .insert( Sprite { custom_size, color, ..default() } )
            .insert( Transform::from_translation( vec3 ) )
            .insert( asset_svr.load( ASSETS_SPRITE_DEBUG_GRID ) as Handle<Image> )
            ;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//3Dオブジェクトの設定
const SIZE_OBJ3D_DEBUG_PLANE: f32 = 5.0; //地面の縦横のサイズ
const SIZE_OBJ3D_DEBUG_CUBE : f32 = 1.0; //立方体の辺のサイズ
const COLOR_OBJ3D_DEBUG_PLANE: Color = Color::rgb( 0.3, 0.5, 0.3 ); //地面の色
const COLOR_OBJ3D_DEBUG_CUBE : Color = Color::rgb( 0.8, 0.7, 0.6 ); //正方形の色

//3Dオブジェクトの配置
pub fn spawn_3d_objects
(   mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)
{   //立方体
    cmds.spawn( PbrBundle::default() )
    .insert( meshes.add( shape::Cube::new( SIZE_OBJ3D_DEBUG_CUBE ).into() ) )
    .insert( Transform::from_translation( Vec3::ZERO ) ) //原点(全軸0.0)
    .insert( materials.add( COLOR_OBJ3D_DEBUG_CUBE.into() ) )
    ;

    //地面
    cmds.spawn( PbrBundle::default() )
    .insert( meshes.add( shape::Plane::from_size( SIZE_OBJ3D_DEBUG_PLANE ).into() ) )
    .insert( Transform::from_translation( Vec3::Y / -2.0 ) ) //Y軸を0.5下方へ移動
    .insert( materials.add( COLOR_OBJ3D_DEBUG_PLANE.into() ) )
    ;
}

////////////////////////////////////////////////////////////////////////////////

//End of code.