use image::ColorType;

// pub mod imageprocessing {
//     tonic::include_proto!("image_processing");
// }

// use imageprocessing::image_processing_client::ImageProcessingClient;
// use imageprocessing::{EdgeDetect, Image, ImageType, Invert};

use process_image::imageprocessing::image_processing_client::ImageProcessingClient;
use process_image::imageprocessing::{EdgeDetect, Image, ImageType, Invert, ResultImage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ImageProcessingClient::connect("http://127.0.0.1:8000").await?;
    let image_data = image::open("./images/hot-air-balloon.jpg")?.to_rgba();

    let request = tonic::Request::new(Image {
        width: image_data.width(),
        height: image_data.height(),
        image_type: ImageType::Rgba as i32,
        // data: vec![25, 99, 250, 255],
        data: image_data.into_vec(),
        // invert: Some(Invert {}),
        invert: None,
        edge_detect: Some(EdgeDetect { threshold: 120 }),
    });

    // let response = client.invert(request).await?;
    let response = client.process_image(request).await?;

    // dbg!(&response);

    let processed_image = response.into_inner();
    // dbg!(inverted_image);

    image::save_buffer(
        "./images/invert.png",
        &processed_image.data,
        processed_image.width,
        processed_image.height,
        ColorType::RGBA(8),
    )?;

    Ok(())
}
