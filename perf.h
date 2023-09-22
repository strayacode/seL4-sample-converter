#ifndef PERF_H
#define PERF_H

#include "types.h"
#include "sample.h"

#define PATH_MAX 4096

// this file defines the data structures used by perf files

struct perf_file_section {
    u64 offset; // file offset of the section
    u64 size; // size of the section. if the size is greater than the struct in the section, then there are multiple elements
};

typedef struct perf_file_section perf_file_section_t;

struct perf_file_header {
    u64 magic; // magic number has to be "PERFFILE"
    u64 size; // size of the header
    u64 attr_size; // size of one attribute section. if it does not match, the entries may need to be swapped
    perf_file_section_t attrs; // list of perf_file_attr entries
    perf_file_section_t data;
    perf_file_section_t event_types; // list of perf_trace_event_type entries. apparently ignored
};

typedef struct perf_file_header perf_file_header_t;

struct perf_trace_event_type {
    u64 event_id; // this entry belongs to the perf_event_attr where .config has the same value as this id
    char name[64]; // name of the event source
};

typedef struct perf_trace_event_type perf_trace_event_type_t;

struct perf_event_attr {
    u32 type; // major type (hardware/software/tracepoint/etc)
    u32 size; // size of this structure
    u64 config; // link to .event_id of perf_event_trace_type

    union {
        u64 sample_period; // number of events when a sample is generated if .freq is not set
        u64 sample_freq; // frequency for sampling if .freq is set
    };

    u64 sample_type; // gives information about what is stored in the sampling record
    u64 read_format;
    u64 disabled : 1, // off by default
        inherit : 1, // children inherit it
        pinned : 1, // must always be on pmu
        exclusive : 1, // only group on pmu
        exclude_user : 1, // don't count user
        exclude_kernel : 1, // ditto kernel
        exclude_hv : 1, // ditto hypervisor
        exclude_idle : 1, // don't count when idle
        mmap : 1, // mmap records are included in this file
        comm : 1, // comm records are included in this file
        freq : 1, // if set sample_freq is valid otherwise sample_period
        inherit_stat : 1, // per task counts
        enable_on_exec : 1, // next exec enable
        task : 1, // trace fork/exit
        watermark : 1, // wakeup_watermark

        /*
        0 - SAMPLE_IP can have arbitrary skid
        1 - SAMPLE_IP must have constant skid
        2 - SAMPLE_IP can have arbitrary skid
        3 - SAMPLE_IP must have 0 skid
        */
        precise_ip : 2,

        mmap_data : 1, // non-exec mmap data
        sample_id_all : 1, // if set, the records have additional information. the bit is assumed to be set
        reserved : 45;

    union {
        u32 wakeup_events; // wakeup every n events
        u32 wakeup_watermark; // bytes before wakeup
    };

    u32 bp_type;

    union {
        u64 bp_addr;
        u64 config1; // extension of config
    };

    union {
        u64 bp_len;
        u64 config2; // extension of config1
    };
};

typedef struct perf_event_attr perf_event_attr_t;

struct perf_file_attr {
    perf_event_attr_t attr;
    perf_file_section_t ids; // list of u64 identifiers for matching with .id of a perf sample
};

typedef struct perf_file_attr perf_file_attr_t;

#define PERF_RECORD_MISC_CPUMODE_MASK 0
#define PERF_RECORD_MISC_CPUMODE_UNKNOWN 1
#define PERF_RECORD_MISC_KERNEL 2
#define PERF_RECORD_MISC_USER 3
#define PERF_RECORD_MISC_HYPERVISOR 4
#define PERF_RECORD_MISC_GUEST_KERNEL 5
#define PERF_RECORD_MISC_GUEST_USER 6

#define PERF_RECORD_MISC_EXACT_IP (1 << 14)
#define PERF_RECORD_MISC_EXT_RESERVED (1 << 15)

struct perf_event_header {
    u32 type; // value from perf_event_type enum
    u16 misc; // represents some misc information with the #define's above
    u16 size; // size of the record (including the header)
};

enum perf_event_type {
    PERF_RECORD_MMAP = 0,
    PERF_RECORD_COMM = 1,
    PERF_RECORD_EXIT = 2,
    PERF_RECORD_FORK = 3,
    PERF_RECORD_SAMPLE = 4,
};

typedef enum perf_event_type perf_event_type_t;

// comm_event is used when .type of perf_event_header is PERF_RECORD_COMM
struct comm_event {
    u32 pid; // process id
    u32 tid; // thread id
    char comm[16]; // name of the application
};

typedef struct comm_event comm_event_t;

// mmap_event is used when .type of perf_event_header is PERF_RECORD_MMAP
struct mmap_event {
    u32 pid; // process id
    u32 tid; // thread id
    u64 start; // start of memory range
    u64 len; // length of memory range
    u64 pgoff; // page offset
    char filename[PATH_MAX]; // binary file using this range
};

typedef struct mmap_event mmap_event_t;

// fork_event is used when .type of perf_event_header is PERF_RECORD_FORK or PERF_RECORD_EXIT
struct fork_event {
    u32 pid; // process id
    u32 ppid; // parent process id
    u32 tid; // thread id
    u32 ptid; // parent thread id
    u64 time; // timestamp
};

typedef struct fork_event fork_event_t;

// perf_sample is used when .type of perf_event_header is PERF_RECORD_SAMPLE
struct perf_sample {
    u64 ip; // instruction pointer
    u32 pid; // process id
    u32 tid; // thread id
    u64 time; // timestamp. this isn't well known but it should just be increasing set of numbers for perf to sort samples
    u64 addr; // probably unused for our case
    u64 id; // identification
    u64 stream_id;
    u32 cpu; // used cpu
    u32 res;
    u64 period; // number of events recorded for the sample
    u64 values; // not sure what this is
    u64 nr; // number of call stack addresses
    u64 ips[CALL_STACK_DEPTH]; // call stack - provides a trace of addresses for functions called
};

typedef struct perf_sample perf_sample_t;

#endif