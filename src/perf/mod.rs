use std::{fs::File, io::Write, path::Path};

use crate::as_raw_bytes;

pub mod header;
pub mod file_section;
pub mod features;
pub mod attributes;

pub struct PerfFile {
    inner: File,
}

impl PerfFile {
    pub fn new<P>(path: P) -> std::io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let inner = File::create(path)?;

        Ok(PerfFile {
            inner,
        })
    }

    pub fn write<T>(&mut self, data: &T) -> std::io::Result<()> {
        let bytes = as_raw_bytes(data);
        self.inner.write_all(bytes)
    }
}