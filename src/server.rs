use image::imageops::thumbnail;
use image::{ImageBuffer, RgbaImage};
use std::convert::TryFrom;
use tonic::{transport::Server, Code, Request, Response, Status};
#[derive(Debug, Default)]
struct ImageProcessingService;

use process_image;
use process_image::imageprocessing::image_processing_server::{
    ImageProcessing, ImageProcessingServer,
};
use process_image::imageprocessing::{Image, ImageParameters, ImageType, ThumbnailImage};
use process_image::ProcessImage;

#[tonic::async_trait]
impl ImageProcessing for ImageProcessingService {
    async fn process_image(
        &self,
        request: Request<ImageParameters>,
    ) -> Result<Response<Image>, Status> {
        let image_object = request.into_inner();

        if image_object.image.data.is_empty() {
            return Err(Status::new(
                Code::InvalidArgument,
                "Data variable is empty.",
            ));
        }
        let image_width = image_object.image.width;
        let image_height = image_object.image.height;

        let channel_count: u32 = match ImageType::try_from(image_object.image.image_type)? {
            ImageType::Rgba => 4,
            ImageType::Rgb => 3,
            ImageType::Grayalpha => 2,
            ImageType::Gray => 1,
        };

        if image_width * image_height * channel_count != image_object.image.data.len() as u32 {
            return Err(
                Status::new(
                Code::InvalidArgument,
                format!(
                        "Width, height, or image_type is not appropriate value\nwidth: {}\nheight: {}\nchannel count: {}.\n width * height * channel_count has to be equal to bytes buffer length.", 
                        image_width,
                        image_height,
                        channel_count
                    )
                )
            );
        }

        let mut user_image: RgbaImage =
            ImageBuffer::from_vec(image_width, image_height, image_object.image.data).unwrap();

        let operations: Vec<Box<dyn ProcessImage>> = {
            let mut operations = Vec::new();

            if let Some(operation) = image_object.invert {
                operations.push(Box::new(operation) as Box<dyn ProcessImage>);
            }

            if let Some(operation) = image_object.box_blur {
                operations.push(Box::new(operation) as Box<dyn ProcessImage>);
            }

            if let Some(operation) = image_object.edge_detect {
                operations.push(Box::new(operation) as Box<dyn ProcessImage>);
            }

            operations
        };

        for operation in operations.iter() {
            operation.process_image(&mut user_image)?;
        }

        let processed_image = Image {
            width: image_width,
            height: image_height,
            image_type: image_object.image.image_type,
            data: user_image.into_vec(),
        };

        Ok(Response::new(processed_image))
    }

    async fn create_thumbnail(
        &self,
        request: Request<ThumbnailImage>,
    ) -> Result<Response<Image>, Status> {
        let image_object = request.into_inner();

        if image_object.image.data.is_empty() {
            return Err(Status::new(
                Code::InvalidArgument,
                "Data variable is empty.",
            ));
        }

        let image_width = image_object.image.width;
        let image_height = image_object.image.height;
        let new_height = image_object.new_height;
        let new_width = image_object.new_width;

        if new_width > image_width || new_height > image_height {
            return Err(Status::new(
                Code::InvalidArgument,
                "new width or height shouldn't be greater than the image width or height.",
            ));
        }

        let channel_count: u32 = match ImageType::try_from(image_object.image.image_type)? {
            ImageType::Rgba => 4,
            ImageType::Rgb => 3,
            ImageType::Grayalpha => 2,
            ImageType::Gray => 1,
        };

        if image_width * image_height * channel_count != image_object.image.data.len() as u32 {
            return Err(
                Status::new(
                Code::InvalidArgument,
                format!(
                        "Width, height, or image_type is not appropriate value\nwidth: {}\nheight: {}\nchannel count: {}.\n width * height * channel_count has to be equal to bytes buffer length.", 
                        image_width,
                        image_height,
                        channel_count
                    )
                )
            );
        }

        let user_image: RgbaImage =
            ImageBuffer::from_vec(image_width, image_height, image_object.image.data).unwrap();

        let thumbnail = Image {
            width: new_width,
            height: new_height,
            image_type: image_object.image.image_type,
            data: thumbnail(&user_image, new_width, new_height).into_vec(),
        };

        Ok(Response::new(thumbnail))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let address = "127.0.0.1:8000".parse()?;
    let service = ImageProcessingService::default();

    Server::builder()
        .add_service(ImageProcessingServer::new(service))
        .serve(address)
        .await?;

    Ok(())
}
