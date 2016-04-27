use elf_section::{ElfSection, ElfSectionIter};


#[derive(Debug)]
#[repr(packed)] // repr(C) would add unwanted padding before first_section
pub struct ElfSectionsTag {
    typ: u32,
    size: u32,
    number_of_sections: u32,
    entry_size: u32,
    shndx: u32, // string table
    first_section: ElfSection,
}

impl ElfSectionsTag {
    pub fn get_type(&'static self) -> u32 { self.typ }
    pub fn get_size(&'static self) -> u32 { self.size }
    pub fn get_number_of_sections(&'static self) -> u32 { self.number_of_sections }
    pub fn get_entry_size(&'static self) -> u32 { self.entry_size }
    pub fn get_shndx(&'static self) -> u32 { self.shndx }
    pub fn get_first_section(&'static self) -> &ElfSection { &self.first_section }

    pub fn get_sections(&'static self) -> ElfSectionIter {
        ElfSectionIter::new(
            &self.first_section,
            self.number_of_sections - 1,
            self.entry_size,
        )
    }
}
