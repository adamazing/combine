use bevy::{prelude::*, render::camera::{DepthCalculation, ScalingMode, WindowOrigin}, window::WindowMode};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    debug!("Spawning camera");
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            depth_calculation: DepthCalculation::ZDifference,
            scaling_mode: ScalingMode::None,
            window_origin: WindowOrigin::BottomLeft,
            ..Default::default()
        },
        ..Default::default()
    });
}
