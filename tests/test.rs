#[cfg(test)]
mod tests {
    use core::time;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        thread,
    };

    use reader::ReaderProducer;
    use std::env::current_dir;
    use writer::WriterConsumer;

    #[test]
    fn send_ru_min_txt() {
        let mut threads = vec![];

        threads.push(thread::spawn(move || {
            let nc = nats::connect("0.0.0.0:4222").unwrap();

            // different behavior with dubugging, may replace to systdm env var
            let write_path = current_dir().unwrap().to_str().unwrap().to_owned()
                + &"/recieved_files/".to_owned();

            // let write_path = "/home/qqrm/repos/r-vision-test/tests/recieved_files/".to_owned();

            let wc = WriterConsumer::new(write_path, nc);
            wc.recieve_file(false).unwrap();
        }));

        thread::sleep(time::Duration::from_secs(2));

        let folder_path =
            current_dir().unwrap().to_str().unwrap().to_owned() + &"/files_to_send/".to_owned();

        // let folder_path = "/home/qqrm/repos/r-vision-test/tests/files_to_send/".to_owned();

        let nc = nats::connect("0.0.0.0:4222").unwrap();

        let rp = ReaderProducer::new(folder_path, nc, 4096);

        let filename = "test_en_min.txt".to_owned();

        rp.process_file(filename).unwrap();

        for thread in threads.into_iter() {
            thread.join().unwrap();
        }

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
    #[should_panic]
    fn send_ru_min_txt_with_delay() {
        let mut threads = vec![];

        threads.push(thread::spawn(move || {
            let nc = nats::connect("0.0.0.0:4222").unwrap();

            let write_path = current_dir().unwrap().to_str().unwrap().to_owned()
                + &"/recieved_files/".to_owned();

            let wc = WriterConsumer::new(write_path, nc);
            wc.recieve_file(true).unwrap();
        }));

        thread::sleep(time::Duration::from_secs(2));

        let folder_path =
            current_dir().unwrap().to_str().unwrap().to_owned() + &"/files_to_send/".to_owned();

        let nc = nats::connect("0.0.0.0:4222").unwrap();

        let rp = ReaderProducer::new(folder_path, nc, 4096);

        let filename = "test_en_min.txt".to_owned();

        rp.process_file(filename).unwrap();

        for thread in threads.into_iter() {
            thread.join().unwrap();
        }
    }
}
