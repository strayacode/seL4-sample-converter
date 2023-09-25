#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include "types.h"
#include "perf.h"
#include "sample.h"

#define SAMPLE_FREQ 100

static u64 next_perf_sample_id = 0;

struct perf_sample_event_node {
    perf_sample_event_t data;
    struct perf_sample_event_node *next;
};

typedef struct perf_sample_event_node perf_sample_event_node_t;

static perf_sample_event_node_t *head = NULL;

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

static perf_file_header_t create_header(void) {
    perf_file_header_t header;

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
    return header;
}

static perf_file_attr_t create_attr(void) {
    perf_file_attr_t file_attr;

    // not sure what type should be set to
    file_attr.attr.type = 0;

    file_attr.attr.size = sizeof(perf_event_attr_t);

    // the sampling frequency used by the sel4 profiler should be set here
    // for now we will use an arbitrary value
    file_attr.attr.sample_freq = SAMPLE_FREQ;

    // set the sample type to include all the sample information we need
    file_attr.attr.sample_type = PERF_SAMPLE_IP | PERF_SAMPLE_TID |
        PERF_SAMPLE_TIME | PERF_SAMPLE_ADDR |
        PERF_SAMPLE_READ | PERF_SAMPLE_CALLCHAIN |
        PERF_SAMPLE_ID | PERF_SAMPLE_CPU |
        PERF_SAMPLE_PERIOD | PERF_SAMPLE_STREAM_ID |
        PERF_SAMPLE_RAW;

    file_attr.attr.freq = true;
    file_attr.attr.sample_id_all = true;

    // our single attribute won't have any ids, so initialise to 0
    file_attr.ids.offset = 0;
    file_attr.ids.size = 0;
    return file_attr;
}

static perf_sample_event_t receive_sel4_sample(sel4_sample_t sel4_sample) {
    // each sample requires a new data record to be created
    perf_sample_event_t sample_event;

    // for now we only handle sample event types
    sample_event.header.type = PERF_RECORD_SAMPLE;

    // not exactly sure what misc type we should use here
    sample_event.header.misc = PERF_RECORD_MISC_USER;

    sample_event.header.size = sizeof(perf_sample_event_t);

    sample_event.sample = convert_to_perf_sample(sel4_sample);
    return sample_event;
}

static void append_sample_event(perf_file_header_t *header, perf_sample_event_t sample_event) {
    perf_sample_event_node_t *sample_event_node = malloc(sizeof(perf_sample_event_node_t));
    sample_event_node->data = sample_event;
    sample_event_node->next = NULL;

    if (head == NULL) {
        head = sample_event_node;
        return;
    }

    perf_sample_event_node_t *curr = head;
    while (curr->next != NULL) {
        curr = curr->next;
    }

    curr->next = sample_event_node;

    // increase the size of the data section in the header
    header->data.size += sizeof(perf_sample_event_t);
}

int main(void) {
    // overall flow of application
    // we will continually receive sample packets
    // the layout of the file will be like:
    // header
    // a single perf_file_attr since we only have one event source (samples)
    // sample_id_all should always be set for attr
    // 

    // initialise the header with default values
    // this will be updated as we receive sel4 samples
    perf_file_header_t header = create_header();

    // create a single perf_file_attr
    perf_file_attr_t file_attr = create_attr();

    // mainloop for receiving sel4 sample packets will go here
    // e.g.
    // while (can_receive_packets) {
        // perf_sample_event_t sample_event = receive_sel4_sample(sample);

        // store to memory
    // }

    sel4_sample_t first_sample;
    first_sample.ip = 0x12345678;
    first_sample.pid = 0xDEADBEEF;
    first_sample.time = 0;
    first_sample.cpu = 0;
    first_sample.period = 100;

    for (int i = 0; i < CALL_STACK_DEPTH; i++) {
        first_sample.ips[i] = i;
    }

    sel4_sample_t second_sample;
    second_sample.ip = 0x87654321;
    second_sample.pid = 0xCAFEBEEF;
    second_sample.time = 2;
    second_sample.cpu = 3;
    second_sample.period = 200;

    for (int i = 0; i < CALL_STACK_DEPTH; i++) {
        second_sample.ips[i] = i;
    }

    append_sample_event(&header, receive_sel4_sample(first_sample));
    append_sample_event(&header, receive_sel4_sample(second_sample));

    // each packet will be stored into memory and will update header.data.size each time
    // (number of records grows)

    // when we stop receiving packets, we will write
    // the header, attribute and data section to a file
    FILE* perf_data_file = fopen("build/perf.data", "w");

    fwrite(&header, sizeof(perf_file_header_t), 1, perf_data_file);
    fwrite(&file_attr, sizeof(perf_file_attr_t), 1, perf_data_file);

    // write each sample event to the file one by one
    perf_sample_event_node_t *curr = head;
    while (curr != NULL) {
        perf_sample_event_node_t *temp = curr;
        fwrite(&curr->data, sizeof(perf_sample_event_t), 1, perf_data_file);
        curr = curr->next;
        free(temp);
    }

    fclose(perf_data_file);
    return 0;
}