#!/bin/sh

make
./build/seL4-sample-converter
cd build/
perf report -v -D -I --header perf.data