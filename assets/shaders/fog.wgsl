#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings

struct FragmentInput {
    #import bevy_pbr::mesh_vertex_output
}

fn sdSphere(p: vec3<f32>, r: f32) -> f32 {
    return length(p) - r;
}

fn fill(d: f32, color: vec4<f32>) -> vec4<f32> {
    let d2 = 1. - (d * 0.13);
    let alpha = clamp(d2 * d2 * d2, 0., 1.) * color.a;
    let shadow_color = 0.2 * color.rgb;
    let aaf = 0.7 / fwidth(d);
    let c = mix(color.rgb, shadow_color, clamp(d * aaf, 0., 1.));
    return vec4<f32>(c, alpha);
}

let num_scattering_points = 3.;

// fn calculate_light(ray_origin: vec3<f32>, ray_dir: vec3<f32>, ray_length: f32, original_col: vec3<f32>) -> f32 {
//     let in_scatter_point = ray_origin;
//     let step_size = ray_length / (num_scattering_points - 1.)
//     let in_scattered_light = vec3<f32>(0., 0., 0.);
//     let view_ray_optical_depth = 0.;

//     for (var i = 0.; i < num_scattering_points; i = i + 1.) {
//         let sun_ray_length = raySphere(planet_center, a)
//     }
// }

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.3, 0.8, 0.3);
}
