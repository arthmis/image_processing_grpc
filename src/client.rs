pub mod imageprocessing {
    tonic::include_proto!("image_processing");
}

use imageprocessing::image_processing_client::{ImageProcessingClient};
use imageprocessing::{Image, ImageType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ImageProcessingClient::connect("http://127.0.0.1:8000").await?;
    let request = tonic::Request::new(Image {
        width: 1,
        height: 1,
        image_type: ImageType::Rgba as i32,
        data: vec![0; 4],
    });

    let response = client.invert(request).await?;
    dbg!(response);
    Ok(())
}