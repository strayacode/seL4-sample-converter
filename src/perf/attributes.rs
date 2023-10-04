// this file outlines structures related to the attributes section of the file

use super::file_section::FileSection;

use bitflags::bitflags;

#[repr(C, packed)]
struct FileAttribute {
    attr: EventAttribute,

    // points to a file section which contains an array of u64 ids
    // i assume these ids are for the events which are linked to this attribute
    ids: FileSection,
}

#[repr(C, packed)]
struct EventAttribute {
    // major type: hardware/software/tracepoint/etc
    event_type: EventType,

    // size of the attribute
    size: u32,

    // TODO: figure out what this is for
    config: u64,

    sample_period_or_freq: u64,
    sample_type: SampleType,
}

#[repr(u32)]
enum EventType {
    Hardware = 0,
    Software = 1,
    Tracepoint = 2,
    HwCache = 3,
    Raw = 4,
    Breakpoint = 5,
}

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct SampleType: u64 {
        const IP = 1 << 0;
        const TID = 1 << 1;
        const TIME = 1 << 2;
        const ADDR = 1 << 3;
        const READ = 1 << 4;
        const CALLCHAIN = 1 << 5;
        const ID = 1 << 6;
        const CPU = 1 << 7;
        const PERIOD = 1 << 8;
        const STREAM_ID = 1 << 9;
        const RAW = 1 << 10;
        const BRANCH_STACK = 1 << 11;
        const REGS_USER = 1 << 12;
        const STACK_USER = 1 << 13;
        const WEIGHT = 1 << 14;
        const DATA_SRC = 1 << 15;
        const IDENTIFIER = 1 << 16;
        const TRANSACTION = 1 << 17;
        const REGS_INTR = 1 << 18;
        const PHY_ADDR = 1 << 19;
        const AUX = 1 << 20;
        const CGROUP = 1 << 21;
        const DATA_PAGE_SIZE = 1 << 22;
        const CODE_PAGE_SIZE = 1 << 23;
        const WEIGHT_STRUCT = 1 << 24;
    }
}