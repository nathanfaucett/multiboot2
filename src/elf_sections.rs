

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
    pub fn get_number_of_sections(&'static self) -> u32 {
        self.number_of_sections
    }
    pub fn get_sections(&'static self) -> ElfSectionIter {
        ElfSectionIter {
            current_section: &self.first_section,
            remaining_sections: self.number_of_sections - 1,
            entry_size: self.entry_size,
        }
    }
}

#[derive(Clone)]
pub struct ElfSectionIter {
    current_section: &'static ElfSection,
    remaining_sections: u32,
    entry_size: u32,
}

impl Iterator for ElfSectionIter {
    type Item = &'static ElfSection;
    fn next(&mut self) -> Option<&'static ElfSection> {
        if self.remaining_sections == 0 {
            None
        } else {
            let section = self.current_section;
            let next_section_addr = (self.current_section as *const _ as u32) + self.entry_size;
            self.current_section = unsafe{ &*(next_section_addr as *const ElfSection) };
            self.remaining_sections -= 1;
			if section.typ == ElfSectionType::Unused as u32 {
				self.next()
			} else {
	            Some(section)
			}
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct ElfSection {
    name: u32,
    typ: u32,
    flags: u64,
    address: u64,
    offset: u64,
    size: u64,
    link: u32,
    info: u32,
    address_align: u64,
    entry_size: u64,
}

impl ElfSection {

    pub fn get_start_address(&self) -> usize {
        self.address as usize
    }

    pub fn get_end_address(&self) -> usize {
        (self.address + self.size) as usize
    }

    pub fn get_flags(&self) -> ElfSectionFlags {
        ElfSectionFlags::from_bits_truncate(self.flags)
    }

    pub fn is_allocated(&self) -> bool {
        self.get_flags().contains(ELF_SECTION_ALLOCATED)
    }
}

#[repr(u32)]
pub enum ElfSectionType {
    Unused = 0,
    ProgramSection = 1,
    LinkerSymbolTable = 2,
    StringTable = 3,
    RelaRelocation = 4,
    SymbolHashTable = 5,
    DynamicLinkingTable = 6,
    Note = 7,
    Uninitialized = 8,
    RelRelocation = 9,
    Reserved = 10,
    DynamicLoaderSymbolTable = 11,
    // plus environment-specific use from 0x60000000 to 0x6FFFFFFF
    // plus processor-specific use from 0x70000000 to 0x7FFFFFFF
}

bitflags! {
    pub flags ElfSectionFlags: u64 {
        const ELF_SECTION_WRITABLE = 0x1,
        const ELF_SECTION_ALLOCATED = 0x2,
        const ELF_SECTION_EXECUTABLE = 0x4,
        // plus environment-specific use at 0x0F000000
        // plus processor-specific use at 0xF0000000
    }
}
