use bevy::prelude::*;
mod world;
mod camera;

fn setup_light(mut commands: Commands){
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(50.0, 15.0, 0.0),
        point_light: PointLight {
            intensity: 100000.0,
            range: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}

pub struct StartupPlugin;
impl Plugin for StartupPlugin{
    fn build(&self, app: &mut App){
        app
           .add_plugin(world::WorldPlugin)
           .add_plugin(camera::CameraPlugin)
           .add_startup_system(setup_light);
    }
}
