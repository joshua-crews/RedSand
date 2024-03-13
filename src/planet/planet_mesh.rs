use bevy::{
    math::Vec3Swizzles,
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use crate::planet;

const HEIGHT_MAP_SCALE: f32 = 0.25;

pub fn spawn_face(
    resolution: u32,
    size: f32,
    direction: Vec3,
    uv_scale: f32,
    height_map: &Image,
) -> Mesh {
    return Mesh::from(planet::PlanetMesh {
        resolution,
        size,
        direction,
        uv_scale,
        height_map: height_map.clone(),
    })
    .with_generated_tangents()
    .unwrap();
}

impl From<planet::PlanetMesh> for Mesh {
    fn from(planet: planet::PlanetMesh) -> Self {
        let (vert_lists, triangle_lists): (Vec<Vec<Vec3>>, Vec<Vec<u32>>) =
            face(planet.resolution, planet.direction, planet.size);

        let vertices = vert_lists
            .iter()
            .flat_map(|v| v.iter().map(|v| [v.x, v.y, v.z]))
            .collect::<Vec<[f32; 3]>>();

        let triangle_list = triangle_lists
            .iter()
            .enumerate()
            .flat_map(|(face_id, list)| {
                list.iter().map(move |local_idx| {
                    let num_indices = planet.resolution * planet.resolution;
                    local_idx + face_id as u32 * num_indices
                })
            })
            .collect::<Vec<u32>>();

        let mut uvs: Vec<[f32; 2]> =
            Vec::with_capacity((planet.resolution * planet.resolution) as usize);
        let mut height_uvs: Vec<[f32; 2]> =
            Vec::with_capacity((planet.resolution * planet.resolution) as usize);
        for y in 0..planet.resolution {
            for x in 0..planet.resolution {
                let u = x as f32 / (planet.resolution - 1) as f32;
                let v = y as f32 / (planet.resolution - 1) as f32;
                uvs.push([u * planet.uv_scale, v * planet.uv_scale]);
                height_uvs.push([u, v]);
            }
        }

        let deformed_vertices = vertices
            .iter()
            .zip(vertices.iter())
            .zip(height_uvs.iter())
            .map(|((&vertex, &normal), &uv)| {
                deform_with_heightmap(
                    &vertex.into(),
                    &normal.into(),
                    &uv.into(),
                    planet.height_map.clone().into(),
                )
            })
            .collect::<Vec<Vec3>>();

        let normals = compute_vertex_normals(&deformed_vertices, &triangle_list);

        let mut mesh: Mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(triangle_list.clone())));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, deformed_vertices.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        return mesh;
    }
}

fn face(resolution: u32, local_up: Vec3, size: f32) -> (Vec<Vec<Vec3>>, Vec<Vec<u32>>) {
    let axis_a = local_up.yzx();
    let axis_b = local_up.cross(axis_a);

    let mut vertices = Vec::with_capacity(resolution as usize * resolution as usize);
    let mut triangles =
        Vec::with_capacity((resolution as usize - 1) * (resolution as usize - 1) * 6);

    for y in 0..resolution {
        for x in 0..resolution {
            let i = x + y * resolution;
            let percent_x = x as f32 / (resolution - 1) as f32;
            let percent_y = y as f32 / (resolution - 1) as f32;

            let point_on_unit_cube: Vec3 =
                local_up + (percent_x - 0.5) * 2.0 * axis_a + (percent_y - 0.5) * 2.0 * axis_b;
            let point_on_unit_sphere: Vec3 = point_on_unit_cube.normalize() * size;

            vertices.push(point_on_unit_sphere);

            if x != resolution - 1 && y != resolution - 1 {
                triangles.push(i);
                triangles.push(i + resolution + 1);
                triangles.push(i + resolution);

                triangles.push(i);
                triangles.push(i + 1);
                triangles.push(i + resolution + 1);
            }
        }
    }
    (vec![vertices], vec![triangles])
}

fn sample_height_map(uv: Vec2, height_map: &Image) -> f32 {
    let width = height_map.texture_descriptor.size.width as f32;
    let height = height_map.texture_descriptor.size.height as f32;

    let x_pos = (uv.x * width).clamp(0.0, width - 1.0) as usize;
    let y_pos = (uv.y * height).clamp(0.0, height - 1.0) as usize;
    let buffer_pos = (y_pos * width as usize + x_pos) * 4; // *4 for RGBA even if we only use R

    if buffer_pos < height_map.data.len() {
        let height_value = height_map.data[buffer_pos];
        height_value as f32 / 255.0
    } else {
        0.0
    }
}

fn deform_with_heightmap(vertex: &Vec3, normal: &Vec3, uv: &Vec2, height_map: Image) -> Vec3 {
    let height_sample = sample_height_map(*uv, &height_map);
    return *vertex + *normal * (height_sample * HEIGHT_MAP_SCALE);
}

fn compute_triangle_normal(p0: Vec3, p1: Vec3, p2: Vec3) -> Vec3 {
    let v0 = p1 - p0;
    let v1 = p2 - p0;
    let normal = v0.cross(v1).normalize();
    return normal;
}

fn compute_vertex_normals(vertices: &Vec<Vec3>, triangles: &Vec<u32>) -> Vec<Vec3> {
    let mut normals = vec![Vec3::ZERO; vertices.len()];
    let mut face_normals = Vec::with_capacity(triangles.len() / 3);

    for triangle in triangles.chunks(3) {
        let normal = compute_triangle_normal(
            vertices[triangle[0] as usize],
            vertices[triangle[1] as usize],
            vertices[triangle[2] as usize],
        );
        face_normals.push(normal);
    }

    for (i, &indice) in triangles.iter().enumerate() {
        let vertex_index = indice as usize;
        normals[vertex_index] += face_normals[i / 3];
    }

    for normal in &mut normals {
        *normal = normal.normalize();
    }

    return normals;
}
