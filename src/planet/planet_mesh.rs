use bevy::{
    math::Vec3Swizzles,
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::{PrimitiveTopology},
    },
};
use unzip3::Unzip3;
use std::f32::consts::PI;
use crate::planet;

impl From<planet::PlanetMesh> for Mesh {
    fn from(planet: planet::PlanetMesh) -> Self {
        let directions = [
            Vec3::Y,
            Vec3::NEG_Y,
            Vec3::NEG_X,
            Vec3::X,
            Vec3::Z,
            Vec3::NEG_Z,
        ];

        let (vert_lists, triangle_lists, uv_lists): (
            Vec<Vec<Vec3>>,
            Vec<Vec<u32>>,
            Vec<Vec<Vec2>>
        ) = directions
            .iter()
            .map(|direction| {
                face(planet.resolution, *direction, planet.size)
            })
            .unzip3();

        let vertices = vert_lists
            .iter()
            .flat_map(|v| v.iter().map(|v| [v.x, v.y, v.z]))
            .collect::<Vec<[f32; 3]>>();

        let triangle_list = triangle_lists
            .iter()
            .enumerate()
            .flat_map(|(face_id, list)| {
                list.iter().map(move |local_idx| {
                    let num_indices = planet.resolution
                        * planet.resolution;
                    local_idx + face_id as u32 * num_indices
                })
            })
            .collect::<Vec<u32>>();

        let uvs = uv_lists
            .iter()
            .flat_map(|uvs| uvs.iter().map(|uv| [uv.x, uv.y]))
            .collect::<Vec<[f32; 2]>>();

        let mut mesh: Mesh= Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(triangle_list.clone())));
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION, vertices.clone(),
        );

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL, vertices.clone(),
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        return mesh;
    }
}

fn face(resolution: u32, local_up: Vec3, size: f32) -> (Vec<Vec3>, Vec<u32>, Vec<Vec2>) {
    let axis_a = local_up.yzx();
    let axis_b = local_up.cross(axis_a);

    let mut vertices = Vec::with_capacity(resolution as usize * resolution as usize);
    let mut triangles = Vec::with_capacity((resolution as usize - 1) * (resolution as usize - 1) * 6);
    let mut uvs = Vec::with_capacity(resolution as usize * resolution as usize);

    for y in 0..resolution {
        for x in 0..resolution {
            let i = x + y * resolution;
            let percent_x = x as f32 / (resolution - 1) as f32;
            let percent_y = y as f32 / (resolution - 1) as f32;

            let mut point_on_unit_cube: Vec3 = local_up
                + (percent_x - 0.5) * 2.0 * axis_a
                + (percent_y - 0.5) * 2.0 * axis_b;
            let mut point_on_unit_sphere: Vec3 = point_on_unit_cube.normalize() * size;

            vertices.push(point_on_unit_sphere);

            let uv_x = 0.5 + point_on_unit_sphere.x.atan2(point_on_unit_sphere.z) / (2.0 * PI);
            let uv_y = 0.5 - point_on_unit_sphere.y.asin() / PI;

            // Correct the UVs at the seam
            let corrected_uv_x = if uv_x < 0.0 { uv_x + 1.0 } else { uv_x };
            let uv = Vec2::new(corrected_uv_x, uv_y);
            uvs.push(uv);

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
    (vertices, triangles, uvs)
}