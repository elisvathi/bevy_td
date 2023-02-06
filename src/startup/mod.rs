use bevy::prelude::*;
mod world;
mod camera;
mod spherical_world;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_atmosphere::prelude::*;
use bevy_editor_pls::prelude::*;

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
        .add_plugin(EditorPlugin)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(AtmospherePlugin)
        .add_plugin(spherical_world::SphericalWorldPlugin)
        // .add_plugin(world::WorldPlugin)
        .add_startup_system(setup_light);
    }
}
