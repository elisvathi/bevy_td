use bevy::{prelude::*, render::render_resource::AsBindGroup, reflect::TypeUuid};
use bevy_mod_picking::PickableBundle;
use perlin2d::PerlinNoise2D;

pub struct WorldPlugin;
const AMPLITUDE: f32 = 5.0;
const DENSITY: f32 = 0.3;
const WIDTH: i32 = 50;
const HEIGHT: i32 = 50;
const SCALE: f64 = 200.0;

#[derive(Component)]
struct Cell {
    pub x: i32,
    pub y: i32,
    pub height: f32,
}

#[derive(Component)]
struct World {
    pub width: i32,
    pub height: i32,
}

impl World {
    fn build_cells(&self) -> Vec<Cell> {
        let mut cells = Vec::new();
        let amplitude = AMPLITUDE;
        let scale = SCALE;
        let octaves = 5;
        let perlin_obj = PerlinNoise2D::new(
            octaves,
            amplitude as f64,
            2.5,
            1.0,
            2.0,
            (scale, scale),
            1.0,
            101,
        );
        for x in 0..self.width {
            for y in 0..self.height {
                let height = perlin_obj.get_noise(x as f64, y as f64) as f32;
                cells.push(Cell {
                    x: x - self.width / 2,
                    y: y - self.height / 2,
                    height,
                });
            }
        }
        cells
    }
}

fn spawn_cell(
    cell: Cell,
    command: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    trees: &Vec<Handle<Scene>>,
) {
    if cell.height < -3.0 {
         return;
    }

    fn pick_random<T: Clone>(data: &Vec<T>) -> T {
        let index = rand::random::<usize>() % data.len();
        data[index].clone()
    }

    let mesh = meshes.add(Mesh::from(shape::Box {
        min_x: 0.0,
        max_x: 1.0,
        min_y: 0.0,
        max_y: 1.0,
        min_z: 0.0,
        max_z: 1.0,
    }));
    let material = materials.add(Color::rgb(0.2, 0.9, 0.3).into());
    let random_rotation = rand::random::<f32>() * 360.0;
    // random number from 1 to 10
    let cell_tree_density: i32 = (rand::random::<f32>() * 10.0 + 1.0) as i32;
    let entity = command
        .spawn(PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(
                Vec3::new(cell.x as f32, cell.height, cell.y as f32),
            ),
            ..Default::default()
        })
        .insert(cell)
        .insert(PickableBundle::default())
        .id();
    let chance = rand::random::<f32>() < DENSITY;
    if chance {
        let selected = pick_random(trees);
        for _ in 0..cell_tree_density {
            let random_scale = rand::random::<f32>() * 0.02 + 0.002;
            let transform = Transform {
                translation: Vec3::new(rand::random::<f32>(), 1.0, rand::random::<f32>()),
                rotation: Quat::from_rotation_y(random_rotation),
                scale: Vec3::new(random_scale, random_scale, random_scale),
                ..default()
            };
            command.entity(entity).with_children(|parent| {
                parent
                    .spawn(SceneBundle {
                        scene: selected.clone(),
                        transform,
                        ..default()
                    })
                    .insert(PickableBundle::default());
            });
        }
    }
}

fn spawn_world(
    world: World,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let cells = world.build_cells();
    commands.spawn(world);
    let trees = load_tree(&asset_server);
    for cell in cells {
        spawn_cell(cell, commands, meshes, materials, &trees);
    }
}

fn spawn_water(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<CustomMaterial>> ){
    let mesh = meshes.add(Mesh::from(shape::Box {
        min_x: -WIDTH as f32 / 2.0,
        max_x: WIDTH as f32 / 2.0,
        min_y: -3.0,
        max_y: -2.0,
        min_z: -HEIGHT as f32 / 2.0,
        max_z: HEIGHT as f32 / 2.0
    }));
    commands.spawn(MaterialMeshBundle{
        mesh,
        material: materials.add(CustomMaterial{}),
        ..default()
    });
}

fn world_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let w = World {
        width: WIDTH,
        height: HEIGHT,
    };
    spawn_world(w, &mut commands, &mut meshes, &mut materials, asset_server);
}

fn load_tree(asset_server: &Res<AssetServer>) -> Vec<Handle<Scene>> {

    vec![
        "models/tree_normal_1.gltf#Scene0",
        "models/tree_thick_1.gltf#Scene0",
    ].iter()
        .map(|x| x.to_string())
        .map(|path| asset_server.load(path))
        .collect()
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(MaterialPlugin::<CustomMaterial>::default())
            .add_startup_system(world_startup_system)
            .add_startup_system(spawn_water);
    }
}

#[derive(AsBindGroup, TypeUuid, Clone, Debug)]
#[uuid = "717819b6-6f04-4ef8-8cda-9d5268011135"]
pub struct CustomMaterial {
}


impl Material for CustomMaterial{

    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/fog.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
