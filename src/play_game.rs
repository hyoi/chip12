use super::*;

mod map;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        //InitApp後に遷移するStateをセットする
        .insert_resource( AfterInitApp { state: MyState::GameStart } )

        //Resourceの登録
        .init_resource::<map::Map>() //Map情報

        //前処理
        .add_systems
        (   OnEnter ( MyState::GameStart ),
            (   map::make_new_data, //新しいMapデータを作る
                map::spawn_entity,  //Mapを3D表示する
            )
            .chain()
        )
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.