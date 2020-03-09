pub mod imageprocessing;
use crate::imageprocessing::{BoxBlur, EdgeDetect, ImageType, Invert};

use tonic::{Code, Status};

use image::RgbaImage;

use std::convert::TryFrom;

pub trait ProcessImage {
    fn process_image(&self, image: &mut RgbaImage) -> Result<(), Status>;
}

impl Invert {
    fn invert(&self, image: &mut RgbaImage) -> Result<(), Status> {
        use image_processing::pixel_ops::invert_mut;

        invert_mut(image);
        Ok(())
    }
}
impl ProcessImage for Invert {
    fn process_image(&self, image: &mut RgbaImage) -> Result<(), Status> {
        self.invert(image).unwrap();
        Ok(())
    }
}

impl EdgeDetect {
    fn edge_detect(&self, image: &mut RgbaImage) -> Result<(), Status> {
        use image::{ConvertBuffer, GrayImage};
        use image_processing::edge_detection::normal_sobel_mut;
        use std::mem;
        let mut new_image: GrayImage = image.convert();

        if self.threshold > std::u8::MAX as u32 {
            return Err(Status::new(
                Code::InvalidArgument,
                format!(
                    "Threshold needs to be between 0 and 255 inclusive. It was: {}",
                    self.threshold
                ),
            ));
        }
        normal_sobel_mut(&mut new_image, self.threshold as u8);
        mem::swap(image, &mut new_image.convert());
        Ok(())
    }
}

impl ProcessImage for EdgeDetect {
    fn process_image(&self, image: &mut RgbaImage) -> Result<(), Status> {
        self.edge_detect(image)?;
        Ok(())
    }
}

impl BoxBlur {
    fn box_blur(&self, image: &mut RgbaImage) -> Result<(), Status> {
        use image_processing::blur::{box_filter_mut, MeanKernel};
        if self.kernel_width % 2 == 0 {
            return Err(Status::new(
                Code::InvalidArgument,
                format!(
                    "Kernel size was not an odd value. It was: {}",
                    self.kernel_width
                ),
            ));
        }
        box_filter_mut(MeanKernel::new(self.kernel_width), image);
        Ok(())
    }
}

impl ProcessImage for BoxBlur {
    fn process_image(&self, image: &mut RgbaImage) -> Result<(), Status> {
        self.box_blur(image)?;
        Ok(())
    }
}

impl TryFrom<i32> for ImageType {
    type Error = Status;

    fn try_from(value: i32) -> Result<Self, Status> {
        match value {
            0 => Ok(ImageType::Rgba),
            1 => Ok(ImageType::Rgb),
            2 => Ok(ImageType::Gray),
            3 => Ok(ImageType::Grayalpha),
            _ => Err(Status::new(
                Code::InvalidArgument,
                format!(
                    "Image type can only be a value between 0 and 3, inclusive. Value was: {}",
                    value
                ),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageBuffer;
    #[test]
    fn test_invert() {
        // let mut image = image::open("./test_images/empire-test.jpg")
        //     .unwrap()
        //     .to_rgba();
        let raw_image = vec![25, 89, 199, 255, 0, 255, 39, 255];
        let manual_invert_image = vec![230, 166, 56, 255, 255, 0, 216, 255];
        let mut image: RgbaImage = ImageBuffer::from_vec(2, 1, raw_image).unwrap();
        let invert_obj = Invert {};
        invert_obj.invert(&mut image).unwrap();
        let inverted_image = image.into_vec();

        for (inverted, manually_inverted) in inverted_image.iter().zip(manual_invert_image.iter()) {
            assert_eq!(
                manually_inverted, inverted,
                "source truth: {}, output of invert: {}",
                manually_inverted, inverted,
            );
        }
    }
    // #[test]
    // fn test_edge_detect() {
    //     let raw_image = vec![25, 89, 199, 255, 0, 255, 39, 255];
    //     let manual_edge_detected_image = vec![230, 166, 56, 255, 255, 0, 216, 255];
    //     let mut image: RgbaImage = ImageBuffer::from_vec(2, 1, raw_image).unwrap();
    // }

    #[test]
    fn basic_test_box_blur() {
        let raw_image = vec![25, 89, 199, 255, 0, 255, 39, 255];
        let mut image: RgbaImage = ImageBuffer::from_vec(2, 1, raw_image).unwrap();
        let box_blur_obj = BoxBlur { kernel_width: 3 };
        box_blur_obj.box_blur(&mut image).unwrap();

        let manual_blurred_image = vec![17, 144, 146, 255, 8, 200, 92, 255];
        let blurred_image = image.into_vec();

        for (blurred, manually_blurred) in blurred_image.iter().zip(manual_blurred_image.iter()) {
            assert_eq!(
                manually_blurred, blurred,
                "source truth: {}, output of box blur: {}",
                manually_blurred, blurred,
            );
        }
    }
}
