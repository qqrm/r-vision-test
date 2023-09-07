use anyhow::{anyhow, Result};
use bincode::serialize;
use common::types::{parse_extension, DataChunk};
use nats::Connection;
use std::{fs, io::Read, path::Path, time::Duration};

/// A producer that reads files and sends them as chunks over a NATS connection.
pub struct ReaderProducer {
    nc: Connection,    // NATS Connection
    path: String,      // Folder containing the files to be read
    chunk_size: usize, // Size of each chunk to be sent
}

impl ReaderProducer {
    /// Create a new ReaderProducer.
    ///
    /// # Arguments
    ///
    /// * `folder_path` - The path to the folder containing files.
    /// * `nc` - The NATS Connection for sending the chunks.
    /// * `chunk_size` - The size of each chunk to be sent.
    pub fn new(folder_path: String, nc: Connection, chunk_size: usize) -> Self {
        Self {
            nc,
            path: folder_path,
            chunk_size,
        }
    }

    /// Process a given file, breaking it into chunks and sending each over the NATS connection.
    ///
    /// # Arguments
    ///
    /// * `file_name` - The name of the file to process.
    pub fn process_file(&self, file_name: String) -> Result<()> {
        let filepath = format!("{}{}", self.path, file_name);
        let mut file = fs::File::open(&filepath)?;

        loop {
            let mut chunk = Vec::with_capacity(self.chunk_size);
            let n = file
                .by_ref()
                .take(self.chunk_size as u64)
                .read_to_end(&mut chunk)?;

            if n == 0 {
                break;
            }

            let ext = Path::new(&filepath).extension();
            let chunk = DataChunk {
                file_name: file_name.clone(),
                data_type: parse_extension(ext),
                last_chunk: n < self.chunk_size,
                data: chunk,
            };

            let bin_data = serialize(&chunk)?;
            self.nc
                .request_timeout("data_chunk", bin_data, Duration::from_secs(2))?;

            if n < self.chunk_size {
                break;
            }
        }

        Ok(())
    }

    /// Process all files in the folder specified in `self.path`.
    pub fn process_files(&self) -> Result<()> {
        let paths = fs::read_dir(&self.path)?;
        for path in paths {
            let filename = path?
                .file_name()
                .to_str()
                .ok_or(anyhow!("Filename parsing error"))?
                .to_owned();

            self.process_file(filename)?;
        }
        Ok(())
    }
}
