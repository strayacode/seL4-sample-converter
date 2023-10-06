// this file outlines structures related to the data section of the file

use bitflags::bitflags;

use crate::sample::Sel4Sample;

use std::mem;

#[repr(C)]
#[derive(Debug)]
pub struct EventHeader {
    event_type: EventType,

    // indicates some miscellaneous information
    misc: Misc,

    // the size of the record including the header
    size: u16,
}

#[repr(u32)]
#[derive(Debug)]
enum EventType {
    Mmap = 1,
    Lost = 2,
    Comm = 3,
    Exit = 4,
    Throttle = 5,
    Unthrottle = 6,
    Fork = 7,
    Read = 8,
    Sample = 9,
    Mmap2 = 10,
    Aux = 11,
    ItraceStart = 12,
    LostSamples = 13,
    Switch = 14,
    SwitchCpuWide = 15,
    Namespaces = 16,
    Ksymbol = 17,
    BpfEvent = 18,
    Cgroup = 19,
    TextPoke = 20,
    AuxOutputHwId = 21,
}

bitflags! {
    #[repr(C)]
    #[derive(Default, Debug)]
    pub struct Misc: u16 {
        const CPUMODE_UNKNOWN = 0 << 0;
        const CPUMODE_KERNEL = 1 << 0;
        const CPUMODE_USER = 2 << 0;
        const CPUMODE_HYPERVISOR = 3 << 0;
        const CPUMODE_GUEST_KERNEL = 4 << 0;
        const CPUMODE_GUEST_USER = 5 << 0;
        const PROC_MAP_PARSE_TIMEOUT = 1 << 12;
        const MMAP_DATA = 1 << 13;
        const COMM_EXEC = 1 << 13;
        const FORK_EXEC = 1 << 13;
        const SWITCH_OUT = 1 << 13;
        const EXACT_IP = 1 << 14;
        const SWITCH_OUT_PREEMPT = 1 << 14;
        const MMAP_BUILD_ID = 1 << 14;
        const EXT_RESERVED = 1 << 15;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SampleEvent {
    header: EventHeader,

    // the perf structure uses constants such as
    // PERF_SAMPLE_IDENTIFIER, etc to optionally allow
    // fields in the sample
    // this means we can just support a basic amount of sample info
    ip: u64,
    pid: u32,
    tid: u32,
    time: u64,
    cpu: u32,
    period: u64,
}

impl SampleEvent {
    pub fn from(sample: Sel4Sample) -> Self {
        let header = EventHeader {
            event_type: EventType::Sample,
            misc: Misc::CPUMODE_USER,
            size: mem::size_of::<SampleEvent>() as u16,
        };

        SampleEvent {
            header,
            ip: sample.ip,
            pid: sample.pid,
            tid: sample.pid,
            time: sample.timestamp,
            cpu: sample.cpu,
            period: sample.period,
        }
    }
}