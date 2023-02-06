use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh}};

const RADIUS: f32 = 100.0;
const RESOLUTION: u32 = 30;
pub struct SphericalWorldPlugin;


fn setup_face(
    calc_point : fn(u32, u32, f32) -> Vec3,
    inverse_face: bool,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let mat = materials.add(Color::rgb(0.2, 0.9, 0.3).into());
    let mut vertices = Vec::<Vec3>::new() ;
    let mut indices = Vec::<u32>::new();
    let mut normals = Vec::<Vec3>::new();
    let step = 2. * RADIUS / RESOLUTION as f32;
    for i in 0..RESOLUTION{
        for j in 0..RESOLUTION{
            // let mut point = Vec3::new(-RADIUS + i as f32 * step, RADIUS, -RADIUS + j as f32 * step);
            let mut point = calc_point(i, j, step);
            let distance = point.length();
            point *= RADIUS / distance;
            vertices.push(point);
            normals.push(point);
        }
    }
    // build indices
    for i in 0..RESOLUTION-1{
        for j in 0..RESOLUTION-1{
            let index = (i * RESOLUTION + j) as u32;
            if inverse_face {
                indices.push(index);
                indices.push(index + RESOLUTION);
                indices.push(index + 1);
                indices.push(index + RESOLUTION);
                indices.push(index + RESOLUTION + 1);
                indices.push(index + 1);
            } else {
                indices.push(index);
                indices.push(index + 1);
                indices.push(index + RESOLUTION);
                indices.push(index + RESOLUTION);
                indices.push(index + 1);
                indices.push(index + RESOLUTION + 1);
            }
        }
    }
    let mut m = Mesh::new(PrimitiveTopology::TriangleList);
    m.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    m.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    m.set_indices(Some(mesh::Indices::U32(indices)));
    commands.spawn(PbrBundle {
        mesh: meshes.add(m),
        material: mat,
        ..Default::default()
    });

}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // top
    setup_face( |i, j, step| Vec3::new(-RADIUS + i as f32 * step, RADIUS, -RADIUS + j as f32 * step), false, &mut commands, &mut meshes, &mut materials);
    // bottom
    setup_face( |i, j, step| Vec3::new(-RADIUS + i as f32 * step, -RADIUS, -RADIUS + j as f32 * step), true, &mut commands, &mut meshes, &mut materials);
    // front
    setup_face( |i, j, step| Vec3::new(-RADIUS + i as f32 * step, -RADIUS + j as f32 * step, -RADIUS), false, &mut commands, &mut meshes, &mut materials);
    // back
    setup_face( |i, j, step| Vec3::new(-RADIUS + i as f32 * step, -RADIUS + j as f32 * step, RADIUS), true, &mut commands, &mut meshes, &mut materials);
    // left
    setup_face( |i, j, step| Vec3::new(-RADIUS, -RADIUS + j as f32 * step, -RADIUS + i as f32 * step), true, &mut commands, &mut meshes, &mut materials);
    // right
    setup_face( |i, j, step| Vec3::new(RADIUS, -RADIUS + j as f32 * step, -RADIUS + i as f32 * step), false, &mut commands, &mut meshes, &mut materials);
}

impl Plugin for SphericalWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}
