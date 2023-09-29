pub const CALL_STACK_DEPTH: usize = 10;

pub struct Sel4Sample {
    // instruction pointer
    ip: u64,

    // process id (in our case id of the process domain)
    pid: u32,

    // timestamp of when the sample occured
    timestamp: u64,

    // cpu affinity - which cpu is being used
    cpu: u32,

    // number of events per sample
    period: u64,

    // call stack - provides a trace of addresses for functions called
    ips: [u64; CALL_STACK_DEPTH],
}