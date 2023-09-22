#include <stdio.h>
#include "perf.h"
#include "sample.h"

int main(int argc, char *argv[]) {
    if (argc != 2) {
        printf("usage: ./seL4-sample-convert <sample_file>\n");
        return 1;
    }

    char *sample_file = argv[1];

    return 0;
}