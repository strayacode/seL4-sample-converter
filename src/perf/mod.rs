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
    comm_events: Vec<CommEvent>,
    sample_events: Vec<SampleEvent>,
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
            comm_events: Vec::new(),
            sample_events: Vec::new(),
        })
    }

    fn write_to_file<T>(data: &T, file: &mut File) -> std::io::Result<()> {
        let bytes = as_raw_bytes(data);
        file.write_all(bytes)
    }

    pub fn create_comm_event(&mut self, pid: u32, application: String) {
        // each time we add a comm event the data section size must be increased
        let comm_event = CommEvent::new(pid, application);
        self.header.data.size += mem::size_of::<CommEvent>() as u64;
        self.comm_events.push(comm_event);
    }

    pub fn create_sample_event(&mut self, sample: Sel4Sample) {
        // each time we add a sample event the data section size must be increased
        let sample_event = SampleEvent::new(sample);
        self.header.data.size += mem::size_of::<SampleEvent>() as u64;
        self.sample_events.push(sample_event);
    }

    pub fn print_summary(&self) {
        println!("header:");
        println!("{:?}", self.header);

        println!("attributes:");
        println!("{:?}", self.attribute);

        println!("comm events");
        for comm_event in &self.comm_events {
            println!("{:?}", comm_event);
        }

        println!("sample events:");
        for sample in &self.sample_events {
            println!("{:?}", sample);
        }
    }

    pub fn dump_to_file(&mut self, file: &mut File) -> std::io::Result<()> {
        Self::write_to_file(&self.header, file)?;
        Self::write_to_file(&self.attribute, file)?;

        for comm_event in &self.comm_events {
            Self::write_to_file(comm_event, file)?;
        }

        for sample_event in &self.sample_events {
            Self::write_to_file(sample_event, file)?;
        }

        println!("profile data successfully dumped to perf.data");
        Ok(())
    }
}