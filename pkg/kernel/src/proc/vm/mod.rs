use alloc::format;
use stack::{STACK_DEF_PAGE, STACK_INIT_BOT};
use x86_64::{
    structures::paging::{page::*, *},
    VirtAddr,
};
use xmas_elf::ElfFile;

use crate::{humanized_size, memory::*};

pub mod stack;

use self::stack::Stack;

use super::{manager::get_process_manager, PageTableContext, ProcessId};

type MapperRef<'a> = &'a mut OffsetPageTable<'static>;
type FrameAllocatorRef<'a> = &'a mut BootInfoFrameAllocator;

pub struct ProcessVm {
    // page table is shared by parent and child
    pub(super) page_table: PageTableContext,

    // stack is pre-process allocated
    pub(super) stack: Stack,
}

impl ProcessVm {
    pub fn new(page_table: PageTableContext) -> Self {
        Self {
            page_table,
            stack: Stack::empty(),
        }
    }

    pub fn init_kernel_vm(mut self) -> Self {
            // TODO: record kernel code usage
            
            self.stack = Stack::kstack();
            self
        }

    // pub fn init_proc_stack(&mut self, pid: ProcessId) -> VirtAddr {
    //     // FIXME: calculate the stack for pid
    //     // FIXME: calculate the stack for pid
    //     let frame_allocator = &mut *get_frame_alloc_for_sure();
    //     let stack_top = stack::STACK_INIT_TOP - ((pid.0 as u64 - 1)* 0x1_0000_0000);
    //     let stack_bot = stack::STACK_INIT_BOT - ((pid.0 as u64 - 1)* 0x1_0000_0000);
    //     let page_table = &mut self.page_table.mapper();
        
    //     let stack_top_addr = VirtAddr::new(stack_top);
        
    //     let _ = elf::map_range(stack_top, STACK_DEF_PAGE, page_table, frame_allocator,true);

    //     self.stack = Stack::new(
    //         Page::containing_address(stack_top_addr),
    //         STACK_DEF_PAGE,
    //     );
    //     stack_top_addr
    // }

    pub fn handle_page_fault(&mut self, addr: VirtAddr) -> bool {
        let mapper = &mut self.page_table.mapper();
        let alloc = &mut *get_frame_alloc_for_sure();

        self.stack.handle_page_fault(addr, mapper, alloc)
    }

    pub(super) fn memory_usage(&self) -> u64 {
        self.stack.memory_usage()
    }

    pub fn load_elf(&mut self, elf:&ElfFile){  //fixed: impl load_elf
        let mapper = &mut self.page_table.mapper();
        let alloc = &mut *get_frame_alloc_for_sure();
        
        self.stack.init(mapper, alloc);

        _ = elf::load_elf(elf, *PHYSICAL_OFFSET.get().unwrap(), mapper, alloc, true);
       
    }
}

impl core::fmt::Debug for ProcessVm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let (size, unit) = humanized_size(self.memory_usage());

        f.debug_struct("ProcessVm")
            .field("stack", &self.stack)
            .field("memory_usage", &format!("{} {}", size, unit))
            .field("page_table", &self.page_table)
            .finish()
    }
}
