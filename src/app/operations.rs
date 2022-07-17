use image::{DynamicImage, GrayImage};

pub fn grayscale(image: &DynamicImage) -> Option<GrayImage> {
    let buf_size = image.width() * image.height();
    let mut buf = Vec::with_capacity(buf_size as usize);
    for pixel in image.to_rgb8().pixels() {
        let grayscale = 0.3 * pixel[0] as f32 + 0.59 * pixel[1] as f32 + 0.11 * pixel[2] as f32;
        let v = grayscale.round() as u8;
        buf.push(v);
    }

    GrayImage::from_vec(image.width(), image.height(), buf)
}

pub fn invert(image: &DynamicImage) -> Option<GrayImage> {
    if let Some(image) = image.as_luma8() {
        let mut result = image.clone();

        for pixel in result.pixels_mut() {
            pixel[0] = 255u8 - pixel[0];
        }

        Some(result)
    } else {
        None
    }
}
