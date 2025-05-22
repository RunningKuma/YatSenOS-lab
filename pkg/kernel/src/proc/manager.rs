use super::*;
use crate::{memory::{
    self,
    allocator::{ALLOCATOR, HEAP_SIZE},
    get_frame_alloc_for_sure, PAGE_SIZE,
}, proc::vm::stack::STACK_INIT_TOP};
use alloc::{collections::*, format, sync::{Arc, Weak}};
use spin::{Mutex, RwLock};

use elf::map_range;
use boot::*;

pub static PROCESS_MANAGER: spin::Once<ProcessManager> = spin::Once::new();

pub fn init(init: Arc<Process>, app_list: AppListRef) { //fixed:add app_list arg

    // FIXME: set init process as Running
    init.write().resume();
    
    processor::set_pid(init.pid()); //inspire：不要直接对全局静态对象更改，使用对应的接口进行更改
    // init.write().pause(); //尝试设为ready
    PROCESS_MANAGER.call_once(|| ProcessManager::new(init,app_list)); //fixed:add app_list arg
}

pub fn get_process_manager() -> &'static ProcessManager {
    PROCESS_MANAGER
        .get()
        .expect("Process Manager has not been initialized")
}

pub struct ProcessManager {
    processes: RwLock<BTreeMap<ProcessId, Arc<Process>>>,
    ready_queue: Mutex<VecDeque<ProcessId>>,
    app_list: boot::AppListRef, //fixed: add app list
}

impl ProcessManager {
    pub fn app_list(&self) -> boot::AppListRef{
        self.app_list
    }
    pub fn new(init: Arc<Process>,app_list: boot::AppListRef) -> Self {  //fixed:add app_list arg
        let mut processes = BTreeMap::new();
        let ready_queue = VecDeque::new();
        let pid = init.pid();

        trace!("Init {:#?}", init);

        processes.insert(pid, init);
        Self {
            processes: RwLock::new(processes),
            ready_queue: Mutex::new(ready_queue),
            app_list: app_list, //fixed
        }
    }

    #[inline]
    pub fn push_ready(&self, pid: ProcessId) {
        self.ready_queue.lock().push_back(pid);
    }

    #[inline]
    fn add_proc(&self, pid: ProcessId, proc: Arc<Process>) {
        self.processes.write().insert(pid, proc);
    }

    #[inline]
    fn get_proc(&self, pid: &ProcessId) -> Option<Arc<Process>> {
        self.processes.read().get(pid).cloned()
    }

    pub fn current(&self) -> Arc<Process> {
        self.get_proc(&processor::get_pid())
            .expect("No current process")
    }

    pub fn save_current(&self, context: &ProcessContext) {
        // FIXME: update current process's tick count   √
        self.current().write().tick();
        // FIXME: save current process's context      √
        self.current().write().save(context);
        
    }

    pub fn switch_next(&self, context: &mut ProcessContext) -> ProcessId {
        // FIXME: fetch the next process from ready queue     √
        let mut queue = self.ready_queue.lock();
        let n_pid = &queue.pop_front();
        let mut n_proc = self.get_proc(&n_pid.unwrap()).unwrap();
        // FIXME: check if the next process is ready,
        //        continue to fetch if not ready      √?
        while n_proc.read().status() != ProgramStatus::Ready {
            n_proc = self.get_proc(&queue.pop_front().unwrap()).unwrap();
        }
        // FIXME: restore next process's context  √
        n_proc.write().restore(context);
        // FIXME: update processor's current pid    √
        processor::set_pid(n_proc.pid());
        // FIXME: return next process's pid      √
        drop(queue);
       // print_process_list();
        return n_proc.pid();
    }

    // pub fn spawn_kernel_thread(
    //     &self,
    //     entry: VirtAddr,
    //     name: String,
    //     proc_data: Option<ProcessData>,
    // ) -> ProcessId {
    //     let kproc = self.get_proc(&KERNEL_PID).unwrap();
    //     let page_table = kproc.read().clone_page_table();
    //     let proc_vm = Some(ProcessVm::new(page_table));
    //     let proc = Process::new(name, Some(Arc::downgrade(&kproc)), proc_vm, proc_data);
    //     let pid = proc.pid();
    //     // alloc stack for the new process base on pid
    //     let stack_top = proc.alloc_init_stack();

