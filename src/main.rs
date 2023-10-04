use std::{fs::File, mem, io::Write, slice, str};

use sample_converter::{perf::header::Header, as_raw_bytes};

fn main() -> std::io::Result<()> {
    let mut file = File::create("perf.data")?;

    let mut header = Header::new();
    let bytes: &[u8] = as_raw_bytes(&header);

    let magic_bytes = header.magic.to_le_bytes();
    let magic_string = str::from_utf8(&magic_bytes).unwrap();
    assert_eq!(magic_string, "PERFILE2");

    println!("magic: {}", magic_string);
    println!("header size: {}", header.size);
    println!("size of a single attribute: {}", header.attr_size);
    println!("attribute section: {}", header.attrs);
    println!("data section: {}", header.data);
    println!("event types section: {}", header.event_types);
    println!("flags: {:#016x}", header.flags);

    Ok(())
}
