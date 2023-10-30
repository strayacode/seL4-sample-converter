# seL4-sample-converter

This repository serves as a tool to convert from seL4 sample packets to a format that can
be used to visualise them with ``perf report``

``perf report`` expects a file called ``perf.data`` or similar,
which should be stored in a format specific to perf

# Running:
- ``cargo run``
- eventually a ``samples.json`` file should be provided for the samples,
and a path to the directory should be provided so that the program can access the sel4 system file
- this will produce a ``perf.data`` file
- ``perf report -D -I -v --header perf.data``