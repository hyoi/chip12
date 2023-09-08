use super::*;

////////////////////////////////////////////////////////////////////////////////

//MapのResource
#[derive( Resource )]
pub struct Map
{   rng   : rand::prelude::StdRng, //専用乱数発生器
    matrix: Vec<Vec<Flag>>,        //map
    start : IVec2,                 //スタート位置
}

//マスの情報
#[derive( Clone )]
struct Flag ( u128 );

//Map::default()の定義
impl Default for Map
{   fn default() -> Self
    {   let seed_dev = 1234567890;
        let seed_rel = || rand::thread_rng().gen::<u64>();
        let seed = if misc::DEBUG() { seed_dev } else { seed_rel() };

        let cell = Flag ( BIT_CELL_UNDEF );
        let column = vec![ cell  ; MAP_GRIDS_HEIGHT as usize ];
        let matrix = vec![ column; MAP_GRIDS_WIDTH  as usize ];

        Self
        {   rng  : StdRng::seed_from_u64( seed ),
            matrix,
            start: IVec2::default(),
        }
    }
}

//マス目の状態を表すビット(フラグは128個まで)
const BIT_CELL_UNDEF  : u128 = 0b000; //未定義
const BIT_CELL_SPACE  : u128 = 0b001; //地形：空地
const BIT_CELL_WALL   : u128 = 0b010; //地形：壁
const BIT_FLAG_DEADEND: u128 = 0b100; //フラグ：行き止り

////////////////////////////////////////////////////////////////////////////////

//Mapのメソッド
impl Map
{   //ユーティリティ
    fn is_inside( &self, cell: IVec2 ) -> bool
    {   MAP_GRIDS_X_RANGE.contains( &cell.x ) &&
        MAP_GRIDS_Y_RANGE.contains( &cell.y )
    }
    fn matrix_mut( &mut self, IVec2 { x, y }: IVec2 ) -> &mut Flag
    {   &mut self.matrix[ x as usize ][ y as usize ]
    }
    fn matrix( &self, IVec2 { x, y }: IVec2 ) -> &Flag
    {   &self.matrix[ x as usize ][ y as usize ]
    }

    //全体を埋める
    fn fill_walls( &mut self )
    {   self.matrix.iter_mut().for_each
        (   |column| column.fill( Flag ( BIT_CELL_WALL ) )
        );
    }

    //指定の位置の地形を書き換える（フラグはクリアされる）
    fn set_space( &mut self, cell: IVec2 )
    {   if ! self.is_inside( cell ) { return }
        *self.matrix_mut( cell ) = Flag ( BIT_CELL_SPACE );
    }

    //指定の位置の地形にフラグを付加する
    fn add_flag_deadend( &mut self, cell: IVec2 )
    {   if ! self.is_inside( cell ) { return }
        self.matrix_mut( cell ).0 |= BIT_FLAG_DEADEND;
    }

    //指定の位置の地形・フラグを判定する
    fn is_wall( &self, cell: IVec2 ) -> bool
    {   if ! self.is_inside( cell ) { return true } //範囲外は壁にする
        self.matrix( cell ).0 & BIT_CELL_WALL != 0
    }
    fn is_space( &self, cell: IVec2 ) -> bool
    {   if ! self.is_inside( cell ) { return false } //範囲外に空地はない
        self.matrix( cell ).0 & BIT_CELL_SPACE != 0
    }
    fn is_deadend( &self, cell: IVec2 ) -> bool
    {   if ! self.is_inside( cell ) { return false } //範囲外に空地はない(＝行き止りもない)
        self.matrix( cell ).0 & BIT_FLAG_DEADEND != 0
    }
}

////////////////////////////////////////////////////////////////////////////////

//Mapのメソッド（迷路作成）
impl Map
{   fn build_labyrinth( &mut self )
    {   //穴を掘る準備
        let mut cell = self.start;
        let mut digable_walls = Vec::new();
        let mut backtrack;

        //穴掘りループ
        loop
        {   //四方の判定の初期化
            digable_walls.clear();
            backtrack = IVec2::default();

            //四方にある掘れる壁と戻り路を記録する
            for news in NEWS
            {   let next = cell + news;

                //外壁は掘れない
                if ! MAP_GRIDS_X_RANGE_INNER.contains( &next.x )
                || ! MAP_GRIDS_Y_RANGE_INNER.contains( &next.y ) { continue }

                //四方のグリッドを調べる
                if self.is_wall( next ) && self.is_digable( next, news )
                {   digable_walls.push( next );
                }
                else if self.is_space( next ) && ! self.is_deadend( next )
                {   //道であり、且つ行止りのマーキングがないなら
                    backtrack = next;
                }
            }

            if ! digable_walls.is_empty()
            {   //掘れる壁が見つかったので、方向をランダムに決めて進む
                cell = digable_walls[ self.rng.gen_range( 0..digable_walls.len() ) ];
                self.set_space( cell );
            }
            else
            {   //掘れる壁が見つからず、戻り路も見つからないなら迷路完成
                if backtrack == IVec2::default() { break } //ループ脱出

                //現在位置に行き止まりをマークし、後戻りする
                self.add_flag_deadend( cell );
                cell = backtrack;
            }
        }

        //他の迷路作成関数を流用して道幅拡張工事をする
        // self.make_type_labyrinth();
    }

