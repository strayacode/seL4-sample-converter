use std::{fs::File, io::Write, str, mem};

use crate::{as_raw_bytes, sample::Sel4Sample, perf::attributes::{AttributeFlags, EventAttribute, SampleType}};

use self::{events::{SampleEvent, CommEvent}, header::Header, attributes::FileAttribute};

pub mod header;
pub mod file_section;
pub mod features;
pub mod attributes;
pub mod events;

#[repr(C)]
pub struct PerfFile {
    header: Header,
    attribute: FileAttribute,
    samples: Vec<SampleEvent>,
}

impl PerfFile {
    pub fn new() -> std::io::Result<Self> {
        let header = Header::new();
        let magic_bytes = header.magic.to_le_bytes();
        let magic_string = str::from_utf8(&magic_bytes).unwrap();
        assert_eq!(magic_string, "PERFILE2");

        let mut attribute = FileAttribute::default();
        attribute.attr.size = mem::size_of::<EventAttribute>() as u32;

        // arbitrary sample frequency value
        attribute.attr.sample_period_or_freq = 4000;

        // include all sample information
        attribute.attr.sample_type = SampleType::IP | SampleType::TID | SampleType::TIME | SampleType::CPU | SampleType::PERIOD;

        attribute.attr.attr_flags = AttributeFlags::DISABLED
            | AttributeFlags::INHERIT
            | AttributeFlags::COMM
            | AttributeFlags::FREQ
            | AttributeFlags::SAMPLE_ID_ALL
            | AttributeFlags::COMM_EXEC;

        Ok(PerfFile {
            header,
            attribute,
            samples: Vec::new(),
        })
    }

    fn write_to_file<T>(data: &T, file: &mut File) -> std::io::Result<()> {
        let bytes = as_raw_bytes(data);
        file.write_all(bytes)
    }

    pub fn create_comm_event(&mut self, pid: u32, application: String) {
        // each time we add a comm event the data section size must be increased
        let comm_event = CommEvent::new(pid, application);
    }

    pub fn create_sample_event(&mut self, sample: Sel4Sample) {
        // each time we add a sample event the data section size must be increased
        let sample_event = SampleEvent::new(sample);
        self.header.data.size += mem::size_of::<SampleEvent>() as u64;
        self.samples.push(sample_event);
    }

    pub fn print_summary(&self) {
        println!("header:");
        println!("{:?}", self.header);

        println!("attributes:");
        println!("{:?}", self.attribute);

        println!("samples:");
        for sample in &self.samples {
            println!("{:?}", sample);
        }
    }

    pub fn dump_to_file(&mut self, file: &mut File) -> std::io::Result<()> {
        Self::write_to_file(&self.header, file)?;
        Self::write_to_file(&self.attribute, file)?;

        for sample in &self.samples {
            Self::write_to_file(sample, file)?;
        }

        println!("profile data successfully dumped to perf.data");
        Ok(())
    }
}