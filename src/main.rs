use std::{fs::File, io::Write};

use crate::perf::header::Header;

pub mod perf;

fn main() -> std::io::Result<()> {
    let mut file = File::create("perf.data")?;

    let mut header = Header::new();
    let bytes: &[u8] = bytemuck::bytes_of(&header);

    file.write_all("hi there".as_bytes()).unwrap();
    println!("Hello, world!");

    Ok(())
}
