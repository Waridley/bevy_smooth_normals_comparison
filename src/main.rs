#![feature(iter_array_chunks)]

use bevy::asset::RenderAssetUsages;
use bevy::input::common_conditions::input_toggle_active;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use std::f32::consts::{FRAC_PI_2, PI};

const CAM_DIST: f32 = 20.0;
const SPIRE_HEIGHT: f32 = 10.0;
const CENTER_Y: f32 = SPIRE_HEIGHT / 2.0;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WireframePlugin)
        .insert_resource(WireframeConfig {
            // default_color: Color::BLACK,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                toggle_wireframes,
                move_cam,
                normal_gizmos.run_if(input_toggle_active(false, KeyCode::KeyN)),
            ),
        )
        .run()
}

fn setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let mut mesh_a = generate_demo_mesh();
    let mut mesh_b = mesh_a.clone();
    
    mesh_a.compute_face_weighted_normals();
    mesh_b.compute_smooth_normals();

    info!(a_center_normal=?mesh_a.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap().as_float3().unwrap().first().unwrap());
    info!(b_center_normal=?mesh_b.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap().as_float3().unwrap().first().unwrap());

    let mesh_a = meshes.add(mesh_a);
    let mesh_b = meshes.add(mesh_b);

    let mat = mats.add(StandardMaterial {
        reflectance: 1.0,
        perceptual_roughness: 1.0,
        ..default()
    });

    cmds.spawn((
        DemoMesh,
        Mesh3d(mesh_a),
        MeshMaterial3d(mat.clone()),
        Transform::from_translation(Vec3::new(-4.0, 0.0, 0.0)),
    ));
    cmds.spawn((
        DemoMesh,
        Mesh3d(mesh_b),
        MeshMaterial3d(mat.clone()),
        Transform::from_translation(Vec3::new(4.0, 0.0, 0.0)),
    ));

    cmds.spawn((
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_height: 15.0,
                min_width: 15.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform {
            translation: Vec3::new(0.0, CAM_DIST + CENTER_Y, 0.0),
            rotation: Quat::from_rotation_arc(Vec3::NEG_Z, Vec3::NEG_Y),
            ..default()
        },
    ));

    let light = SpotLight {
        range: 30.0,
        outer_angle: PI / 16.0,
        intensity: 3_000_000.0,
        ..default()
    };
    cmds.spawn((
        light.clone(),
        Transform {
            translation: Vec3::new(-4.0, 20.0, 0.0),
            rotation: Quat::from_rotation_arc(Vec3::NEG_Z, Vec3::NEG_Y),
            ..default()
        },
    ));
    cmds.spawn((
        light.clone(),
        Transform {
            translation: Vec3::new(4.0, 20.0, 0.0),
            rotation: Quat::from_rotation_arc(Vec3::NEG_Z, Vec3::NEG_Y),
            ..default()
        },
    ));

    // Ground to show where lights are shining.
    cmds.spawn((
        Mesh3d(meshes.add(Rectangle::new(30.0, 30.0).mesh().build())),
        MeshMaterial3d(mats.add(Color::srgb(0.25, 0.25, 0.25))),
        Transform {
            translation: Vec3::NEG_Y,
            rotation: Quat::from_rotation_arc(Vec3::Z, Vec3::Y),
            ..default()
        },
    ));
}

#[derive(Component)]
struct DemoMesh;

fn generate_demo_mesh() -> Mesh {
    let verts = vec![
        Vec3::new(0.0, 10.0, 0.0),  // spire peak (0)
        Vec3::new(-1.0, 0.0, -1.0), // spire back left (1)
        Vec3::new(-0.2, 8.0, 0.0),  // spire left center (2)
        Vec3::new(-1.0, 0.0, 1.0),  // spire front left (3)
        Vec3::new(1.0, 0.0, 1.0),   // spire front right (4)
        Vec3::new(1.0, 0.0, -1.0),  // spire back right (5)
        Vec3::new(-2.0, 0.0, -1.0), // gable back left (6)
        Vec3::new(-2.0, 8.0, 0.0),  // gable left center (7)
        Vec3::new(-2.0, 0.0, 1.0),  // gable front left (8)
    ];
    #[rustfmt::skip]
	let indices = Indices::U16(vec![
		0, 5, 1, // spire peak
		0, 1, 2, // spire left back
		0, 2, 3, // spire left front
		0, 3, 4, // spire front
		0, 4, 5, // spire right
	
		2, 1, 7, // gable back tri A
		7, 1, 6, // gable back tri B
		2, 7, 3, // gable front tri A
		3, 7, 8, // gable front tri B
		7, 6, 8, // gable end
	]);
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, verts)
    .with_inserted_indices(indices)
}

fn move_cam(
    keys: Res<ButtonInput<KeyCode>>,
    mut cam: Single<&mut Transform, With<Camera>>,
    t: Res<Time>,
) {
    let (mut yaw, mut pitch, _roll) = cam.rotation.to_euler(EulerRot::YXZ);
    if keys.pressed(KeyCode::ArrowLeft) {
        yaw -= t.delta_secs();
    }
    if keys.pressed(KeyCode::ArrowRight) {
        yaw += t.delta_secs();
    }
    if keys.pressed(KeyCode::ArrowUp) {
        pitch = (pitch - t.delta_secs()).clamp(-FRAC_PI_2, FRAC_PI_2);
    }
    if keys.pressed(KeyCode::ArrowDown) {
        pitch = (pitch + t.delta_secs()).clamp(-FRAC_PI_2, FRAC_PI_2);
    }
    if keys.just_pressed(KeyCode::KeyX) {
        yaw = -FRAC_PI_2;
        pitch = 0.0;
    }
    if keys.just_pressed(KeyCode::KeyY) {
        yaw = 0.0;
        pitch = -FRAC_PI_2;
    }
    if keys.just_pressed(KeyCode::KeyZ) {
        yaw = 0.0;
        pitch = 0.0;
    }
    let new_rot = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
    cam.rotation = new_rot;
    cam.translation = (new_rot * Vec3::Z * CAM_DIST) + Vec3::Y * CENTER_Y;
}

fn toggle_wireframes(keys: Res<ButtonInput<KeyCode>>, mut cfg: ResMut<WireframeConfig>) {
    if keys.just_pressed(KeyCode::KeyW) {
        cfg.global = !cfg.global;
    }
}

fn normal_gizmos(
    objects: Query<(&Mesh3d, &GlobalTransform), With<DemoMesh>>,
    meshes: Res<Assets<Mesh>>,
    mut gizmos: Gizmos,
) {
    for (mesh, xform) in &objects {
        let mesh = meshes.get(mesh).unwrap();
        let positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .unwrap()
            .as_float3()
            .unwrap();
        let normals = mesh
            .attribute(Mesh::ATTRIBUTE_NORMAL)
            .unwrap()
            .as_float3()
            .unwrap();
        for (pos, norm) in positions.iter().zip(normals) {
            let pos = Vec3::from_array(*pos) + xform.translation();
            let norm = xform.rotation() * Vec3::from_array(*norm);
            gizmos.arrow(pos, pos + norm, bevy::color::palettes::css::ORANGE);
        }
    }
}
