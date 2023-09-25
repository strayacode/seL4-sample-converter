#include <stdio.h>
#include <string.h>
#include "types.h"
#include "perf.h"
#include "sample.h"

static u64 next_perf_sample_id = 0;

static u64 generate_perf_sample_id(void) {
    u64 id = next_perf_sample_id;
    next_perf_sample_id++;
    return id;
}

static perf_sample_t convert_to_perf_sample(sel4_sample_t sel4_sample) {
    perf_sample_t perf_sample;
    perf_sample.ip = sel4_sample.ip;
    perf_sample.pid = sel4_sample.pid;
    perf_sample.tid = sel4_sample.pid;
    perf_sample.time = sel4_sample.time;
    perf_sample.id = generate_perf_sample_id();
}

static perf_file_header_t create_perf_header(void) {
    perf_file_header_t header;
    memcpy(&header.magic, "PERFFILE", 8);
    header.size = sizeof(perf_file_header_t);
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        printf("usage: ./seL4-sample-convert <sample_file>\n");
        return 1;
    }

    char *sample_file = argv[1];

    return 0;
}