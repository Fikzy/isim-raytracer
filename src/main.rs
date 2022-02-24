use object::SceneObjectTrait;

mod object;
mod ray;
mod texture;

fn main() {
    let img = image::ImageBuffer::from_fn(32, 32, |x, y| {
        if x % 2 == 0 || y % 2 == 0 {
            image::Rgb([0u8, 0u8, 0u8])
        } else {
            image::Rgb([255u8, 255u8, 255u8])
        }
    });

    img.save("test.ppm").unwrap();
}
