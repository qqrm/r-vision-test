use anyhow::Result;
use reader::ReaderProducer;
use std::env;

fn main() -> Result<()> {
    // Log the start of chunk sending
    println!("Start sending chunks");

    // Retrieve the path from command-line arguments, default to a predefined path if not provided
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "/home/qqrm/repos/r-vision-test/files_to_send/test.txt".into());

    // Initialize a Nats connection
    let nc = nats::connect("0.0.0.0:4222")?;

    // Create a ReaderProducer with a folder path, Nats connection, and chunk size
    let rp = ReaderProducer::new(path, nc, 4096);

    // Process files and send them as chunks
    rp.process_files()?;

    Ok(())
}
