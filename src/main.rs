use std::{fs::File, io::Write};

pub mod perf;

fn main() -> std::io::Result<()> {
    let mut file = File::create("perf.data")?;
    file.write_all("hi there".as_bytes()).unwrap();
    println!("Hello, world!");

    Ok(())
}
