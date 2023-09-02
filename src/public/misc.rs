use super::*;

////////////////////////////////////////////////////////////////////////////////

//.run_if( condition )用
pub const DEBUG: fn() -> bool = || cfg!( debug_assertions );

////////////////////////////////////////////////////////////////////////////////

//極座標カメラの設定値
const ORBIT_CAMERA_INIT_R    : f32 = 5.0;      //初期値
const ORBIT_CAMERA_INIT_THETA: f32 = PI * 0.6; //初期値(ラジアン) 1.0:天頂、0.5:真横、0.0:真下
const ORBIT_CAMERA_INIT_PHI  : f32 = PI * 1.8; //初期値(ラジアン) 6時方向が0.0で反時計回り

//2D cameraをspawnする
pub fn spawn_2d_camera( mut cmds: Commands )
{   cmds.spawn( Camera2dBundle::default() )
    .insert( Camera { order: CAMERA2D_ORDER, ..default() } )
    .insert( Camera2d { clear_color: CAMERA2D_BGCOLOR } )
    ;
}

//3D cameraをspawnする
pub fn spawn_3d_camera( mut cmds: Commands )
{   let _id = 
    cmds.spawn( Camera3dBundle:: default() )
    .insert( Camera { order: CAMERA3D_ORDER, ..default() } )
    .insert( Camera3d { clear_color: CAMERA3D_BGCOLOR, ..default() } )
    .id()
    ;

    //debug時にcameraのtransformをセットする
    //（ここでセットしないと期待した表示にならなかった）
    #[cfg( debug_assertions )]
    cmds.entity( _id )
    .insert
    (   Transform::from_translation
        (   Orbit
            {   r    : ORBIT_CAMERA_INIT_R,
                theta: ORBIT_CAMERA_INIT_THETA,
                phi  : ORBIT_CAMERA_INIT_PHI,
            }
            .convert_pixel()
        )
        .looking_at( Vec3::ZERO, Vec3::Y )
    );
}

//3D lightをspawnする
pub fn spawn_3d_light( mut cmds: Commands )
{   let illuminance = LIGHT3D_BRIGHTNESS;
    let shadows_enabled = true;
    let light = DirectionalLight { illuminance, shadows_enabled, ..default() };

    cmds.spawn( DirectionalLightBundle::default() )
    .insert( light )
    .insert( LIGHT3D_TRANSFORM.looking_at( Vec3::ZERO, Vec3::Y ) )
    ;
}

////////////////////////////////////////////////////////////////////////////////

//End of code.