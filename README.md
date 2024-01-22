# Image Upload and Sharing Service

This project is a simple image upload and sharing service implemented in Rust using the Tide web framework. It includes a Bash script for taking screenshots, uploading them to the server, and copying the image URL to the clipboard.

## Prerequisites
- [xclip](https://github.com/astrand/xclip): Clipboard utility for X
- [flameshot](https://github.com/flameshot-org/flameshot): Screenshot software
- [Rust](https://www.rust-lang.org/) and [Cargo](https://doc.rust-lang.org/cargo/) for building and running the Rust server

## Setup and Usage

### Bash Script
The Bash script (`upload.sh`) is designed to be bound to a key combination in the i3 window manager. It captures a screenshot using Flameshot, saves it to a temporary file, uploads it to the server, and copies the image URL to the clipboard.

### Rust Server
1. Install Rust and Cargo: Follow the instructions on [Rust's official website](https://www.rust-lang.org/).
2. Clone this repository.
4. Run the server using `cargo run`.

## Rust Server API

### Upload Image
- **Endpoint**: `/new/:file` (HTTP PUT)
- **Description**: Uploads an image file to the server.
- **Response**: Returns the URL of the uploaded image.

### View Uploaded Images
- **Endpoint**: `/` (serves images in the "images/" directory)
- **Description**: Access the uploaded images directly through the server.

## Additional Notes
- No .env has been implemented, URL's are hard-coded
- The server logs are output to the console and include debugging information.

Feel free to customize the server and script according to your needs.
