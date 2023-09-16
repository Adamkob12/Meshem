//! An example that showcases how to use the meshem function.
#[allow(unused_imports)]
use bevy::pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy_meshem::prelude::*;

/// Constants for us to use.
const FACTOR: usize = 10;
const SPEED: f32 = FACTOR as f32 * 2.0;
const MESHING_ALGORITHM: MeshingAlgorithm = MeshingAlgorithm::Culling;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins).add_plugins(WireframePlugin);

    app.insert_resource(BlockRegistry {
        block: default_block(),
    })
    .insert_resource(AmbientLight {
        brightness: 1.5,
        color: Color::WHITE,
    });

    app.add_systems(Startup, setup).add_systems(
        Update,
        (
            input_handler,
            toggle_wireframe,
            input_handler_rotation,
            regenerate_mesh,
        ),
    );

    app.add_event::<ToggleWireframe>()
        .add_event::<RegenerateMesh>();

    app.run();
}

#[derive(Component)]
struct Meshy {
    ma: MeshingAlgorithm,
    meta: MeshMD<u16>,
}

#[derive(Component)]
struct MeshInfo;

#[derive(Event, Default)]
struct ToggleWireframe;

#[derive(Event, Default)]
struct RegenerateMesh;

/// Setting up everything to showcase the mesh.
fn setup(
    breg: Res<BlockRegistry>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // wireframe_config: ResMut<WireframeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let grid: Vec<u16> = vec![1; FACTOR * FACTOR * FACTOR];
    let dims: Dimensions = (FACTOR, FACTOR, FACTOR);

    let (culled_mesh, metadata) =
        mesh_grid(dims, grid, breg.into_inner(), MESHING_ALGORITHM).unwrap();
    let culled_mesh_handle: Handle<Mesh> = meshes.add(culled_mesh.clone());
    commands.spawn((
        PbrBundle {
            mesh: culled_mesh_handle,
            material: materials.add(StandardMaterial {
                base_color: Color::SALMON,
                alpha_mode: AlphaMode::Mask(0.5),
                ..default()
            }),
            ..default()
        },
        Meshy {
            ma: MESHING_ALGORITHM,
            meta: metadata,
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

    // Camera in 3D space.
    commands.spawn(Camera3dBundle {
        transform: camera_and_light_transform,
        ..default()
    });

    // Light up the scene.
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 5000.0,
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
    commands.spawn(
        TextBundle::from_section(
            format!(
                "X/Y/Z: Rotate\nR: Reset orientation\nMove Camera: W/A/S/D/Left-Shift/Space\nToggle Wireframe: T\n"),
            TextStyle {
                font_size: 26.0,
                color: Color::LIME_GREEN,
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
    commands.spawn((
        MeshInfo,
        TextBundle::from_section(
            format!("Press -C- To regenerate the mesh with a different Algorithm\nVertices Count: {}\nMeshing Algorithm: {:?}",culled_mesh.count_vertices(),
                MESHING_ALGORITHM,),
            TextStyle {
                font_size: 26.0,
                color: Color::LIME_GREEN,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    ));
}

#[derive(Resource)]
struct BlockRegistry {
    block: Mesh,
}

/// The important part! Without implementing a [`VoxelRegistry`], you can't use the function.
impl VoxelRegistry for BlockRegistry {
    /// The type of our Voxel, the example uses u16 for Simplicity but you may have a struct
    /// Block { Name: ..., etc ...}, and you'll define that as the type, but encoding the block
    /// data onto simple type like u16 or u64 is probably prefferable.
    type Voxel = u16;
    /// The get_mesh function, probably the most important function in the
    /// [`VoxelRegistry`], it is what allows us to  quickly access the Mesh of each Voxel.
    fn get_mesh(&self, voxel: &Self::Voxel) -> Option<&Mesh> {
        if *voxel == 0 {
            return None;
        }
        Some(&self.block)
    }
    /// Important function that tells our Algorithm if the Voxel is "full", for example, the Air
    /// in minecraft is not "full", but it is still on the chunk data, to singal there is nothing.
    fn is_voxel(&self, voxel: &u16) -> bool {
        return *voxel != 0;
    }
    /// The center of the Mesh, out mesh is defined in src/default_block.rs, just a constant.
    fn get_center(&self) -> [f32; 3] {
        return [0.0, 0.0, 0.0];
    }
    /// The dimensions of the Mesh, out mesh is defined in src/default_block.rs, just a constant.
    fn get_voxel_dimensions(&self) -> [f32; 3] {
        return [1.0, 1.0, 1.0];
    }
    /// The attributes we want to take from out voxels, note that using a lot of different
    /// attributes will likely lead to performance problems and unpredictible behaviour.
    /// We chose these 3 because they are very common, the algorithm does preserve UV data.
    fn all_attributes(&self) -> Vec<bevy::render::mesh::MeshVertexAttribute> {
        return vec![
            Mesh::ATTRIBUTE_POSITION,
            Mesh::ATTRIBUTE_UV_0,
            Mesh::ATTRIBUTE_NORMAL,
        ];
    }
}

/// Simple system to handle inputs for the showcase.
fn input_handler(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Meshy>>,
    time: Res<Time>,
    mut event_writer: EventWriter<ToggleWireframe>,
    mut e: EventWriter<RegenerateMesh>,
) {
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
    if keyboard_input.just_pressed(KeyCode::T) {
        event_writer.send_default();
    }
    if keyboard_input.just_pressed(KeyCode::C) {
        e.send_default();
    }
}

/// Function to toggle wireframe (seeing the vertices and indices of the mesh).
fn toggle_wireframe(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    with: Query<Entity, With<Wireframe>>,
    without: Query<Entity, (Without<Wireframe>, With<Meshy>)>,
    mut events: EventReader<ToggleWireframe>,
) {
    for _ in events.iter() {
        if let Ok(ent) = with.get_single() {
            commands.entity(ent).remove::<Wireframe>();
            for (_, material) in materials.iter_mut() {
                material.base_color.set_a(1.0);
            }
        } else if let Ok(ent) = without.get_single() {
            commands.entity(ent).insert(Wireframe);
            for (_, material) in materials.iter_mut() {
                material.base_color.set_a(0.0);
            }
        }
    }
}

fn input_handler_rotation(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    let t = query.get_single_mut().unwrap().into_inner();
    if keyboard_input.pressed(KeyCode::Space) {
        t.translation += Vec3::Y * SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        t.translation += Vec3::NEG_Y * SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::S) {
        t.translation += t.back() * SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::W) {
        t.translation += t.forward() * SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::A) {
        t.translation += t.left() * SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::D) {
        t.translation += t.right() * SPEED * time.delta_seconds();
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

/// System to regenerate the mesh, but using a different algorithm.
fn regenerate_mesh(
    mut meshy: Query<&mut Meshy>,
    breg: Res<BlockRegistry>,
    mut meshes: ResMut<Assets<Mesh>>,
    mesh_query: Query<&Handle<Mesh>>,
    mut event_reader: EventReader<RegenerateMesh>,
    mut text_query: Query<&mut Text, With<MeshInfo>>,
) {
    for _ in event_reader.iter() {
        let mesh = meshes
            .get_mut(mesh_query.get_single().unwrap())
            .expect("Couldn't get a mut ref to the mesh");
        let grid: Vec<u16> = vec![1; FACTOR * FACTOR * FACTOR];
        let dims: Dimensions = (FACTOR, FACTOR, FACTOR);

        let m = meshy.get_single_mut().unwrap().into_inner();
        let t = text_query.get_single_mut().unwrap().into_inner();
        match m.ma {
            MeshingAlgorithm::Culling => m.ma = MeshingAlgorithm::Naive,
            MeshingAlgorithm::Naive => m.ma = MeshingAlgorithm::Culling,
        }

        (*mesh, m.meta) = mesh_grid(dims, grid, breg.into_inner(), m.ma.clone()).unwrap();

        t.sections[0].value = format!("Press -C- To regenerate the mesh with a different Algorithm\nVertices Count: {}\nMeshing Algorithm: {:?}",mesh.count_vertices(),m.ma);
        return;
    }
}
