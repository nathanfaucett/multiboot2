use elf_section::{ElfSection, ElfSectionIter};


#[derive(Debug)]
#[repr(packed)] // repr(C) adds unwanted padding before first_section
pub struct ElfSectionsTag {
    kind: u32,
    size: u32,
    number_of_sections: u32,
    entry_size: u32,
    shndx: u32, // string table
    first_section: ElfSection,
}

impl ElfSectionsTag {
    #[inline(always)]
    pub fn kind(&'static self) -> u32 { self.kind }
    #[inline(always)]
    pub fn size(&'static self) -> u32 { self.size }
    #[inline(always)]
    pub fn number_of_sections(&'static self) -> u32 { self.number_of_sections }
    #[inline(always)]
    pub fn entry_size(&'static self) -> u32 { self.entry_size }
    #[inline(always)]
    pub fn shndx(&'static self) -> u32 { self.shndx }
    #[inline(always)]
    pub fn first_section(&'static self) -> &ElfSection { &self.first_section }

    #[inline(always)]
    pub fn sections(&'static self) -> ElfSectionIter {
        ElfSectionIter::new(
            &self.first_section,
            self.number_of_sections - 1,
            self.entry_size,
        )
    }
}
