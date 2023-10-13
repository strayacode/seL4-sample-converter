use std::fs::File;

use sample_converter::{perf::PerfFile, sample::Sel4Sample, sample_parser};

fn main() -> std::io::Result<()> {
    let mut file = File::create("perf.data")?;
    let mut perf_file = PerfFile::new()?;

    // add samples from samples file
    let samples_file = sample_parser::parse_samples("samples/a.json")?;

    for sample in samples_file.samples {
        perf_file.create_sample_event(sample);
    }

    perf_file.print_summary();
    perf_file.dump_to_file(&mut file)?;

    Ok(())
}
