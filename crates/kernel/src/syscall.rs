use hyperion_arch::{syscall::SyscallRegs, vmm::PageMap};
use hyperion_drivers::acpi::hpet::HPET;
use hyperion_log::*;
use hyperion_mem::vmm::PageMapImpl;
use x86_64::{structures::paging::PageTableFlags, VirtAddr};

//

pub fn syscall(args: &mut SyscallRegs) {
    let id = args.syscall_id;
    let (result, name): (u64, &str) = match id {
        1 => (log(args), "log"),

        2 | 420 => exit(args),

        3 => (yield_now(args), "yield_now"),

        4 => (timestamp(args), "timestamp"),

        5 => (nanosleep(args), "nanosleep"),

        _ => {
            debug!("invalid syscall");
            // invalid syscall id, kill the process as a f u
            args.syscall_id = 2;
            args.arg0 = i64::MIN as _;
            exit(args);
        }
    };

    if result != 0 {
        debug!("syscall `{name}` (id {id}) returned {result}",);
    }
    args.syscall_id = result;
}

/// print a string to logs
///
/// # arguments
/// - syscall_id : 1
/// - arg0 : _utf8 string address_
/// - arg1 : _utf8 string length_
/// - arg2 : _ignored_
/// - arg3 : _ignored_
/// - arg4 : _ignored_
///
/// # return codes (in syscall_id after returning)
///  - 0 : ok
///  - 1 : invalid address range (arg0 .. arg1)
///  - 2 : address range not mapped for the user (arg0 .. arg1)
///  - 3 : invalid utf8
pub fn log(args: &SyscallRegs) -> u64 {
    let Some(end) = args.arg0.checked_add(args.arg1) else {
        return 1;
    };

    let (Ok(start), Ok(end)) = (VirtAddr::try_new(args.arg0), VirtAddr::try_new(end)) else {
        return 1;
    };

    if !PageMap::current().is_mapped(start..end, PageTableFlags::USER_ACCESSIBLE) {
        return 2;
    }

    // TODO:
    // SAFETY: this is most likely unsafe
    let str: &[u8] = unsafe { core::slice::from_raw_parts(start.as_ptr(), args.arg1 as _) };

    let Ok(str) = core::str::from_utf8(str) else {
        return 3;
    };

    hyperion_log::print!("{str}");

    0
}

/// exit and kill the current process
///
/// # arguments
/// - syscall_id : 2
/// - arg0 : _exit code_
/// - arg1 : _ignored_
/// - arg2 : _ignored_
/// - arg3 : _ignored_
/// - arg4 : _ignored_
///
/// # return codes (in syscall_id after returning)
/// _won't return_
pub fn exit(_args: &SyscallRegs) -> ! {
    // TODO: exit code
    hyperion_scheduler::stop();
}

/// give the processor back to the kernel temporarily
///
/// # arguments
/// - syscall_id : 3
/// - arg0 : _ignored_
/// - arg1 : _ignored_
/// - arg2 : _ignored_
/// - arg3 : _ignored_
/// - arg4 : _ignored_
///
/// # return codes (in syscall_id after returning)
///  - 0 : ok
pub fn yield_now(_args: &SyscallRegs) -> u64 {
    hyperion_scheduler::yield_now();
    0
}

pub fn timestamp(args: &mut SyscallRegs) -> u64 {
    let nanos = HPET.nanos();

    /* let bytes = nanos.to_ne_bytes();
    args.arg0 = u64::from_ne_bytes(bytes[0..8].try_into().unwrap());
    args.arg1 = u64::from_ne_bytes(bytes[8..16].try_into().unwrap()); */
    args.arg0 = nanos as u64;
    args.arg1 = (nanos >> 64) as u64;

    0
}

pub fn nanosleep(args: &SyscallRegs) -> u64 {
    // hyperion_arch::clock;
    let now = HPET.nanos();
    let until = now + args.arg0 as u128;
    loop {
        hyperion_scheduler::yield_now();
        if HPET.nanos() >= until {
            break;
        }
    }
    0
}
