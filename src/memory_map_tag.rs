use memory_area::{MemoryArea, MemoryAreaIter};


#[repr(C)]
pub struct MemoryMapTag {
    kind: u32,
    size: u32,
    entry_size: u32,
    entry_version: u32,
    first_area: MemoryArea,
}

impl MemoryMapTag {
    pub fn get_kind(&self) -> u32 { self.kind }
    pub fn get_type(&self) -> u32 { self.kind }
    pub fn get_size(&self) -> u32 { self.size }
    pub fn get_entry_size(&self) -> u32 { self.entry_size }
    pub fn get_entry_version(&self) -> u32 { self.entry_version }

    pub fn get_memory_areas(&self) -> MemoryAreaIter {
        let self_ptr = self as *const MemoryMapTag;
        let start_area = (&self.first_area) as *const MemoryArea;

        MemoryAreaIter::new(
            start_area,
            ((self_ptr as u32) + self.size - self.entry_size) as *const MemoryArea,
            self.entry_size,
        )
    }
}
