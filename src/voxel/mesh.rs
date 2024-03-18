use rand::Rng;
use crate::voxel::voxel::VoxelType;

/// Holds everything relevant for a mesh.
/// Later turned into a Bevy mesh, using all data.
pub struct Mesh {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub colors: Vec<[f32; 4]>,
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

    pub fn set_normals(&mut self, normals: [f32; 3]) {
        self.normals.extend([normals; 4]);
    }

    pub fn colorize_vertices(&mut self, voxel_type: &VoxelType) {
        self.colors.extend([voxel_type.type_to_color(); 4]);
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            vertices: Vec::new(),
            normals: Vec::new(),
            indices: Vec::new(),
            colors: Vec::new(),
        }
    }
}