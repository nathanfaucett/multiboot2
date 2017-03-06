use super::elf_sections_tag::ElfSectionsTag;
use super::memory_map_tag::MemoryMapTag;


#[repr(C)]
pub struct Multiboot2 {
    total_size: u32,
    reserved: u32,
    first_tag: Tag,
}

impl Multiboot2 {
    #[inline]
    pub unsafe fn new(address: usize) -> &'static Multiboot2 {
        let multiboot2 = &*(address as *const Multiboot2);
        assert!(multiboot2.has_valid_end_tag());
        multiboot2
    }
    #[inline(always)]
    pub fn start_address(&self) -> usize {
        self as *const _ as usize
    }
    #[inline(always)]
    pub fn end_address(&self) -> usize {
        self.start_address() + self.total_size as usize
    }
    #[inline(always)]
    pub fn elf_sections_tag(&self) -> Option<&'static ElfSectionsTag> {
        self.tag(9).map(|tag| unsafe{
            &*(tag as *const Tag as *const ElfSectionsTag)
        })
    }
    #[inline(always)]
    pub fn memory_map_tag(&self) -> Option<&'static MemoryMapTag> {
        self.tag(6).map(|tag| unsafe{
            &*(tag as *const Tag as *const MemoryMapTag)
        })
    }
    #[inline]
    fn has_valid_end_tag(&self) -> bool {
        const END_TAG: Tag = Tag{
            kind: 0,
            size: 8,
        };

        let self_ptr = self as *const _;
        let end_tag_addr = self_ptr as usize + (self.total_size - END_TAG.size) as usize;
        let end_tag = unsafe{
            &*(end_tag_addr as *const Tag)
        };

        end_tag.kind == END_TAG.kind && end_tag.size == END_TAG.size
    }
    #[inline(always)]
    fn tag(&self, kind: u32) -> Option<&'static Tag> {
        self.tags().find(|tag| tag.kind == kind)
    }
    #[inline(always)]
    fn tags(&self) -> TagIter {
        TagIter {
            current: &self.first_tag as *const _,
        }
    }
}

#[repr(C)]
struct Tag {
    kind: u32,
    size: u32,
}

struct TagIter {
    current: *const Tag,
}

impl Iterator for TagIter {
    type Item = &'static Tag;

    #[inline]
    fn next(&mut self) -> Option<&'static Tag> {
        match unsafe { &*self.current } {
            &Tag{
                kind: 0,
                size: 8,
            } => None,
            tag => {
                let mut tag_addr = self.current as usize;
                tag_addr += tag.size as usize;
                tag_addr = ((tag_addr-1) & !0x7) + 0x8;
                self.current = tag_addr as *const _;
                Some(tag)
            },
        }
    }
}
