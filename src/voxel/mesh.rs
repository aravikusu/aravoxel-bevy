use bevy::math::IVec3;
use rand::Rng;
use crate::voxel::voxel::VoxelType;

/// Holds everything relevant for a mesh.
/// Later turned into a Bevy mesh, using all data.
pub struct Mesh {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub colors: Vec<[f32; 4]>,
    pub uvs: Vec<[f32; 2]>,
    pub aos: Vec<u32>,
}

impl Mesh {
    /// Called if clown vomit setting is enabled.
    /// Nukes all textures and replaces it with... you guessed it,
    /// clown vomit.
    pub fn clown_vomit(&mut self) {
        self.colors.clear();
        for _v in &self.vertices {
            let mut rng = rand::thread_rng();
            let color1 = rng.gen_range(0.0..1.0) as f32;
            let color2 = rng.gen_range(0.0..1.0) as f32;
            let color3 = rng.gen_range(0.0..1.0) as f32;
            self.colors.push([color1, color2, color3, 1.0]);
        }
    }

    pub fn set_vertices(&mut self, vertices: Vec<[f32; 3]>) {
        self.vertices.extend(vertices);
    }

    pub fn set_indices(&mut self, indices: Vec<u32>) {
        let idx = self.vertices.len() as u32;
        let vec: Vec<_> = indices.into_iter()
            .map(|i| {
                //println!("{}", i);
                i + idx
            })
            .collect();

        self.indices.extend(vec);
    }

    pub fn set_normals(&mut self, normals: IVec3) {
        let normal = [
            normals.x as f32,
            normals.y as f32,
            normals.z as f32,
        ];

        self.normals.extend([normal; 4])
    }

    pub fn set_uvs(&mut self, voxel_type: &VoxelType) {
        self.uvs.extend(voxel_type.type_to_uvs());
    }

    pub fn add_ao_color(&mut self, alpha: f32) {
        self.colors = self.vertices
            .iter()
            .enumerate()
            .map(|(i, _)| match self.aos[i] {
                0 => [0.1, 0.1, 0.1, alpha],
                1 => [0.3, 0.3, 0.3, alpha],
                2 => [0.5, 0.5, 0.5, alpha],
                3 => [1.0, 1.0, 1.0, alpha],
                _ => [1.0, 1.0, 1.0, alpha],
            }).collect();
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            vertices: Vec::new(),
            normals: Vec::new(),
            indices: Vec::new(),
            colors: Vec::new(),
            uvs: Vec::new(),
            aos: Vec::new()
        }
    }
}