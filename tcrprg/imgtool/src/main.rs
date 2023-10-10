use std::env;
use std::path::Path;
use log::{info};

fn main() {
    env_logger::init();
    let image_path = env::args().skip(1).next().unwrap();
    let path = Path::new(&image_path);
    info!("Rotating image: {:?}", path);
    let img = image::open(path).unwrap();
    let rotated = img.rotate90();
    let filename = path.file_name().unwrap();
    let new_path = format!(r"{}/{}", "./rotated_images", filename.to_str().unwrap());
    rotated.save(new_path).unwrap();
}
