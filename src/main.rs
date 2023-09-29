use std::{fs::File, mem, io::Write, slice};

use crate::perf::header::Header;

pub mod perf;

fn as_raw_bytes<T>(data: &T) -> &[u8] {
    let ptr = data as *const T;
    let size = mem::size_of::<T>();
    let bytes = unsafe { slice::from_raw_parts(ptr, size) };
    bytes
}

fn main() -> std::io::Result<()> {
    let mut file = File::create("perf.data")?;

    let mut header = Header::new();
    let bytes: &[u8] = bytemuck::bytes_of(&header);

    file.write_all("hi there".as_bytes()).unwrap();
    println!("Hello, world!");

    Ok(())
}
