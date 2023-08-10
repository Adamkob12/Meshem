use bevy::prelude::*;
use meshem::prelude::*;

const FACTOR: usize = 12;
const MESHING_ALGORITHM: MeshingAlgorithm = MeshingAlgorithm::Culling;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.insert_resource(BlockRegistry {
        block: default_block(),
    });

    app.add_systems(Startup, setup)
        .add_systems(Update, input_handler);
    app.run();
}

#[derive(Component)]
struct Meshy;

fn setup(
    breg: Res<BlockRegistry>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // let grid: Vec<u16> = vec![1; 27];
    // let dims: Dimensions = (3, 3, 3);

    let grid: Vec<u16> = vec![1; FACTOR * FACTOR * FACTOR];
    let dims: Dimensions = (FACTOR, FACTOR, FACTOR);

    // let grid: Vec<u16> = vec![1; 8];
    // let dims: Dimensions = (2, 2, 2);

    let culled_mesh = meshem(dims, grid, breg.into_inner(), MESHING_ALGORITHM).unwrap();
    let culled_mesh_handle: Handle<Mesh> = meshes.add(culled_mesh.clone());
    let custom_texture_handle: Handle<Image> = asset_server.load("array_texture.png");
    commands.spawn((
        PbrBundle {
            mesh: culled_mesh_handle,
            material: materials.add(StandardMaterial {
                base_color_texture: Some(custom_texture_handle),
                ..default()
            }),
            ..default()
        },
        Meshy,
    ));

    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_and_light_transform = Transform::from_xyz(
        FACTOR as f32 * 2.0,
        FACTOR as f32 * 2.0,
        FACTOR as f32 * 2.0,
    )
    .looking_at(Vec3::ZERO, Vec3::Y);

    // Camera in 3D space.
    commands.spawn(Camera3dBundle {
        transform: camera_and_light_transform,
        ..default()
    });

    // Light up the scene.
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 500.0,
            ..default()
        },
        transform: camera_and_light_transform,
        ..default()
    });
    // for (att, _val) in culled_mesh.attributes() {
    //     // dbg!(att);
    //     if att == Mesh::ATTRIBUTE_POSITION.id {}
    // }
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~\n");
    dbg!(culled_mesh.count_vertices());
    println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~");
    commands.spawn(
        TextBundle::from_section(
            "X/Y/Z: Rotate\nR: Reset orientation",
            TextStyle {
                font_size: 20.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );
}

#[derive(Resource)]
struct BlockRegistry {
    block: Mesh,
}

impl VoxelRegistry for BlockRegistry {
    type Voxel = u16;
    fn get_mesh(&self, voxel: &Self::Voxel) -> Option<&Mesh> {
        if *voxel == 0 {
            return None;
        }
        Some(&self.block)
    }
    fn is_voxel(&self, voxel: &u16) -> bool {
        return *voxel != 0;
    }
    fn get_center(&self) -> [f32; 3] {
        return [0.0, 0.0, 0.0];
    }
    fn all_attributes(&self) -> Vec<bevy::render::mesh::MeshVertexAttribute> {
        return vec![
            Mesh::ATTRIBUTE_POSITION,
            Mesh::ATTRIBUTE_UV_0,
            Mesh::ATTRIBUTE_NORMAL,
        ];
    }
}
fn input_handler(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Meshy>>,
    time: Res<Time>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {}
    if keyboard_input.pressed(KeyCode::X) {
        for mut transform in &mut query {
            transform.rotate_x(time.delta_seconds() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::Y) {
        for mut transform in &mut query {
            transform.rotate_y(time.delta_seconds() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::Z) {
        for mut transform in &mut query {
            transform.rotate_z(time.delta_seconds() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::R) {
        for mut transform in &mut query {
            transform.look_to(Vec3::NEG_Z, Vec3::Y);
        }
    }
}
