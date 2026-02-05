use macroquad::prelude::Vec3;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub mod vec3_vec {
    use super::*;

    pub fn serialize<S>(vec: &Vec<Vec3>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let arrays: Vec<[f32; 3]> = vec.iter().map(|v| [v.x, v.y, v.z]).collect();
        arrays.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Vec3>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let arrays: Vec<[f32; 3]> = Vec::deserialize(deserializer)?;
        Ok(arrays
            .into_iter()
            .map(|[x, y, z]| Vec3::new(x, y, z))
            .collect())
    }
}
