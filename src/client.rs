use image::ColorType;

use process_image::imageprocessing::image_processing_client::ImageProcessingClient;
use process_image::imageprocessing::{
    BoxBlur, EdgeDetect, Image, ImageParameters, ImageType, Invert, ThumbnailImage,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ImageProcessingClient::connect("http://127.0.0.1:8000").await?;
    let image_data = image::open("./images/hot-air-balloon.jpg")?.to_rgba();
    let thumbnail_data = image_data.clone();

    // let request = tonic::Request::new(ImageParameters {
    //     image: Image {
    //         width: image_data.width(),
    //         height: image_data.height(),
    //         image_type: ImageType::Rgba as i32,
    //         data: image_data.into_vec(),
    //     },
    //     invert: None,
    //     edge_detect: Some(EdgeDetect { threshold: 100 }),
    //     box_blur: Some(BoxBlur { kernel_width: 3 }),
    // });

    // let response = client.process_image(request).await?;

    // let processed_image = response.into_inner();

    // image::save_buffer(
    //     "./images/processed_image.png",
    //     &processed_image.data,
    //     processed_image.width,
    //     processed_image.height,
    //     ColorType::RGBA(8),
    // )?;
    let request = tonic::Request::new(ThumbnailImage {
        new_width: 500,
        new_height: 400,
        image: Image {
            width: thumbnail_data.width(),
            height: thumbnail_data.height(),
            image_type: ImageType::Rgba as i32,
            data: thumbnail_data.into_vec(),
        },
    });

    let response = client.create_thumbnail(request).await?;

    let thumbnail = response.into_inner();

    image::save_buffer(
        "./images/processed_image.png",
        &thumbnail.data,
        thumbnail.width,
        thumbnail.height,
        ColorType::RGBA(8),
    )?;
    Ok(())
}
