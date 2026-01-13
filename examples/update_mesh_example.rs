//! An example that showcases how to update the mesh.
#[allow(unused_imports, dead_code)]
use bevy::pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin};
use bevy::{color::palettes::css::LIMEGREEN, prelude::*};
use bevy_meshem::prelude::*;
use rand::prelude::*;

/// Constants for us to use.
const FACTOR: usize = 8;
const CHUNK_LEN: usize = FACTOR * FACTOR * FACTOR;
const SPEED: f32 = FACTOR as f32 * 2.0;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins).add_plugins(WireframePlugin::default());

    let mesh = generate_voxel_mesh(
        [1.0, 1.0, 1.0],
        [1, 4],
        [
            (Top, [0, 0]),
            (Bottom, [0, 0]),
            (Right, [0, 0]),
            (Left, [0, 0]),
            (Back, [0, 0]),
            (Forward, [0, 0]),
        ],
        [0.0, 0.0, 0.0],
        0.05,
        Some(0.8),
        1.0,
    );
    let mesh2 = generate_voxel_mesh(
        [1.0, 1.0, 1.0],
        [1, 4],
        [
            (Top, [0, 1]),
            (Bottom, [0, 1]),
            (Right, [0, 1]),
            (Left, [0, 1]),
            (Back, [0, 1]),
            (Forward, [0, 1]),
        ],
        [0.0, 0.0, 0.0],
        0.05,
        Some(0.8),
        1.0,
    );
    app.insert_resource(BlockRegistry {
        grass: mesh,
        dirt: mesh2,
    });

    app.add_systems(Startup, setup).add_systems(
        Update,
        (
            input_handler,
            toggle_wireframe,
            input_handler_rotation,
            mesh_update,
        ),
    );

    app.add_message::<ToggleWireframe>()
        .add_message::<RegenerateMesh>();

    app.run();
}

#[derive(Component)]
struct Meshy {
    meta: MeshMD<u16>,
    grid: [u16; CHUNK_LEN],
}

#[derive(Component)]
struct MeshInfo;

#[derive(Message, Default)]
struct ToggleWireframe;

#[derive(Message, Default)]
struct RegenerateMesh;

/// Setting up everything to showcase the mesh.
fn setup(
    breg: Res<BlockRegistry>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // wireframe_config: ResMut<WireframeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let mut grid: Vec<u16> = vec![1; CHUNK_LEN];
    grid = grid
        .iter_mut()
        .enumerate()
        .map(|(i, x)| {
            if i >= FACTOR * FACTOR * FACTOR - FACTOR * FACTOR {
                2
            } else {
                *x
            }
        })
        .collect();
    let g: [u16; CHUNK_LEN] = grid.try_into().unwrap();
    let dims: Dimensions = (FACTOR, FACTOR, FACTOR);
    let texture_mesh = asset_server.load("array_texture.png");

    let (culled_mesh, metadata) = mesh_grid(
        dims,
        &[],
        &g,
        breg.into_inner(),
        MeshingAlgorithm::Culling,
        None,
    )
    .unwrap();
    let culled_mesh_handle: Handle<Mesh> = meshes.add(culled_mesh.clone());
    commands.spawn((
        Mesh3d(culled_mesh_handle),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::Srgba(LIMEGREEN),
            base_color_texture: Some(texture_mesh),
            ..default()
        })),
        Meshy {
            meta: metadata,
            grid: g,
        },
    ));

    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_and_light_transform = Transform::from_xyz(
        FACTOR as f32 * 1.7,
        FACTOR as f32 * 1.7,
        FACTOR as f32 * 1.7,
    )
    .looking_at(
        Vec3::new(
            FACTOR as f32 * 0.5,
            FACTOR as f32 * 0.5,
            FACTOR as f32 * 0.5,
        ),
        Vec3::Y,
    );

    // Ambient Light
    commands.spawn(AmbientLight {
        brightness: 400.0,
        color: Color::WHITE,
        ..default()
    });

    // Camera in 3D space.
    commands.spawn((
        Camera3d::default(),
        camera_and_light_transform
    ));

    // Light up the scene.
    commands.spawn((
        PointLight {
            intensity: 7000.0,
            range: 1000.0,
            ..default()
        },
        camera_and_light_transform
    ));
    // for (att, _val) in culled_mesh.attributes() {
    //     // dbg!(att);
    //     if att == Mesh::ATTRIBUTE_POSITION.id {}
    // }
    commands.spawn((
       Text(
            format!(
                "X/Y/Z: Rotate\nR: Reset orientation\nMove Camera: W/A/S/D/Left-Shift/Space\nToggle Wireframe: T\n"
            )
        ),
        TextColor(Color::Srgba(LIMEGREEN)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }
    ));
    commands.spawn((
        MeshInfo,
        Text(
            format!("Press -C- To Break / Add a random voxel\n")
        ),
        TextColor(Color::Srgba(LIMEGREEN)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }
    ));
}

#[derive(Resource)]
struct BlockRegistry {
    grass: Mesh,
    dirt: Mesh,
}

