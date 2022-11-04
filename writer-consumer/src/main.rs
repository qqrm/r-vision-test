use anyhow::Ok;
use writer::WriterConsumer;

fn main() -> anyhow::Result<()> {
    println!("Start recieve chunks");

    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/home/qqrm/repos/r-vision-test/recieved_files/".to_owned());

    let nc = nats::connect("0.0.0.0:4222")?;

    let wc = WriterConsumer::new(path, nc);
    wc.recieve_file(false)?;

    Ok(())
}
