#[cfg(test)]
mod tests {
    use std::env::current_dir;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        thread,
        time::Duration,
    };

    use reader::ReaderProducer;
    use writer::WriterConsumer;

    #[test]
    fn send_ru_min_txt() {
        // Spawn a new thread for receiving the file.
        thread::spawn(|| {
            let nc = nats::connect("0.0.0.0:4222").expect("Failed to connect to NATS");
            let write_path = format!(
                "{}/recieved_files/",
                current_dir()
                    .expect("Failed to get current directory")
                    .to_str()
                    .expect("Failed to convert path to string")
            );
            let wc = WriterConsumer::new(write_path, nc);
            wc.receive_file(false).expect("Failed to receive file");
        });

        // Sleep to make sure the above thread is ready.
        thread::sleep(Duration::from_secs(1));

        // Perform the send operation.
        let folder_path = format!(
            "{}/files_to_send/",
            current_dir()
                .expect("Failed to get current directory")
                .to_str()
                .expect("Failed to convert path to string")
        );
        let nc = nats::connect("0.0.0.0:4222").expect("Failed to connect to NATS");
        let rp = ReaderProducer::new(folder_path, nc, 4096);
        rp.process_file("test_en_min.txt".to_owned())
            .expect("Failed to process file");

        let etalon_path = current_dir().unwrap().to_str().unwrap().to_owned()
            + &"/etalon/test_en_min.txt".to_owned();

        let res_path = current_dir().unwrap().to_str().unwrap().to_owned()
            + &"/recieved_files/test_en_min.txt".to_owned();

        let etalon = File::open(etalon_path).unwrap();
        let res = File::open(res_path).unwrap();

        let etalon_rerader = BufReader::new(etalon);
        let res_reader = BufReader::new(res);

        for (el_line, res_line) in etalon_rerader.lines().zip(res_reader.lines()) {
            assert_eq!(el_line.unwrap(), res_line.unwrap());
        }
    }

    #[test]
    #[should_panic] // various panics
    fn send_ru_min_txt_with_delay() {
        // Similar to the first test, but with a delay in the receiver.

        thread::spawn(|| {
            let nc = nats::connect("0.0.0.0:4222").expect("Failed to connect to NATS");
            let write_path = format!(
                "{}/recieved_files/",
                current_dir()
                    .expect("Failed to get current directory")
                    .to_str()
                    .expect("Failed to convert path to string")
            );
            let wc = WriterConsumer::new(write_path, nc);
            wc.receive_file(true).expect("Failed to receive file");
        });

        // Sleep to give the receiver a head start.
        thread::sleep(Duration::from_secs(2));

        // Perform the send operation here, expecting a panic.
        let folder_path = format!(
            "{}/files_to_send/",
            current_dir()
                .expect("Failed to get current directory")
                .to_str()
                .expect("Failed to convert path to string")
        );
        let nc = nats::connect("0.0.0.0:4222").expect("Failed to connect to NATS");
        let rp = ReaderProducer::new(folder_path, nc, 4096);

        // This is expected to panic, hence the #[should_panic] attribute.
        rp.process_file("test_en_min.txt".to_owned())
            .expect("Failed to process file");
    }
}
