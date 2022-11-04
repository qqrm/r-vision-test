use reader::ReaderProducer;

fn main() -> anyhow::Result<()> {
    println!("Start send chunks");

    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/home/qqrm/repos/r-vision-test/files_to_send/test.txt".to_owned());

    let nc = nats::connect("0.0.0.0:4222")?;
    let rp = ReaderProducer::new(path, nc, 4096);
    rp.process_files()?;

    Ok(())
}
