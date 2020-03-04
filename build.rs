fn main() {
    // tonic_build::compile_protos("proto/image_processing/image_processing.proto")
    //     .unwrap_or_else(|err| panic!("Failed to compile protos {:?}", err));
    tonic_build::configure()
        .out_dir("./src")
        .compile(&["proto/imageprocessing/imageprocessing.proto"], &["proto/imageprocessing"])
        .unwrap_or_else(|err| panic!("Failed to compile protos {:?}", err));
}