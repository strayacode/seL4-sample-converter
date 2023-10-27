// this file outlines structures related to the data section of the file

use crate::sample::Sel4Sample;

use std::{mem, io::Write};

const CPUMODE_UNKNOWN: u16 = 0 << 0;
const CPUMODE_KERNEL: u16 = 1 << 0;
const CPUMODE_USER: u16 = 2 << 0;
const CPUMODE_HYPERVISOR: u16 = 3 << 0;
const CPUMODE_GUEST_KERNEL: u16 = 4 << 0;
const CPUMODE_GUEST_USER: u16 = 5 << 0;
const PROC_MAP_PARSE_TIMEOUT: u16 = 1 << 12;
const MMAP_DATA: u16 = 1 << 13;
const COMM_EXEC: u16 = 1 << 13;
const FORK_EXEC: u16 = 1 << 13;
const SWITCH_OUT: u16 = 1 << 13;
const EXACT_IP: u16 = 1 << 14;
const SWITCH_OUT_PREEMPT: u16 = 1 << 14;
const MMAP_BUILD_ID: u16 = 1 << 14;
const EXT_RESERVED: u16 = 1 << 15;

const PATH_MAX: usize = 4096;
const COMM_MAX: usize = 32;

fn align_up(address: usize, size: usize) -> usize {
    let mask = size - 1;
    (address + mask) & !mask
}

#[repr(C)]
#[derive(Debug)]
pub struct EventHeader {
    event_type: EventType,

    // indicates some miscellaneous information
    misc: u16,

    // the size of the record including the header
    pub size: u16,
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
    pub fn new(sample: Sel4Sample) -> Self {
        let header = EventHeader {
            event_type: EventType::Sample,
            misc: CPUMODE_USER,
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

#[repr(C)]
#[derive(Debug)]
pub struct CommEvent {
    header: EventHeader,
    pid: u32,
    tid: u32,
    comm: [u8; COMM_MAX],
}

impl CommEvent {
    pub fn new(pid: u32, application: &str) -> CommEvent {
        let header = EventHeader {
            event_type: EventType::Comm,
            misc: 0x4002,
            size: mem::size_of::<CommEvent>() as u16,
        };

        let mut comm = [0; COMM_MAX];
        fill_from_str(&mut comm, application);
        comm[COMM_MAX - 1] = 0;

        CommEvent {
            header,
            pid,
            tid: pid,
            comm,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct MmapEvent {
    pub header: EventHeader,
    pid: u32,
    tid: u32,
    start: u64,
    len: u64,
    pgoff: u64,
    filename: [u8; PATH_MAX],
}

impl MmapEvent {
    pub fn new(pid: u32, application: &str) -> MmapEvent {
        let application_size = align_up(application.len() + 1, 8);
        let header = EventHeader {
            event_type: EventType::Mmap,
            misc: CPUMODE_USER,
            size: ((mem::size_of::<MmapEvent>() - PATH_MAX + application_size) + 0x10) as u16,
        };

        let mut filename: [u8; PATH_MAX] = [0; PATH_MAX];
        fill_from_str(&mut filename, application);
        filename[PATH_MAX - 1] = 0;

        MmapEvent {
            header,
            pid,
            tid: pid,
            start: 0,
            len: 4096,
            pgoff: 4096,
            filename,
        }
    }
}

fn fill_from_str(mut bytes: &mut [u8], s: &str) {
    bytes.write(s.as_bytes()).unwrap();
}