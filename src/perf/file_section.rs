use std::fmt;

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Default, Clone, Copy, Pod, Zeroable)]
pub struct FileSection {
    // an offset into the file for where this section starts
    offset: u64,

    // the size of the section
    size: u64,
}

impl fmt::Display for FileSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:#016x}, {:#016x})", self.offset, self.offset + self.size)
    }
}