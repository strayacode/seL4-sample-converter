use std::{fs::File, path::Path};

use sample_converter::{perf::PerfFile, sample_parser};

fn main() -> std::io::Result<()> {
    let mut file = File::create("perf.data")?;
    let mut perf_file = PerfFile::new()?;

    // add samples from samples file
    let samples_file = sample_parser::parse_samples("samples/symbolstuff.json")?;

    for (application, pid) in samples_file.pd_mappings {
        let filename = Path::new(&application).file_name().unwrap().to_str().unwrap();
        perf_file.create_comm_event(pid, filename);
        perf_file.create_mmap_event(pid, &application);
    }

    for sample in samples_file.samples {
        perf_file.create_sample_event(sample);
    }

    perf_file.print_summary();
    perf_file.dump_to_file(&mut file)?;

    Ok(())
}
