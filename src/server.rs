use tonic::{transport::Server, Request, Response, Status};
#[derive(Debug, Default)]
struct ImageProcessingService;

pub mod imageprocessing {
    tonic::include_proto!("image_processing");
}

use imageprocessing::image_processing_server::{ImageProcessing, ImageProcessingServer};
use imageprocessing::{Image, ImageType};

#[tonic::async_trait]
impl ImageProcessing for ImageProcessingService {

    async fn invert(&self, request: Request<Image>) -> Result<Response<Image>, Status> {
        // println!("Got a request: {:?}", request);
        let image = request.into_inner();
        dbg!(&image);

        let inverted_image = Image {
            width: image.width,
            height: image.height, 
            image_type: ImageType::Rgba as i32,
            data: vec![0_u8; image.data.len()],
        };

        Ok(Response::new(inverted_image))
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
