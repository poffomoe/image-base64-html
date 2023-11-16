use base64::{engine::general_purpose, Engine};
use std::{fs::File, io::{Read, Write}};

fn main() {
    let mut img = File::options()
                            .read(true)
                            .open("image.png")
                            .expect("no image provided, please put the image.png file in the project root");

    let mut buffer = Vec::new();
    img.read_to_end(&mut buffer)
                            .unwrap();
    let b64 = general_purpose::STANDARD
                            .encode(buffer.as_slice());

    let mut html = File::options()
                            .write(true)
                            .create(true)
                            .truncate(true)
                            .open("result.html")
                            .expect("couldn't open the html file for writing");
                        
    html.write_all(format!("<img src=\"data:image/png;base64,{}\" alt=\"image\">", b64)
                            .as_bytes())
                            .expect("couldn't write, the file might be read-only");
}
