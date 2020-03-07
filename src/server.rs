use image::{ImageBuffer, RgbaImage};
use std::convert::TryFrom;
use tonic::{transport::Server, Code, Request, Response, Status};
#[derive(Debug, Default)]
struct ImageProcessingService;

use process_image;
use process_image::imageprocessing::image_processing_server::{
    ImageProcessing, ImageProcessingServer,
};
use process_image::imageprocessing::{Image, ImageType, ResultImage};
use process_image::ProcessImage;

#[tonic::async_trait]
impl ImageProcessing for ImageProcessingService {
    async fn process_image(
        &self,
        request: Request<Image>,
    ) -> Result<Response<ResultImage>, Status> {
        let image_object = request.into_inner();

        if image_object.data.is_empty() {
            return Err(Status::new(
                Code::InvalidArgument,
                "Data variable is empty.",
            ));
        }

        let channel_count: u32 = match ImageType::try_from(image_object.image_type)? {
            ImageType::Rgba => 4,
            ImageType::Rgb => 3,
            ImageType::Grayalpha => 2,
            ImageType::Gray => 1,
        };

        if image_object.width * image_object.height * channel_count
            != image_object.data.len() as u32
        {
            return Err(
                Status::new(
                Code::InvalidArgument,
                format!(
                        "Width, height, or image_type is not appropriate value\nwidth: {}\nheight: {}\nchannel count: {}.\n width * height * channel_count has to be equal to bytes buffer length.", 
                        image_object.width,
                        image_object.height,
                        channel_count
                    )
                )
            );
        }

        let mut user_image: RgbaImage =
            ImageBuffer::from_vec(image_object.width, image_object.height, image_object.data)
                .unwrap();

        let operations: Vec<Box<dyn ProcessImage>> = {
            let mut operations = Vec::new();

            if let Some(operation) = image_object.invert {
                operations.push(Box::new(operation) as Box<dyn ProcessImage>);
            }

            if let Some(operation) = image_object.edge_detect {
                operations.push(Box::new(operation) as Box<dyn ProcessImage>);
            }

            if let Some(operation) = image_object.box_blur {
                operations.push(Box::new(operation) as Box<dyn ProcessImage>);
            }

            operations
        };

        for operation in operations.iter() {
            operation.process_image(&mut user_image)?;
        }

        let processed_image = ResultImage {
            width: image_object.width,
            height: image_object.height,
            image_type: ImageType::Rgba as i32,
            data: user_image.into_vec(),
        };

        Ok(Response::new(processed_image))
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
