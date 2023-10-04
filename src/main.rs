use std::{fs::File, mem, io::Write, slice, str};

use sample_converter::{perf::{header::Header, attributes::{FileAttribute, EventAttribute}, PerfFile}, as_raw_bytes};

fn main() -> std::io::Result<()> {
    let mut perf_file = PerfFile::new("perf.data")?;
    
    let mut header = Header::new();
    
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

    perf_file.write(&header)?;

    let mut file_attr = FileAttribute::default();
    file_attr.attr.size = mem::size_of::<EventAttribute>() as u32;

    // arbitrary sample frequency value
    file_attr.attr.sample_period_or_freq = 100;

    // include all sample information
    // file_attr.attr.sample_type = 

    Ok(())
}
