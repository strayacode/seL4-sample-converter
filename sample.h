#ifndef SAMPLE_H
#define SAMPLE_H

#include <stdint.h>

// this file defines the format used for sel4 profiling packets

#define CALL_STACK_DEPTH 10

struct sel4_sample {
    uint64_t ip; // instruction pointer
    uint32_t pid; // process id
    uint64_t time; // timestamp of when the sample occured
    uint32_t cpu; // cpu affinity - which cpu is being used
    uint64_t period; // number of events per sample
    uint64_t ips[CALL_STACK_DEPTH]; // call stack - provides a trace of addresses for functions called
};

typedef sel4_sample sel4_sample_t;

#endif