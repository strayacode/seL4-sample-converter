#ifndef SAMPLE_H
#define SAMPLE_H

#include "types.h"

// this file defines the format used for sel4 profiling packets

#define CALL_STACK_DEPTH 10

struct sel4_sample {
    u64 ip; // instruction pointer
    u32 pid; // process id
    u64 time; // timestamp of when the sample occured
    u32 cpu; // cpu affinity - which cpu is being used
    u64 period; // number of events per sample
    u64 ips[CALL_STACK_DEPTH]; // call stack - provides a trace of addresses for functions called
};

typedef struct sel4_sample sel4_sample_t;

#endif