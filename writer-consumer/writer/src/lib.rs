mod text_procesors;

use anyhow::Ok;
use bincode::deserialize;
use common::types::{DataChunk, DataType};
use core::time;
use nats::Connection;
use std::{fs, io::Write, path::Path, thread};
use text_procesors::process_text;

pub struct WriterConsumer {
    nc: Connection,
    path: String,
}

impl WriterConsumer {
    pub fn new(path: String, nc: Connection) -> Self {
        Self { nc, path }
    }

    pub fn process_chunk(data_chunk: &mut DataChunk) -> anyhow::Result<()> {
        match data_chunk.data_type {
            DataType::Text => process_text(&mut data_chunk.data),
            _ => Ok(()),
        }
    }

    pub fn recieve_file(&self, with_delay: bool) -> anyhow::Result<()> {
        let s = self.nc.subscribe("data_chunk").unwrap();

        let tmp_path_to_save = self.path.to_owned() + "temp_file.tmp";

        if Path::new(&tmp_path_to_save).exists() {
            std::fs::remove_file(&tmp_path_to_save)?;
        }

        let file_name: String;
        {
            let mut file = fs::File::create(&tmp_path_to_save)?;

            loop {
                let chunk = s.next().unwrap();

                let mut data_chunk: DataChunk = deserialize(&chunk.data)?;

                if with_delay {
                    thread::sleep(time::Duration::from_secs(4));
                }

                WriterConsumer::process_chunk(&mut data_chunk)?;

                file.write_all(&data_chunk.data)?;

                chunk.respond("ok")?;

                if data_chunk.last_chunk {
                    file_name = data_chunk.file_name.to_owned();
                    break;
                }
            }
        }

        let path_with_original_filename = self.path.to_owned() + &file_name;

        if Path::new(&path_with_original_filename).exists() {
            fs::remove_file(&path_with_original_filename)?;
        }

        fs::rename(tmp_path_to_save, path_with_original_filename)?;

        Ok(())
    }
}
