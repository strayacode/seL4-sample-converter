#include <stdio.h>
#include <string.h>
#include "types.h"
#include "perf.h"
#include "sample.h"

static u64 next_perf_sample_id = 0;

// this will be updated as we receive sel4 samples
static perf_file_header_t header;

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

    // not sure what this is
    perf_sample.stream_id = 0;

    perf_sample.cpu = sel4_sample.cpu;

    // not sure what this is
    perf_sample.res = 0;

    // number of events recorded for the sample
    perf_sample.period = sel4_sample.period;

    // not sure what this is
    perf_sample.values = 0;

    // number of call stack addresses
    perf_sample.nr = CALL_STACK_DEPTH;

    memcpy(perf_sample.ips, sel4_sample.ips, CALL_STACK_DEPTH * sizeof(u64));

    return perf_sample;
}

static void header_init(void) {
    // currently we are using perf file format v1
    memcpy(&header.magic, "PERFFILE", 8);

    header.size = sizeof(perf_file_header_t);
    header.attr_size = sizeof(perf_file_attr_t);

    // the attrs file section will occur right after the header
    // in the file
    header.attrs.offset = header.size;

    // our perf file will only contain 1 file_attr with no ids
    header.attrs.size = sizeof(perf_file_attr_t);
    
    // since we only have 1 attribute, we can have the data file
    // section start right after the attribute
    header.data.offset = header.attrs.offset + header.attrs.size;

    // when initialising the header we don't know yet how many records we will
    // instantiate, so leave this initialised at 0 for now
    header.data.size = 0;

    // TODO: what values should header.event_types have?
    header.event_types.offset = 0;
    header.event_types.size = 0;

    // probably unused so set to 0
    header.features0 = 0;
    header.features1 = 0;
    header.features2 = 0;
    header.features3 = 0;
}

static void receive_sel4_sample(sel4_sample_t sel4_sample) {
    perf_sample_t perf_sample = convert_to_perf_sample(sel4_sample);
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        printf("usage: ./seL4-sample-convert <sample_file>\n");
        return 1;
    }

    // overall flow of application
    // we will continually receive sample packets
    // the layout of the file will be like:
    // header
    // a single perf_file_attr since we only have one event source (samples)
    // sample_id_all should always be set for attr
    // 

    char *sample_file = argv[1];

    // initialise the header with default values
    header_init();

    // mainloop for receiving sel4 sample packets will go here
    // e.g.
    // while (can_receive_packets) {
    //     receive_sel4_sample(sample);
    // }

    // each packet will be stored into memory and will update file section offsets/sizes as well

    // when we stop receiving packets, we will write
    // the header, attribute and data section to a file
    FILE* perf_data_file = fopen("build/perf.data", "w");

    fwrite(&header, sizeof(perf_file_header_t), 1, perf_data_file);

    fclose(perf_data_file);
    return 0;
}