    //     // FIXME: set the stack frame
    //     proc.write().init_stack_frame(entry, stack_top);
    //     // FIXME: add to process map
    //     self.add_proc(pid,proc);
    //     // FIXME: push to ready queue
    //     self.push_ready(pid);
    //     // FIXME: return new process pid
    //     pid
    // }

    pub fn kill_current(&self, ret: isize) {
        self.kill(processor::get_pid(), ret);
    }

    pub fn handle_page_fault(&self, addr: VirtAddr, err_code: PageFaultErrorCode) -> bool {
        // FIXME: handle page fault
        let proc = self.current();
        debug!("err_code: {:#?}", err_code);
        if !err_code.contains(PageFaultErrorCode::PROTECTION_VIOLATION){
            proc.write().handle_page_fault(addr) //交给进程处理，判断是否在栈空间也一并交给它们
        }
        else{
            warn!("This cause by Protection Violation or other reason: {:#x}", addr);
            return false;
        }
    }

    pub fn kill(&self, pid: ProcessId, ret: isize) {
        let proc = self.get_proc(&pid);

        if proc.is_none() {
            warn!("Process #{} not found.", pid);
            return;
        }

        let proc = proc.unwrap();

        if proc.read().status() == ProgramStatus::Dead {
            warn!("Process #{} is already dead.", pid);
            return;
        }

        trace!("Kill {:#?}", &proc);

        proc.kill(ret);
    }

    pub fn print_process_list(&self) {
        let mut output = String::from("  PID | PPID | Process Name |  Ticks  | Status\n");

        self.processes
            .read()
            .values()
            .filter(|p| p.read().status() != ProgramStatus::Dead)
            .for_each(|p| output += format!("{}\n", p).as_str());

        // TODO: print memory usage of kernel heap

        output += format!("Queue  : {:?}\n", self.ready_queue.lock()).as_str();

        output += &processor::print_processors();

        print!("{}", output);
    }

    pub fn pid_return_code(&self,p_pid: ProcessId) -> Option<isize>{
        x86_64::instructions::interrupts::without_interrupts(|| {
            self.get_proc(&p_pid).expect("No exist").read().exit_code()
        })
    }

    pub fn spawn(
        &self,
        elf: &ElfFile,
        name: String,
        parent: Option<Weak<Process>>,
        proc_data: Option<ProcessData>,
    ) -> ProcessId {
        let kproc = self.get_proc(&KERNEL_PID).unwrap();
        let page_table = kproc.read().clone_page_table();
        let proc_vm = Some(ProcessVm::new(page_table));
        let proc = Process::new(name, parent, proc_vm, proc_data);
        // let stack_top = proc.alloc_init_stack();
        let mut inner = proc.write();
        // FIXME: load elf to process pagetable
        inner.load_elf(elf);
        // FIXME: alloc new stack for process
        // let stack_top = proc.alloc_init_stack();
        inner.init_stack_frame(VirtAddr::new_truncate(elf.header.pt2.entry_point()), VirtAddr::new_truncate(STACK_INIT_TOP));
        // FIXME: mark process as ready
        inner.pause();
        drop(inner);

        trace!("New {:#?}", &proc);

        let pid = proc.pid();
        // FIXME: something like kernel thread
        self.add_proc(pid, proc);
        self.push_ready(pid);
        pid
    }

    pub fn read(&self, fd: u8, buf: &mut [u8]) -> isize { //add read func
        self.current().read().read(fd, buf)
    }

    pub fn write(&self, fd: u8, buf: &[u8]) -> isize { //add write func
        self.current().read().write(fd, buf)
    }

    pub fn get_process_status(&self, pid: ProcessId) -> ProgramStatus {
        self.get_proc(&pid).unwrap().read().status()
    }
}
