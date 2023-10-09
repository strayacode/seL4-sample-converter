use std::fs::File;

use sample_converter::{perf::PerfFile, sample::Sel4Sample};

fn main() -> std::io::Result<()> {
    let mut file = File::create("perf.data")?;
    let mut perf_file = PerfFile::new()?;
    
    // add some test samples
    let sample1 = Sel4Sample {
        ip: 0x12345678,
        pid: 3,
        timestamp: 0,
        cpu: 0,
        period: 600,
    };

    let sample2: Sel4Sample = Sel4Sample {
        ip: 0x87654321,
        pid: 2,
        timestamp: 2,
        cpu: 3,
        period: 1000,
    };

    perf_file.add_sel4_sample(sample1);
    perf_file.add_sel4_sample(sample2);
    perf_file.print_summary();
    perf_file.dump_to_file(&mut file)?;

    Ok(())
}
