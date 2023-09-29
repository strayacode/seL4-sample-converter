use std::mem;

use bytemuck::{
    Pod, Zeroable,
};

use super::{file_section::FileSection, header_flags::HeaderFlags};

const PERF_MAGIC: u64 = 0x32454c4946524550;

#[repr(C)]
#[derive(Default, Pod, Clone, Copy, Zeroable)]
pub struct Header {
    // must be PERFFILE2 in little endian format
    pub magic: u64,

    // size of the header
    pub size: u64,

    // size of an attribute in attribute section
    pub attr_size: u64,

    // this section refers to particular attributes, which can be linked to events
    // in the data section
    pub attrs: FileSection,

    // the data section contains multiple events
    pub data: FileSection,

    // TODO: figure out what this section represents
    pub event_types: FileSection,

    // flags are used to extend the perf file with extra info
    // in our case only the first 64-bit variable is used
    pub flags: HeaderFlags,
    pub flags1: [u64; 3],
}

impl Header {
    // creates a header with default fields
    pub fn new() -> Header {
        Header {
            magic: PERF_MAGIC,
            size: mem::size_of::<Header>() as u64,
            ..Default::default()
        }
    }
}