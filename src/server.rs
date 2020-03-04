use image::{ImageBuffer, RgbaImage};
use image_processing::pixel_ops::invert_mut;
// use process_image::ProcessImage;
use tonic::{transport::Server, Request, Response, Status};
#[derive(Debug, Default)]
struct ImageProcessingService;

// pub mod imageprocessing {
//     tonic::include_proto!("image_processing");
// }

// use imageprocessing::image_processing_server::{ImageProcessing, ImageProcessingServer};
// use imageprocessing::{Image, ImageType};

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
        // println!("Got a request: {:?}", request);
        let image_object = request.into_inner();
        // dbg!(&image_object);
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

            operations
        };

        for operation in operations.iter() {
            operation.process_image(&mut user_image);
        }

        let processed_image = ResultImage {
            width: image_object.width,
            height: image_object.height,
            image_type: ImageType::Rgba as i32,
            // data: vec![0_u8; image_object.data.len()],
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
