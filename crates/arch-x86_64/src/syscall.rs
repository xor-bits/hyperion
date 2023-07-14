use core::arch::asm;

use crossbeam::atomic::AtomicCell;
use memoffset::offset_of;
use x86_64::{
    registers::{
        model_specific::{Efer, EferFlags, LStar, SFMask, Star},
        rflags::RFlags,
    },
    VirtAddr,
};

use crate::{cpu::gdt::SegmentSelectors, tls::ThreadLocalStorage};

//

/// init `syscall` and `sysret`
pub fn init(selectors: SegmentSelectors) {
    // IA32_STAR : 0xC0000081
    Star::write(
        selectors.user_code,
        selectors.user_data,
        selectors.kernel_code,
        selectors.kernel_data,
    )
    .expect("IA32_STAR write incorrect");

    // syscall handler addr
    // IA32_LSTAR : 0xC0000082
    LStar::write(VirtAddr::new(syscall_wrapper as usize as u64));

    // disable interrupts on syscall
    // IA32_LSTAR : 0xC0000084
    SFMask::write(RFlags::INTERRUPT_FLAG /* | RFlags::TRAP_FLAG */);

    // enable syscall, sysret, systenter, sysexit
    // IA32_EFER : 0xC0000080
    unsafe {
        Efer::update(|flags| {
            flags.insert(EferFlags::SYSTEM_CALL_EXTENSIONS);
        });
    }
}

pub fn set_handler(f: fn(&mut SyscallRegs)) {
    SYSCALL_HANDLER.store(f);
}

//

#[allow(unused)]
#[repr(C)]
pub struct SyscallRegs {
    _r15: u64,
    _r14: u64,
    _r13: u64,
    _r12: u64,
    _r11: u64,
    _r10: u64,
    pub arg4: u64, // r9
    pub arg3: u64, // r8
    _rbp: u64,
    pub arg1: u64, // rsi
    pub arg0: u64, // rdi
    pub arg2: u64, // rdx
    _rcx: u64,
    _rbx: u64,
    pub syscall_id: u64, // rax, also the return register
}

//

/// # Safety
/// the `_instr_ptr` (RIP) and `_stack_ptr` (RSP) arguments must be valid user space
/// application virtual addresses
///
/// syscalls must also be initialized before calling this
///
/// this call won't return
#[no_mangle]
pub unsafe extern "sysv64" fn userland(_instr_ptr: VirtAddr, _stack_ptr: VirtAddr) -> ! {
    // rdi = _instr_ptr
    // rsi = _stack_ptr
    asm!(
        "cli",
        "swapgs",
        "mov rcx, rdi", // RDI = _instr_ptr
        "mov rsp, rsi", // RSI = _stack_ptr
        "mov r11, {rflags}",
        // clear some registers
        "xor rax, rax",
        "xor rbx, rbx",
        // no zeroing rcx, sysreq returns to the address in it (`instr_ptr`)
        "xor rdx, rdx",
        "xor rdi, rdi",
        "xor rsi, rsi",
        "xor rbp, rbp",
        // no zeroing rsp, a stack is needed
        "xor r8, r8",
        "xor r9, r9",
        "xor r10, r10",
        // no zeroing r11, it holds RFLAGS
        "xor r12, r12",
        "xor r13, r13",
        "xor r14, r14",
        "xor r15, r15",
        // "call {halt}",
        "sysretq",
        rflags = const(RFlags::INTERRUPT_FLAG.bits() /* | RFlags::TRAP_FLAG.bits() */),
        options(noreturn)
    )
}

//

#[naked]
unsafe extern "C" fn syscall_wrapper() {
    // the stack is still the userland stack
    //
    // rcx = return address
    // rsp = user stack
    // r11 = rflags
    asm!(

        "swapgs", // swap gs and kernelgs to open up a few temporary data locations
        "mov gs:{user_stack}, rsp",   // backup the user stack
        "mov rsp, gs:{kernel_stack}", // switch to the kernel stack

        "push QWORD PTR gs:{user_stack}",

        "push rax",
        "push rbx",
        "push rcx",
        "push rdx",
        "push rdi",
        "push rsi",
        "push rbp",
        "push r8",
        "push r9",
        "push r10",
        "push r11",
        "push r12",
        "push r13",
        "push r14",
        "push r15",

        "mov rdi, rsp",
        "call {syscall}",

        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",
        "pop rbp",
        "pop rsi",
        "pop rdi",
        "pop rdx",
        "pop rcx",
        "pop rbx",
        "pop rax",

        "pop QWORD PTR gs:{user_stack}",

        "mov rsp, gs:{user_stack}",
        "swapgs",
        // TODO: fix the sysret vulnerability
        "sysretq",
        syscall = sym syscall,
        user_stack = const(offset_of!(ThreadLocalStorage, user_stack)),
        kernel_stack = const(offset_of!(ThreadLocalStorage, kernel_stack)),
        options(noreturn)
    );
}

#[inline(always)]
#[no_mangle]
unsafe extern "C" fn syscall(regs: &mut SyscallRegs) {
    hyperion_log::debug!(
        "got syscall {} with args {} {} {} {} {}",
        regs.syscall_id,
        regs.arg0,
        regs.arg1,
        regs.arg2,
        regs.arg3,
        regs.arg4,
    );

    SYSCALL_HANDLER.load()(regs);
}

static SYSCALL_HANDLER: AtomicCell<fn(&mut SyscallRegs)> = AtomicCell::new(|_| {
    hyperion_log::error!("Syscall handler not initialized");
});