use super::{file_section::FileSection, header_flags::HeaderFlags};

const PERF_MAGIC_NUMBER: u64 = 0x32454c4946524550;

pub struct Header {
    // must be PERFFILE2 in little endian format
    magic: u64,

    // size of the header
    size: u64,

    // size of an attribute in attribute section
    attr_size: u64,

    // this section refers to particular attributes, which can be linked to events
    // in the data section
    attrs: FileSection,

    // the data section contains multiple events
    data: FileSection,

    // TODO: figure out what this section represents
    event_types: FileSection,

    // flags are used to extend the perf file with extra info
    // in our case only the first 64-bit variable is used
    flags: HeaderFlags,
    flags1: [u64; 3],
}