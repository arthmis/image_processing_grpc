pub mod imageprocessing;
use crate::imageprocessing::{BoxBlur, EdgeDetect, Invert};

use image::RgbaImage;

pub trait ProcessImage {
    fn process_image(&self, image: &mut RgbaImage);
}

impl Invert {
    fn invert(&self, image: &mut RgbaImage) {
        use image_processing::pixel_ops::invert_mut;

        invert_mut(image);
    }
}
impl ProcessImage for Invert {
    fn process_image(&self, image: &mut RgbaImage) {
        self.invert(image);
    }
}

impl EdgeDetect {
    fn edge_detect(&self, image: &mut RgbaImage) {
        use image::{ConvertBuffer, GrayImage};
        use image_processing::edge_detection::normal_sobel_mut;
        use std::mem;
        let mut new_image: GrayImage = image.convert();

        normal_sobel_mut(&mut new_image, self.threshold as u8);
        mem::swap(image, &mut new_image.convert());
    }
}

impl ProcessImage for EdgeDetect {
    fn process_image(&self, image: &mut RgbaImage) {
        self.edge_detect(image);
    }
}

impl BoxBlur {
    fn box_blur(&self, image: &mut RgbaImage, kernel_size: u32) {
        use image_processing::blur::{box_filter_mut, MeanKernel};

        box_filter_mut(MeanKernel::new(kernel_size), image);
    }
}

impl ProcessImage for BoxBlur {
    fn process_image(&self, image: &mut RgbaImage) {
        self.box_blur(image, self.kernel_width);
    }
}
