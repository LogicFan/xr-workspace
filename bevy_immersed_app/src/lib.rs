#[cfg(not(target_os = "android"))]
compile_error!("this crate is intended to be compiled against android");

mod screen;

use bevy::prelude::*;

use bevy::render::render_resource::TextureFormat;
use bevy::tasks::{ComputeTaskPool, TaskPoolBuilder};
use bevy_oxr::bevy_openxr::add_xr_plugins;
use bevy_oxr::bevy_openxr::init::OxrInitPlugin;
use bevy_oxr::bevy_openxr::types::{AppInfo, Version};

#[bevy_main]
pub fn main() {
    ComputeTaskPool::get_or_init(|| {
        TaskPoolBuilder::default()
            .num_threads(10)
            .on_thread_spawn(|| {
                let ctx = ndk_context::android_context();
                let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
                vm.attach_current_thread_permanently();
            })
            .thread_name("Compute Task Pool".to_string())
            .build()
    });

    App::new()
        .add_plugins(add_xr_plugins(DefaultPlugins).set(OxrInitPlugin {
            app_info: AppInfo {
                name: "XR Workspace".into(),
                version: Version(0, 0, 1),
            },
            exts: Default::default(),
            blend_modes: Default::default(),
            backends: Default::default(),
            formats: Some(vec![TextureFormat::Rgba8UnormSrgb]),
            resolutions: Default::default(),
            synchronous_pipeline_compilation: Default::default(),
        }))
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::rgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
