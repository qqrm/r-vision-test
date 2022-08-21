use bincode::serialize;
use common::types::{parse_extention, DataChunk};
use nats::Connection;
use std::{fs, io::Read, path::Path, time::Duration};

pub struct ReaderProducer {
    nc: Connection,
    path: String,
    chunk_size: usize,
}

impl ReaderProducer {
    pub fn new(folder_path: String, nc: Connection, chunk_size: usize) -> Self {
        Self {
            nc,
            path: folder_path,
            chunk_size,
        }
    }

    pub fn process_file(&self, file_name: String) -> anyhow::Result<()> {
        println!("process file: {}", &file_name);

        let filepath = self.path.clone() + &file_name;

        let mut file = std::fs::File::open(&filepath)?;

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
                file_name: file_name.to_owned(),
                data_type: parse_extention(ext),
                last_chunk: self.chunk_size > n,
                data: chunk,
            };

            let bin_data = serialize(&chunk)?;
            // let _ = self.nc.request("data_chunk", bin_data)?;

            let _ = self
                .nc
                .request_timeout("data_chunk", bin_data, Duration::from_secs(2))?;

            if n < self.chunk_size {
                break;
            }
        }

        Ok(())
    }

    pub fn process_files(&self) -> anyhow::Result<()> {
        let paths = fs::read_dir(&self.path)?;

        for path in paths {
            let filename = path?
                .file_name()
                .to_str()
                .expect("filename getting error")
                .to_owned();

            self.process_file(filename)?
        }

        Ok(())
    }
}