    //壁が掘れるか調べる
    fn is_digable( &self, cell: IVec2, sides: News ) -> bool
    {   match sides
        {   News::North =>
                if self.is_wall( cell + News::North + News::West )
                && self.is_wall( cell + News::North              ) // 壁壁壁
                && self.is_wall( cell + News::North + News::East ) // 壁？壁
                && self.is_wall( cell + News::West               ) // 　◎
                && self.is_wall( cell + News::East               ) { return true },
            News::West =>
                if self.is_wall( cell + News::North + News::West )
                && self.is_wall( cell + News::North              ) // 壁壁
                && self.is_wall( cell + News::West               ) // 壁？◎
                && self.is_wall( cell + News::South + News::West ) // 壁壁
                && self.is_wall( cell + News::South              ) { return true },
            News::East =>
                if self.is_wall( cell + News::North              )
                && self.is_wall( cell + News::North + News::East ) // 　壁壁
                && self.is_wall( cell + News::East               ) // ◎？壁
                && self.is_wall( cell + News::South              ) // 　壁壁
                && self.is_wall( cell + News::South + News::East ) { return true },
            News::South =>
                if self.is_wall( cell + News::West               )
                && self.is_wall( cell + News::East               ) // 　◎
                && self.is_wall( cell + News::South + News::West ) // 壁？壁
                && self.is_wall( cell + News::South              ) // 壁壁壁
                && self.is_wall( cell + News::South + News::East ) { return true },
        };

        false //掘れない
    }
}

////////////////////////////////////////////////////////////////////////////////

//新しいMapデータを作る
pub fn make_new_data
(   mut map: ResMut<Map>,
)
{   //初期化する
    map.fill_walls();

    //スタート地点を決める
    map.start = IVec2::new( MAP_GRIDS_WIDTH / 2, MAP_GRIDS_HEIGHT / 2 );
    let start = map.start;
    map.set_space( start );

    //迷路を掘る
    map.build_labyrinth();

    //迷路の構造解析
    // map.examine_structure(); //広間と通路を識別して袋小路に目印を付ける
}

////////////////////////////////////////////////////////////////////////////////

//Mapの全Entityの親になるEntityに印をつけるComponent
#[derive( Component )]
pub struct MapZeroEntity;

//mapオブジェクト関係
const WALL_CUBE_OBJ3D_SIZE      : f32 = 1.0; //拡大率
const WALL_CUBE_OBJ3D_COLOR     : Color = Color::BISQUE;
const WALL_CUBE_OBJ3D_COLOR_ZERO: Color = Color::RED; //debug時の原点Cubeの色
const GROUND_PLANE_OBJ3D_COLOR  : Color = Color::MAROON;

//迷路の3Dオブジェクトをspawnする
pub fn spawn_entity
(   q_entity: Query<Entity, With<MapZeroEntity>>,
    map: Res<Map>,
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)
{   //既存のEntityがあれば削除する
    q_entity.for_each( | id | cmds.entity( id ).despawn_recursive() );

    //壁のサイズ、原点の壁のテクスチャ、他の壁のテクスチャ、地面のテクスチャ
    let size = WALL_CUBE_OBJ3D_SIZE * 0.9;
    let texture_wall_zero   = WALL_CUBE_OBJ3D_COLOR_ZERO.into();
    let texture_wall_normal: StandardMaterial = WALL_CUBE_OBJ3D_COLOR.into();
    let texture_ground = GROUND_PLANE_OBJ3D_COLOR.into();

    //迷路をspawnする
    cmds.spawn( ( PbrBundle::default(), MapZeroEntity ) ) //Cube(親)
    .insert( meshes.add( shape::Cube::new( size ).into() ) )
    .insert( Transform::from_translation( Vec3::ZERO ) ) //原点
    .insert( materials.add( texture_wall_zero ) )
    .with_children
    (   | cmds |
        {   //子は、親からの相対位置にspawnされる(XZ平面)
            for x in MAP_GRIDS_X_RANGE
            {    for y in MAP_GRIDS_Y_RANGE
                {   //原点は親なのでスキップ
                    if x == 0 && y == 0 { continue }

                    //3D空間での座標
                    let grid = IVec2::new( x, y );
                    let vec3 = grid.convert_3d_space();

                    //壁
                    if map.is_wall( grid )
                    {   cmds.spawn( PbrBundle::default() )
                        .insert( meshes.add( shape::Cube::new( size ).into() ) )
                        .insert( Transform::from_translation( vec3 ) )
                        .insert( materials.add( texture_wall_normal.clone() ) )
                        ;
                    }
                }
            }

            //地面(子)も相対位置でspawnする
            let long_side = MAP_GRIDS_WIDTH.max( MAP_GRIDS_HEIGHT ) as f32;
            let half = long_side / 2.0;
            let position = Vec3::new( half, 0.0, half ) - Vec3::ONE / 2.0;
            cmds.spawn( PbrBundle::default() )
            .insert( meshes.add( shape::Plane::from_size( long_side ).into() ) )
            .insert( Transform::from_translation( position ) )
            .insert( materials.add( texture_ground ) )
            ;
        }
    );
}

////////////////////////////////////////////////////////////////////////////////


//End of code.