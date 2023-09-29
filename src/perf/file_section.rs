use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Default, Clone, Copy, Pod, Zeroable)]
pub struct FileSection {
    // an offset into the file for where this section starts
    offset: u64,

    // the size of the section
    size: u64,
}