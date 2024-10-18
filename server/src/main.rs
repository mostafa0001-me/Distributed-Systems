use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};
use image::{DynamicImage, RgbaImage};
use steganography::encoder::Encoder;

fn main() {
    // Get server IP addresses from command line arguments (currently not used)
    let args: Vec<String> = env::args().collect();
    let _peer_server_ips: Vec<String> = args[1..].to_vec(); // Placeholder for future use

    // Bind the server to a port
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind to port");

    for stream in listener.incoming() {
        let stream = stream.expect("Failed to establish connection");
        handle_client(stream);
    }
}

// Function to handle incoming requests and perform steganography encryption
fn handle_client(mut stream: TcpStream) {
    let mut buffer = Vec::new();
    
    // Read image data sent by the middleware
    stream.read_to_end(&mut buffer).expect("Failed to read image");

    // Save the received image temporarily
    let temp_image_path = "/home/mostafa/Distributed-Systems-Project/server/temp.jpg";
    std::fs::write(temp_image_path, &buffer).expect("Failed to save image");

    // Load the image
    let img = image::open(temp_image_path).expect("Failed to open image");

    // Perform steganography encryption (we'll hide a simple message inside the image)
    let secret_message = "Hidden Message for Steganography";
    let encrypted_image = encrypt_image_with_steganography(img, secret_message);

    // Save the encrypted image to a temporary file
    let encrypted_image_path = "/home/mostafa/Distributed-Systems-Project/server/temp_e.jpg";
    encrypted_image.save(encrypted_image_path).expect("Failed to save encrypted image");

    // Read the encrypted image and send it back to the middleware
    let encrypted_image_data = std::fs::read(encrypted_image_path).expect("Failed to read encrypted image");
    stream.write_all(&encrypted_image_data).expect("Failed to send encrypted image");
    stream.shutdown(Shutdown::Write).expect("Failed to shut down writing side of stream");

    // Cleanup temporary files
    std::fs::remove_file(temp_image_path).expect("Failed to remove temp image");
    std::fs::remove_file(encrypted_image_path).expect("Failed to remove encrypted image");
}

// Function to perform steganography on the image by hiding the message inside it
fn encrypt_image_with_steganography(img: DynamicImage, message: &str) -> DynamicImage {
    let mut rgba_img: RgbaImage = img.to_rgba();

    // Create the encoder with the message and the image
    let encoder = Encoder::new(message.as_bytes(), DynamicImage::ImageRgba8(rgba_img.clone()));

    // Perform the encoding by using the encode_alpha method
    let encoded_img = encoder.encode_alpha();

    DynamicImage::ImageRgba8(encoded_img)
}

