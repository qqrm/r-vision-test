mod text_processors;

use anyhow::Result;
use bincode::deserialize;
use common::types::{DataChunk, DataType};
use nats::Connection;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    thread,
    time::Duration,
};
use text_processors::process_text;

/// Responsible for consuming data chunks and writing them to files.
pub struct WriterConsumer {
    nc: Connection, // Nats connection for data consumption
    path: String,   // Destination path for writing files
}

impl WriterConsumer {
    /// Creates a new `WriterConsumer`.
    ///
    /// * `path`: Destination path for writing files.
    /// * `nc`: Nats connection for consuming data chunks.
    pub fn new(path: String, nc: Connection) -> Self {
        Self { nc, path }
    }

    /// Processes a single data chunk based on its data type.
    ///
    /// * `data_chunk`: The data chunk to be processed.
    ///
    /// Returns:
    /// * `Result`: Standard Result type, Ok if successful, Error otherwise.
    fn process_chunk(data_chunk: &mut DataChunk) -> Result<()> {
        match data_chunk.data_type {
            DataType::Text => process_text(&mut data_chunk.data),
            _ => Ok(()),
        }
    }

    /// Consumes data chunks from a NATS stream and writes them to a file.
    ///
    /// * `with_delay`: A boolean flag to introduce artificial delay in processing.
    ///
    /// Returns:
    /// * `Result`: Standard Result type, Ok if successful, Error otherwise.
    pub fn receive_file(&self, with_delay: bool) -> Result<()> {
        // Subscribe to the "data_chunk" NATS subject.
        let subscription = self.nc.subscribe("data_chunk")?;

        // Create a temporary file to save the incoming data chunks.
        let tmp_path_to_save = format!("{}temp_file.tmp", self.path);
        if Path::new(&tmp_path_to_save).exists() {
            fs::remove_file(&tmp_path_to_save)?;
        }
        let mut file = File::create(&tmp_path_to_save)?;
        let file_name: String;

        loop {
            let msg = subscription.next().unwrap();
            let mut data_chunk: DataChunk = deserialize(&msg.data)?;

            // Introduce artificial delay if `with_delay` is true.
            if with_delay {
                thread::sleep(Duration::from_secs(4));
            }

            // Process the data chunk.
            Self::process_chunk(&mut data_chunk)?;

            // Write the processed data chunk to the temporary file.
            file.write_all(&data_chunk.data)?;
            msg.respond("ok")?;

            // If this is the last chunk, break out of the loop.
            if data_chunk.last_chunk {
                file_name = data_chunk.file_name.clone();
                break;
            }
        }

        // Move the temporary file to the final location.
        let final_path = format!("{}{}", self.path, file_name);
        if Path::new(&final_path).exists() {
            fs::remove_file(&final_path)?;
        }
        fs::rename(tmp_path_to_save, final_path)?;

        Ok(())
    }
}
