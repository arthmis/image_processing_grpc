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
    fn box_blur(&self, image: &mut RgbaImage, kernel_size: u32) -> Result<(), Status> {
        use image_processing::blur::{box_filter_mut, MeanKernel};
        if kernel_size % 2 == 0 {
            return Err(Status::new(
                Code::InvalidArgument,
                format!(
                    "Kernel size was not an odd value. It was: {}",
                    self.kernel_width
                ),
            ));
        }
        box_filter_mut(MeanKernel::new(kernel_size), image);
        Ok(())
    }
}

impl ProcessImage for BoxBlur {
    fn process_image(&self, image: &mut RgbaImage) -> Result<(), Status> {
        self.box_blur(image, self.kernel_width)?;
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
