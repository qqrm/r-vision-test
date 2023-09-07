use anyhow::Result;
use std::env;
use writer::WriterConsumer;

/// Entry point of the application
fn main() -> Result<()> {
    // Log the start of chunk receiving
    println!("Start receiving chunks");

    // Retrieve the path from command-line arguments, or use a default path if not provided
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "/home/qqrm/repos/r-vision-test/received_files/".into());

    // Initialize a Nats connection
    let nc = nats::connect("0.0.0.0:4222")?;

    // Create a WriterConsumer with a folder path and a Nats connection
    let wc = WriterConsumer::new(path, nc);

    // Receive file chunks and reconstruct the file
    wc.receive_file(false)?;

    Ok(())
}
