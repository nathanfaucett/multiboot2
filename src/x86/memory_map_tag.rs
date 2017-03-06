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
    #[inline(always)]
    pub fn kind(&self) -> u32 { self.kind }
    #[inline(always)]
    pub fn size(&self) -> u32 { self.size }
    #[inline(always)]
    pub fn entry_size(&self) -> u32 { self.entry_size }
    #[inline(always)]
    pub fn entry_version(&self) -> u32 { self.entry_version }

    #[inline]
    pub fn memory_areas(&self) -> MemoryAreaIter {
        let self_ptr = self as *const MemoryMapTag;
        let start_area = (&self.first_area) as *const MemoryArea;

        MemoryAreaIter::new(
            start_area,
            ((self_ptr as u32) + self.size - self.entry_size) as *const MemoryArea,
            self.entry_size,
        )
    }
}
