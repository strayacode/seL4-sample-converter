# seL4-sample-converter
- This repository serves as a tool to convert from seL4 sample packets to a format that can perf can interpret

- By utilising the pre-existing perf infrastructure we're able to take advantage of the many perf tools, as well as open source tools like flamegraph

## Installation
- This tool is written in the Rust Programming Language, and can be installed by following the link https://www.rust-lang.org/tools/install

- ``git clone https://github.com/strayacode/seL4-sample-converter.git``
- ``cd seL4-sample-converter``

## Building
- Run ``cargo build``. The binary will be built at ``/target/debug/sample_converter``

## Running
- Running the binary: ``./target/debug/sample_converter <-s/--samples_path insert_sample_file_path_here>``
- Running through cargo: ``cargo run -- <-s/--samples_path insert_sample_file_path_here>``
- Similar to ``perf record``, a ``perf.data`` file will be outputted in the current directory
- This ``perf.data`` file can be used to visualise samples recorded from an seL4 based system through commands such as ``perf report``, etc

## Details
- To use this tool, you must provide samples information in the following format

### Example sample file:
```
{
    "pd_mappings": {
        "a.elf": 0
    },
    "samples": [
        {
            "ip": 0,
            "pid": 0,
            "timestamp": 0,
            "cpu": 0,
            "period": 300
        },
        {
            "ip": 4,
            "pid": 0,
            "timestamp": 20,
            "cpu": 0,
            "period": 300
        },
        {
            "ip": 8,
            "pid": 0,
            "timestamp": 40,
            "cpu": 0,
            "period": 300
        },
    ]
}
```

- pd_mappings: an object that maps keys (elf paths) to pids expected by perf
- samples: contains a list of objects with the following fields:
    - ip: the instruction pointer of the cpu when the sample was recorded
    - pid: the process id of the process the sample was running in
        - for seL4 purposes we term pd is used instead
    - timestamp: the time of when the sample was recorded since the program started running
    - cpu: the id of the cpu that the sample was recorded on
    - period: refers to how often sample data is sampleds