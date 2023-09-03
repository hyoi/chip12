use super::*;

////////////////////////////////////////////////////////////////////////////////

//.run_if( condition )用
pub const DEBUG: fn() -> bool = || cfg!( debug_assertions );
pub const WASM : fn() -> bool = || cfg!( target_arch = "wasm32" );

////////////////////////////////////////////////////////////////////////////////

//2D cameraをspawnする
pub fn spawn_2d_camera( mut cmds: Commands )
{   cmds.spawn( Camera2dBundle::default() )
    .insert( Camera { order: CAMERA2D_ORDER, ..default() } )
    .insert( Camera2d { clear_color: CAMERA2D_BGCOLOR } )
    ;
}

//デフォルトの3D CameraのComponent
#[derive( Component )]
pub struct AppDefault3dCamera;

//3D cameraをspawnする
pub fn spawn_3d_camera( mut cmds: Commands )
{   let _id = 
    cmds.spawn( ( Camera3dBundle:: default(), AppDefault3dCamera ) )
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

//ウィンドウとフルスクリーンの切換(トグル動作)
pub fn toggle_window_mode
(   mut q_window: Query<&mut Window>,
    keys: Res<Input<KeyCode>>,
    gpdbtn: Res<Input<GamepadButton>>,
    gamepads: Res<Gamepads>,
)
{   let Ok( mut window ) = q_window.get_single_mut() else { return };

    //[Alt]＋[Enter]の状態
    let is_key_pressed =
        ( keys.pressed( KeyCode::AltRight ) || keys.pressed( KeyCode::AltLeft ) )
            && keys.just_pressed( KeyCode::Return );

    //ゲームパッドは抜き挿しでIDが変わるので.iter()で回す
    let button_type = GamepadButtonType::Select; //ps4[SHARE]
    let mut is_gpdbtn_pressed = false;
    for gamepad in gamepads.iter()
    {   if gpdbtn.just_pressed( GamepadButton { gamepad, button_type } )
        {   is_gpdbtn_pressed = true;
            break;
        }
    }

    //入力がないなら
    if ! is_key_pressed && ! is_gpdbtn_pressed { return }

    //ウィンドウとフルスクリーンを切り替える
    window.mode = match window.mode
    {   WindowMode::Windowed => WindowMode::SizedFullscreen,
        _                    => WindowMode::Windowed,
    };
}

////////////////////////////////////////////////////////////////////////////////

//QueryしたEnityを再帰的に削除する
pub fn despawn<T: Component>
(   q_entity: Query<Entity, With<T>>,
    mut cmds: Commands,
)
{   q_entity.for_each( | id | cmds.entity( id ).despawn_recursive() );
}

////////////////////////////////////////////////////////////////////////////////

//ゲームパッドによって極座標カメラの位置を更新する
pub fn catch_input_gamepad
(   o_camera: Option<ResMut<OrbitCamera>>,
    time: Res<Time>,
    axis_button: Res<Axis<GamepadButton>>,
    axis_stick : Res<Axis<GamepadAxis>>,
    gamepads: Res<Gamepads>,
)
{   let Some ( mut camera ) = o_camera else { return };
    let orbit = &mut camera.orbit;
    let time_delta = time.delta().as_secs_f32(); //前回の実行からの経過時間

    //ゲームパッドは抜き挿しでIDが変わるので.iter()で回す
    for gamepad in gamepads.iter()
    {   //左トリガーでズームイン
        let button_type = GamepadButtonType::LeftTrigger2;
        let button = GamepadButton { gamepad, button_type };
        if let Some ( value ) = axis_button.get( button )
        {   orbit.r -= value * time_delta;
            orbit.r = orbit.r.max( ORBIT_CAMERA_MIN_R );
        }

        //右トリガーでズームアウト
        let button_type = GamepadButtonType::RightTrigger2; 
        let button = GamepadButton { gamepad, button_type };
        if let Some ( value ) = axis_button.get( button )
        {   orbit.r += value * time_delta;
            orbit.r = orbit.r.min( ORBIT_CAMERA_MAX_R );
        }

        //左スティックのＹ軸で上下首振り
        let axis_type = GamepadAxisType::LeftStickY;
        let stick_y = GamepadAxis { gamepad, axis_type };
        if let Some ( value ) = axis_stick.get( stick_y )
        {   orbit.theta += value * time_delta;
            orbit.theta = orbit.theta
                .min( ORBIT_CAMERA_MAX_THETA )
                .max( ORBIT_CAMERA_MIN_THETA );
        }

        //左スティックのＸ軸で左右回転
        let axis_type = GamepadAxisType::LeftStickX;
        let stick_x = GamepadAxis { gamepad, axis_type };
        if let Some ( value ) = axis_stick.get( stick_x )
        {   orbit.phi += value * time_delta;
            orbit.phi -= if orbit.phi >= TAU { TAU } else { 0.0 };
            orbit.phi += if orbit.phi <  0.0 { TAU } else { 0.0 };
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//マウス入力によって極座標カメラの位置を更新する
pub fn catch_input_mouse
(   o_camera: Option<ResMut<OrbitCamera>>,
    mouse_nutton: Res<Input<MouseButton>>,
    mut e_mouse_motion: EventReader<mouse::MouseMotion>,
    mut e_mouse_wheel: EventReader<mouse::MouseWheel>,
)
{   let Some ( mut camera ) = o_camera else { return };
    let orbit = &mut camera.orbit;

    //ホイール
    for mouse_wheel in e_mouse_wheel.iter()
    {   orbit.r += mouse_wheel.y * MOUSE_WHEEL_Y_COEF; //感度良すぎるので
        orbit.r = orbit.r
            .min( ORBIT_CAMERA_MAX_R )
            .max( ORBIT_CAMERA_MIN_R );
    }

    //右ボタンが押されていないなら
    if ! mouse_nutton.pressed( MouseButton::Left ) { return }

    //マウスの上下左右
    for mouse_motion in e_mouse_motion.iter()
    {   //上下首振り
        orbit.theta += mouse_motion.delta.y * MOUSE_MOTION_Y_COEF; //感度良すぎるので
        orbit.theta = orbit.theta
            .min( ORBIT_CAMERA_MAX_THETA )
            .max( ORBIT_CAMERA_MIN_THETA );

        //左右回転
        orbit.phi -= mouse_motion.delta.x * MOUSE_MOTION_X_COEF; //感度良すぎるので
        orbit.phi -= if orbit.phi >= TAU { TAU } else { 0.0 };
        orbit.phi += if orbit.phi <  0.0 { TAU } else { 0.0 };
    }
}

////////////////////////////////////////////////////////////////////////////////

//キー入力によって極座標カメラの位置を更新する
pub fn catch_input_keyboard
(   o_camera: Option<ResMut<OrbitCamera>>,
    time: Res<Time>,
    inkey: Res<Input<KeyCode>>,
)
{   let Some ( mut camera ) = o_camera else { return };
    let orbit = &mut camera.orbit;
    let time_delta = time.delta().as_secs_f32(); //前回の実行からの経過時間

    for keycode in inkey.get_pressed()
    {   match keycode
        {   KeyCode::X =>
                orbit.r = ( orbit.r + time_delta ).min( ORBIT_CAMERA_MAX_R ),
            KeyCode::Z =>
                orbit.r = ( orbit.r - time_delta ).max( ORBIT_CAMERA_MIN_R ),
            KeyCode::Up =>
                orbit.theta = ( orbit.theta + time_delta ).min( ORBIT_CAMERA_MAX_THETA ),
            KeyCode::Down =>
                orbit.theta = ( orbit.theta - time_delta ).max( ORBIT_CAMERA_MIN_THETA ),
            KeyCode::Right =>
            {   orbit.phi += time_delta;
                orbit.phi -= if orbit.phi >= TAU { TAU } else { 0.0 };
            }
            KeyCode::Left =>
            {   orbit.phi -= time_delta;
                orbit.phi += if orbit.phi < 0.0 { TAU } else { 0.0 };
            }
            _ => (),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//極座標に従って3D Cameraを移動する
//＜副作用＞ Res<OrbitCamera>が見つからない場合、Resouceを作成する
pub fn move_orbit_camera<T: Component>
(   mut q_camera: Query<&mut Transform, With<T>>,
    o_camera: Option<Res<OrbitCamera>>,
    mut cmds: Commands,
)
{   let Ok ( mut transform ) = q_camera.get_single_mut() else { return };
    let orbit = if let Some ( camera ) = o_camera
    {   camera.orbit
    }
    else
    {   cmds.init_resource::<OrbitCamera>(); //<OrbitCamera>が見つからない場合
        OrbitCamera::default().orbit
    };

    //カメラの位置と向きを更新する
    let vec3 = orbit.convert_pixel();
    *transform = Transform::from_translation( vec3 ).looking_at( Vec3::ZERO, Vec3::Y );
}

////////////////////////////////////////////////////////////////////////////////

//End of code.