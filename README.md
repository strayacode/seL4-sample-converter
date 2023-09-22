# seL4-sample-converter

This repository serves as a tool to convert from seL4 sample packets to a format that can
be used to visualise them with ``perf report``

``perf report`` expects a file called ``perf.data`` or similar,
which should be stored in a format specific to perf

More info can be found at https://openlab-mu-internal.web.cern.ch/03_Documents/3_Technical_Documents/Technical_Reports/2011/Urs_Fassler_report.pdf

## Building
- ``mkdir build``
- ``make``

## Running
- ``cd build``
- ``./sel4-sample-converter <path_to_samples_file>``