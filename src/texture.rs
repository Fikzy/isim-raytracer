extern crate nalgebra as na;
use na::Vector3;

pub trait TextureMaterialTrait {
    fn find_pos(&self, pos: Vector3<f32>) -> (f32, f32, f32);
}

pub struct UniformTexture {
    // texture?
}

impl TextureMaterialTrait for UniformTexture {
    fn find_pos(&self, _pos: Vector3<f32>) -> (f32, f32, f32) {
        (0.0, 0.0, 0.0) // retrieve (kd, ks, ka?) from texture using pos
    }
}