/// The important part! Without implementing a [`VoxelRegistry`], you can't use the function.
impl VoxelRegistry for BlockRegistry {
    /// The type of our Voxel, the example uses u16 for Simplicity but you may have a struct
    /// Block { Name: ..., etc ...}, and you'll define that as the type, but encoding the block
    /// data onto simple type like u16 or u64 is probably preferable.
    type Voxel = u16;
    /// The get_mesh function, probably the most important function in the
    /// [`VoxelRegistry`], it is what allows us to  quickly access the Mesh of each Voxel.
    fn get_mesh(&self, voxel: &Self::Voxel) -> VoxelMesh<&Mesh> {
        if *voxel == 0 {
            return VoxelMesh::Null;
        }
        if *voxel == 1 {
            return VoxelMesh::NormalCube(&self.dirt);
        }
        if *voxel == 2 {
            return VoxelMesh::NormalCube(&self.grass);
        }
        VoxelMesh::Null
    }
    /// Important function that tells our Algorithm if the Voxel is "full", for example, the Air
    /// in minecraft is not "full", but it is still on the chunk data, to signal there is nothing.
    fn is_covering(&self, voxel: &u16, _side: Face) -> bool {
        return *voxel != 0;
    }
    /// The center of the Mesh, out mesh is defined in src/voxel_mesh.rs, just a constant.
    fn get_center(&self) -> [f32; 3] {
        return [0.0, 0.0, 0.0];
    }
    /// The dimensions of the Mesh, out mesh is defined in src/voxel_mesh.rs, just a constant.
    fn get_voxel_dimensions(&self) -> [f32; 3] {
        return [1.0, 1.0, 1.0];
    }
    /// The attributes we want to take from out voxels, note that using a lot of different
    /// attributes will likely lead to performance problems and unpredictable behaviour.
    /// We chose these 3 because they are very common, the algorithm does preserve UV data.
    fn all_attributes(&self) -> Vec<bevy::mesh::MeshVertexAttribute> {
        return vec![
            Mesh::ATTRIBUTE_POSITION,
            Mesh::ATTRIBUTE_UV_0,
            Mesh::ATTRIBUTE_NORMAL,
        ];
    }
}

/// Simple system to handle inputs for the showcase.
fn input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Meshy>>,
    time: Res<Time>,
    mut message_writer: MessageWriter<ToggleWireframe>,
    mut e: MessageWriter<RegenerateMesh>,
) {
    if keyboard_input.pressed(KeyCode::KeyX) {
        for mut transform in &mut query {
            transform.rotate_x(time.delta_secs() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyY) {
        for mut transform in &mut query {
            transform.rotate_y(time.delta_secs() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyZ) {
        for mut transform in &mut query {
            transform.rotate_z(time.delta_secs() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyR) {
        for mut transform in &mut query {
            transform.look_to(Vec3::NEG_Z, Vec3::Y);
        }
    }
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        message_writer.write_default();
    }
    if keyboard_input.pressed(KeyCode::KeyC) {
        e.write_default();
    }
}

/// Function to toggle wireframe (seeing the vertices and indices of the mesh).
fn toggle_wireframe(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    with: Query<Entity, With<Wireframe>>,
    without: Query<Entity, (Without<Wireframe>, With<Meshy>)>,
    mut messages: MessageReader<ToggleWireframe>,
) {
    for _ in messages.read() {
        if let Ok(ent) = with.single() {
            commands.entity(ent).remove::<Wireframe>();
            for (_, material) in materials.iter_mut() {
                material.base_color.set_alpha(1.0);
            }
        } else if let Ok(ent) = without.single() {
            commands.entity(ent).insert(Wireframe);
            for (_, material) in materials.iter_mut() {
                material.base_color.set_alpha(0.0);
            }
        }
    }
}

fn input_handler_rotation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    let t = query.single_mut().unwrap().into_inner();
    if keyboard_input.pressed(KeyCode::Space) {
        t.translation += Vec3::Y * SPEED * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        t.translation += Vec3::NEG_Y * SPEED * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        t.translation += t.back() * SPEED * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        t.translation += t.forward() * SPEED * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        t.translation += t.left() * SPEED * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        t.translation += t.right() * SPEED * time.delta_secs();
    }
    t.look_at(
        Vec3::new(
            FACTOR as f32 * 0.5,
            FACTOR as f32 * 0.5,
            FACTOR as f32 * 0.5,
        ),
        Vec3::Y,
    );
    // for mut transform in &mut query {
    //     transform.translation += dir;
    // }
}

/// System to add or break random voxels.
fn mesh_update(
    mut meshy: Query<&mut Meshy>,
    breg: Res<BlockRegistry>,
    mut meshes: ResMut<Assets<Mesh>>,
    mesh_query: Query<&Mesh3d>,
    mut message_reader: MessageReader<RegenerateMesh>,
) {
    for _ in message_reader.read() {
        let mesh = meshes
            .get_mut(mesh_query.single().unwrap())
            .expect("Couldn't get a mut ref to the mesh");

        let m = meshy.single_mut().unwrap().into_inner();
        let mut rng = rand::thread_rng();
        let choice = m.grid.iter().enumerate().choose(&mut rng).unwrap();
        let neighbors: [Option<u16>; 6] = {
            let mut r = [None; 6];
            for i in 0..6 {
                match get_neighbor(choice.0, Face::from(i), m.meta.dims) {
                    None => {}
                    Some(j) => r[i] = Some(m.grid[j]),
                }
            }
            r
        };
        match choice {
            (i, 1) => {
                m.meta.log(VoxelChange::Broken, i, 1, neighbors);
                update_mesh(mesh, &mut m.meta, breg.into_inner());
                m.grid[i] = 0;
            }
            (i, 0) => {
                m.meta.log(VoxelChange::Added, i, 1, neighbors);
                update_mesh(mesh, &mut m.meta, breg.into_inner());
                m.grid[i] = 1;
            }
            _ => {}
        }
        break;
    }
}
