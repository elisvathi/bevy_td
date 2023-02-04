use bevy::prelude::*;
use perlin2d::PerlinNoise2D;

pub struct WorldPlugin;
const AMPLITUDE: f32 = 5.0;
const DENSITY : f32 = 0.3;
const WIDTH: i32 = 30;
const HEIGHT: i32 = 30;

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
        let scale = 200.0;
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
                let height = perlin_obj.get_noise(x as f64, y as f64) as f32 + amplitude;
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
    trees: &Vec<Handle<Scene>>
) {

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
    let random_scale = rand::random::<f32>() * 0.05 + 0.01;
    let random_rotation = rand::random::<f32>() * 360.0;
    let transform = Transform {
        translation: Vec3::new(0.5, 1.0, 0.5),
        rotation: Quat::from_rotation_y(random_rotation),
        scale: Vec3::new(random_scale, random_scale, random_scale),
        ..default()
    };
    let entity = command
        .spawn(PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(
                Vec3::new(cell.x as f32, cell.height, cell.y as f32),
            ),
            ..Default::default()
        }).insert(cell).id();
    // commands.entity()
    let chance = rand::random::<f32>() < DENSITY;

    if chance {
        command.entity(entity).with_children(|parent| {
            parent.spawn(SceneBundle{
                scene: pick_random(trees),
                transform, 
                ..default()
            });
        });
    }
}

fn spawn_world(
    world: World,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    let cells = world.build_cells();
    commands.spawn(world);
    let trees = load_tree(&asset_server);
    for cell in cells {
        spawn_cell(cell, commands, meshes, materials, &trees);
    }
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

fn load_tree(asset_server: &Res<AssetServer>)-> Vec<Handle<Scene>> {
    vec![asset_server.load("tree_normal_1.gltf#Scene0")]
    // let tree_long_1 = asset_server.load("tree_long_1.gltf#Scene0");
    // let tree_thick_1 = asset_server.load("tree_thick_1.gltf#Scene0");
    // let rock_01 = asset_server.load("rock_01.gltf#Scene0");
    // vec![tree_normal_1]
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(world_startup_system);
    }
}
