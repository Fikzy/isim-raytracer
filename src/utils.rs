extern crate nalgebra as na;
use nalgebra::Point2;

const GOLDEN_ANGLE: f32 = 2.399963;

pub fn get_vogel_disk_sample(sample_index: usize, sample_count: usize, phi: f32) -> Point2<f32> {
    let sample_index_f = sample_index as f32;
    let r = ((sample_index_f + 0.5) / sample_count as f32).sqrt();
    let theta = sample_index_f * GOLDEN_ANGLE + phi;
    na::point![theta.cos(), theta.sin()] * r
}
