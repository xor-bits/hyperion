use alloc::{boxed::Box, string::ToString, sync::Arc, vec::Vec};
use core::any::{type_name_of_val, Any};

use hyperion_arch::{syscall::SyscallRegs, vmm::PageMap};
use hyperion_drivers::acpi::hpet::HPET;
use hyperion_instant::Instant;
use hyperion_kernel_impl::VFS_ROOT;
use hyperion_log::*;
use hyperion_mem::vmm::PageMapImpl;
use hyperion_scheduler::{
    ipc::pipe::{Channel, Pipe, Receiver, Sender},
    lock::{Futex, Mutex},
    process, task,
    task::{AllocErr, FreeErr, Process, ProcessExt},
};
use hyperion_syscall::{
    err::{Error, Result},
    fs::FileOpenFlags,
    id,
    net::{Protocol, SocketDesc, SocketDomain, SocketType},
};
use hyperion_vfs::{
    device::FileDevice,
    error::{IoError, IoResult},
    path::Path,
    tree::{FileRef, Node},
};
use lock_api::ArcMutexGuard;
use time::Duration;
use x86_64::{structures::paging::PageTableFlags, VirtAddr};

//

pub fn syscall(args: &mut SyscallRegs) {
    let id = args.syscall_id;
    let (result, name) = match id as usize {
        id::LOG => call_id(log, args),
        id::EXIT => call_id(exit, args),
        id::YIELD_NOW => call_id(yield_now, args),
        id::TIMESTAMP => call_id(timestamp, args),
        id::NANOSLEEP => call_id(nanosleep, args),
        id::NANOSLEEP_UNTIL => call_id(nanosleep_until, args),
        id::PTHREAD_SPAWN => call_id(pthread_spawn, args),
        id::PALLOC => call_id(palloc, args),
        id::PFREE => call_id(pfree, args),
        id::SEND => call_id(send, args),
        id::RECV => call_id(recv, args),
        id::RENAME => call_id(rename, args),

        id::OPEN => call_id(open, args),
        id::CLOSE => call_id(close, args),
        id::READ => call_id(read, args),
        id::WRITE => call_id(write, args),

        id::SOCKET => call_id(socket, args),
        id::BIND => call_id(bind, args),
        id::LISTEN => call_id(listen, args),
        id::ACCEPT => call_id(accept, args),
        id::CONNECT => call_id(connect, args),

        id::GET_PID => call_id(get_pid, args),
        id::GET_TID => call_id(get_tid, args),

        _ => {
            debug!("invalid syscall");
            hyperion_scheduler::stop();
        }
    };

    _ = (result, name);
    // if result < 0 {
    //     debug!("syscall `{name}` (id {id}) returned {result}",);
    // }
}

fn call_id(
    f: impl FnOnce(&mut SyscallRegs) -> Result<usize>,
    args: &mut SyscallRegs,
) -> (Result<usize>, &str) {
    let name = type_name_of_val(&f);

    // debug!(
    //     "{name}<{}>({}, {}, {}, {}, {})",
    //     args.syscall_id, args.arg0, args.arg1, args.arg2, args.arg3, args.arg4,
    // );

    let res = f(args);
    args.syscall_id = Error::encode(res) as u64;
    (res, name)
}

/// print a string to logs
///
/// [`hyperion_syscall::log`]
pub fn log(args: &mut SyscallRegs) -> Result<usize> {
    let str = read_untrusted_str(args.arg0, args.arg1)?;
    hyperion_log::print!("{str}");
    return Ok(0);
}

/// exit and kill the current process
///
/// [`hyperion_syscall::exit`]
pub fn exit(_args: &mut SyscallRegs) -> Result<usize> {
    // TODO: exit code
    hyperion_scheduler::stop();
}

/// give the processor back to the kernel temporarily
///
/// [`hyperion_syscall::yield_now`]
pub fn yield_now(_args: &mut SyscallRegs) -> Result<usize> {
    hyperion_scheduler::yield_now();
    return Ok(0);
}

/// get the number of nanoseconds after boot
///
/// [`hyperion_syscall::timestamp`]
pub fn timestamp(args: &mut SyscallRegs) -> Result<usize> {
    let nanos = HPET.nanos();

    let bytes = read_untrusted_bytes_mut(args.arg0, 16)?;
    bytes.copy_from_slice(&nanos.to_ne_bytes());

    return Ok(0);
}

/// sleep at least arg0 nanoseconds
///
/// [`hyperion_syscall::nanosleep`]
pub fn nanosleep(args: &mut SyscallRegs) -> Result<usize> {
    hyperion_scheduler::sleep(Duration::nanoseconds((args.arg0 as i64).max(0)));
    return Ok(0);
}

/// sleep at least until the nanosecond arg0 happens
///
/// [`hyperion_syscall::nanosleep_until`]
pub fn nanosleep_until(args: &mut SyscallRegs) -> Result<usize> {
    hyperion_scheduler::sleep_until(Instant::new(args.arg0 as u128));
    return Ok(0);
}

/// spawn a new thread
///
/// thread entry signature: `extern "C" fn thread_entry(stack_ptr: usize, arg1: usize) -> !`
///
/// [`hyperion_syscall::pthread_spawn`]
pub fn pthread_spawn(args: &mut SyscallRegs) -> Result<usize> {
    hyperion_scheduler::spawn_userspace(args.arg0, args.arg1);
    return Ok(0);
}

/// allocate physical pages and map them to virtual memory
///
/// returns the virtual address pointer
///
/// [`hyperion_syscall::palloc`]
pub fn palloc(args: &mut SyscallRegs) -> Result<usize> {
    let n_pages = args.arg0 as usize;
    let flags =
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;

    match process().alloc(n_pages, flags) {
        Ok(ptr) => Ok(ptr as _),
        Err(AllocErr::OutOfVirtMem) => Err(Error::OUT_OF_VIRTUAL_MEMORY),
    }
}

/// free allocated physical pages
///
/// [`hyperion_syscall::pfree`]
pub fn pfree(args: &mut SyscallRegs) -> Result<usize> {
    let Ok(ptr) = VirtAddr::try_new(args.arg0) else {
        return Err(Error::INVALID_ADDRESS);
    };
    let n_pages = args.arg1 as usize;

    match process().free(n_pages, ptr) {
        Ok(()) => Ok(0),
        Err(FreeErr::InvalidAddr) => Err(Error::INVALID_ADDRESS),
        Err(FreeErr::InvalidAlloc) => Err(Error::INVALID_ALLOC),
    }
}

/// rename the current process
///
/// [`hyperion_syscall::rename`]
pub fn rename(args: &mut SyscallRegs) -> Result<usize> {
    let new_name = read_untrusted_str(args.arg0, args.arg1)?;
    hyperion_scheduler::rename(new_name.to_string());
    return Ok(0);
}

/// open a file
///
/// [`hyperion_syscall::open`]
pub fn open(args: &mut SyscallRegs) -> Result<usize> {
    let path = read_untrusted_str(args.arg0, args.arg1)?;

    let Some(flags) = FileOpenFlags::from_bits(args.arg2 as usize) else {
        return Err(Error::INVALID_FLAGS);
    };

    let this = process();
    let ext = process_ext_with(&this);

    let create = flags.contains(FileOpenFlags::CREATE) || flags.contains(FileOpenFlags::CREATE_NEW);

    if flags.contains(FileOpenFlags::CREATE_NEW)
        || flags.contains(FileOpenFlags::TRUNC)
        || flags.contains(FileOpenFlags::APPEND)
        || (!flags.contains(FileOpenFlags::READ) && !flags.contains(FileOpenFlags::WRITE))
    {
        return Err(Error::FILESYSTEM_ERROR);
    }

    let mkdirs = true; // TODO: tmp

    let file_ref = VFS_ROOT
        .find_file(path, mkdirs, create)
        .map_err(map_vfs_err_to_syscall_err)?;

    let fd = ext.files.lock().push(File {
        file_ref,
        position: 0,
    });

    return Ok(fd);
}

/// close a file
///
/// [`hyperion_syscall::close`]
pub fn close(args: &mut SyscallRegs) -> Result<usize> {
    let this = process();
    let ext = process_ext_with(&this);

    if ext.files.lock().remove(args.arg0 as _).is_none() {
        return Err(Error::BAD_FILE_DESCRIPTOR);
    }

    return Ok(0);
}

/// read bytes from a file
///
/// [`hyperion_syscall::read`]
pub fn read(args: &mut SyscallRegs) -> Result<usize> {
    let buf = read_untrusted_bytes_mut(args.arg1, args.arg2)?;

    let this = process();
    let ext = process_ext_with(&this);

    let mut files = ext.files.lock();

    let file = files
        .get_mut(args.arg0 as _)
        .ok_or(Error::BAD_FILE_DESCRIPTOR)?;

    let read = file
        .file_ref
        .lock()
        .read(file.position, buf)
        .map_err(map_vfs_err_to_syscall_err)?;
    file.position += read;

    return Ok(read);
}

/// write bytes into a file
///
/// [`hyperion_syscall::write`]
pub fn write(args: &mut SyscallRegs) -> Result<usize> {
    let buf = read_untrusted_bytes(args.arg1, args.arg2)?;

    let this = process();
    let ext = process_ext_with(&this);

    let mut files = ext.files.lock();

    let file = files
        .get_mut(args.arg0 as _)
        .ok_or(Error::BAD_FILE_DESCRIPTOR)?;

    let written = file
        .file_ref
        .lock()
        .write(file.position, buf)
        .map_err(map_vfs_err_to_syscall_err)?;
    file.position += written;

    return Ok(written);
}

/// create a socket
///
/// [`hyperion_syscall::socket`]
fn socket(args: &mut SyscallRegs) -> Result<usize> {
    let domain = SocketDomain(args.arg0 as _);
    let ty = SocketType(args.arg1 as _);
    let proto = Protocol(args.arg2 as _);

    _socket(domain, ty, proto).map(|fd| fd.0)
}

fn _socket(domain: SocketDomain, ty: SocketType, proto: Protocol) -> Result<SocketDesc> {
    if domain != SocketDomain::LOCAL {
        return Err(Error::INVALID_DOMAIN);
    }

    if ty != SocketType::STREAM {
        return Err(Error::INVALID_TYPE);
    }

    if proto != Protocol::LOCAL {
        return Err(Error::UNKNOWN_PROTOCOL);
    }

    Ok(_socket_from(SocketFile {
        domain,
        ty,
        proto,
        incoming: None,
        connection: None,
    }))
}

fn _socket_from(socket: SocketFile) -> SocketDesc {
    let this = process();
    let ext = process_ext_with(&this);

    let socket = Socket {
        socket_ref: Arc::new(Mutex::new(socket)),
    };

    let fd = ext.sockets.lock().push(socket);

    return SocketDesc(fd);
}

/// bind a socket
///
/// [`hyperion_syscall::bind`]
fn bind(args: &mut SyscallRegs) -> Result<usize> {
    let socket = SocketDesc(args.arg0 as _);
    let addr = read_untrusted_str(args.arg1, args.arg2)?;

    _bind(socket, addr).map(|_| 0)
}

fn _bind(socket: SocketDesc, addr: &str) -> Result<()> {
    // TODO: this is only for LOCAL domain sockets atm
    let path = Path::from_str(addr);
    let Some((dir, sock_file)) = path.split() else {
        return Err(Error::NOT_FOUND);
    };

    let socket = get_socket(socket)?;

    VFS_ROOT
        // find the directory node
        .find_dir(dir, false)
        .map_err(map_vfs_err_to_syscall_err)?
        // lock the directory
        .lock_arc()
        // create the socket file in that directory
        .create_node(sock_file, Node::File(socket))
        .map_err(map_vfs_err_to_syscall_err)?;

    return Ok(());
}

/// start listening to connections on a socket
///
/// [`hyperion_syscall::listen`]
fn listen(args: &mut SyscallRegs) -> Result<usize> {
    let socket = SocketDesc(args.arg0 as _);
    _listen(socket).map(|_| 0)
}

fn _listen(socket: SocketDesc) -> Result<()> {
    get_socket_locked(socket)?.incoming();

    Ok(())
}

/// accept a connection on a socket
///
/// [`hyperion_syscall::accept`]
fn accept(args: &mut SyscallRegs) -> Result<usize> {
    let socket = SocketDesc(args.arg0 as _);

    _accept(socket).map(|fd| fd.0)
}

fn _accept(socket: SocketDesc) -> Result<SocketDesc> {
    let mut socket = get_socket_locked(socket)?;

    let domain = socket.domain;
    let ty = socket.ty;
    let proto = socket.proto;

    // `listen` syscall is not required
    let conn = socket.incoming();

    drop(socket);

    // blocks here
    let conn = conn.recv().unwrap();

    Ok(_socket_from(SocketFile {
        domain,
        ty,
        proto,
        incoming: None,
        connection: Some(conn),
    }))
}

/// connect to a socket
///
/// [`hyperion_syscall::connect`]
fn connect(args: &mut SyscallRegs) -> Result<usize> {
    let socket = SocketDesc(args.arg0 as _);
    let addr = read_untrusted_str(args.arg1, args.arg2)?;

    _connect(socket, addr).map(|_| 0)
}

fn _connect(socket: SocketDesc, addr: &str) -> Result<()> {
    // get the client socket early to test for errors, but lock it late
    let client = get_socket(socket)?;

    let server = VFS_ROOT
        .find_file(addr, false, false)
        .map_err(map_vfs_err_to_syscall_err)?
        .lock_arc();

    // TODO: inode
    let incoming = server
        .as_any()
        .downcast_ref::<SocketFile>()
        .ok_or(Error::CONNECTION_REFUSED)?
        .try_incoming()
        .ok_or(Error::CONNECTION_REFUSED)?;

    drop(server);

    let (conn_client, conn_server) = LocalSocketConn::new();
    client.lock().connection = Some(conn_client);
    incoming
        .send(conn_server)
        .map_err(|_| Error::CONNECTION_REFUSED)?;

    Ok(())
}

/// send data to a socket
///
/// [`hyperion_syscall::send`]
pub fn send(args: &mut SyscallRegs) -> Result<usize> {
    let socket = SocketDesc(args.arg0 as _);
    let data = read_untrusted_bytes(args.arg1, args.arg2)?;
    let flags = args.arg3 as _;

    _send(socket, data, flags).map(|_| 0)
}

fn _send(socket: SocketDesc, data: &[u8], _flags: usize) -> Result<()> {
    let socket = get_socket_locked(socket)?;

    let Some(conn) = socket.try_connection() else {
        return Err(Error::BAD_FILE_DESCRIPTOR);
    };

    drop(socket);

    conn.send.send_slice(data).map_err(|_| Error::CLOSED)?;

    return Ok(());
}

/// recv data from a socket
///
/// [`hyperion_syscall::recv`]
pub fn recv(args: &mut SyscallRegs) -> Result<usize> {
    let socket = SocketDesc(args.arg0 as _);
    let buf = read_untrusted_bytes_mut(args.arg1, args.arg2)?;
    let flags = args.arg3 as _;

    _recv(socket, buf, flags)
}

fn _recv(socket: SocketDesc, buf: &mut [u8], _flags: usize) -> Result<usize> {
    let socket = get_socket_locked(socket)?;

    let Some(conn) = socket.try_connection() else {
        return Err(Error::BAD_FILE_DESCRIPTOR);
    };

    drop(socket);

    let n_bytes = conn.recv.recv_slice(buf).map_err(|_| Error::CLOSED)?;

    return Ok(n_bytes);
}

/// pid of the current process
///
/// [`hyperion_syscall::get_pid`]
pub fn get_pid(_args: &mut SyscallRegs) -> Result<usize> {
    Ok(process().pid.num())
}

/// tid of the current thread
///
/// [`hyperion_syscall::get_tid`]
pub fn get_tid(_args: &mut SyscallRegs) -> Result<usize> {
    Ok(task().tid.num())
}

//

struct SparseVec<T> {
    inner: Vec<Option<T>>,
}

impl<T> SparseVec<T> {
    pub const fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index).and_then(Option::as_ref)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.inner.get_mut(index).and_then(Option::as_mut)
    }

    pub fn push(&mut self, v: T) -> usize {
        let index;
        if let Some((_index, spot)) = self
            .inner
            .iter_mut()
            .enumerate()
            .find(|(_, spot)| spot.is_none())
        {
            index = _index;
            *spot = Some(v);
        } else {
            index = self.inner.len();
            self.inner.push(Some(v));
        }

        index
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        self.inner.get_mut(index).and_then(Option::take)
    }
}

struct ProcessExtra {
    files: Mutex<SparseVec<File>>,
    sockets: Mutex<SparseVec<Socket>>,
}

struct File {
    file_ref: FileRef<Futex>,
    position: usize,
}

struct Socket {
    socket_ref: SocketRef,
}

type SocketRef = Arc<Mutex<SocketFile>>;

struct SocketFile {
    domain: SocketDomain,
    ty: SocketType,
    proto: Protocol,

    incoming: Option<Arc<Channel<LocalSocketConn>>>,
    connection: Option<LocalSocketConn>,
}

impl SocketFile {
    pub fn incoming(&mut self) -> Arc<Channel<LocalSocketConn>> {
        self.incoming
            .get_or_insert_with(|| Arc::new(Channel::new(16)))
            .clone()
    }

    pub fn try_incoming(&self) -> Option<Arc<Channel<LocalSocketConn>>> {
        self.incoming.as_ref().cloned()
    }

    pub fn try_connection(&self) -> Option<LocalSocketConn> {
        self.connection.as_ref().cloned()
    }
}

#[derive(Clone)]
struct LocalSocketConn {
    send: Sender<u8>,
    recv: Receiver<u8>,
}

impl LocalSocketConn {
    pub fn new() -> (Self, Self) {
        let pipe_0 = Pipe::new(0x1000);
        let pipe_1 = Pipe::new(0x1000);
        let (send_0, recv_1) = pipe_0.split();
        let (send_1, recv_0) = pipe_1.split();
        (
            Self {
                send: send_0,
                recv: recv_0,
            },
            Self {
                send: send_1,
                recv: recv_1,
            },
        )
    }
}

impl FileDevice for SocketFile {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn len(&self) -> usize {
        0
    }

    fn read(&self, _offset: usize, _buf: &mut [u8]) -> IoResult<usize> {
        Err(IoError::PermissionDenied)
    }

    fn write(&mut self, _offset: usize, _buf: &[u8]) -> IoResult<usize> {
        Err(IoError::PermissionDenied)
    }
}

impl ProcessExt for ProcessExtra {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

//

fn get_socket_locked(socket: SocketDesc) -> Result<ArcMutexGuard<Futex, SocketFile>> {
    get_socket(socket).map(|v| v.lock_arc())
}

fn get_socket(socket: SocketDesc) -> Result<Arc<Mutex<SocketFile>>> {
    let this = process();
    let ext = process_ext_with(&this);

    let socket = ext
        .sockets
        .lock()
        .get(socket.0)
        .ok_or(Error::BAD_FILE_DESCRIPTOR)?
        .socket_ref
        .clone();

    Ok(socket)
}

// fn get_file(file: FileDesc) -> Result {
//     let this = process();
//     let ext = process_ext_with(&this);

//     let mut files = ext.files.lock();

//     let file = files.get_mut(file.0).ok_or(Error::BAD_FILE_DESCRIPTOR)?;
// }

fn process_ext_with(proc: &Process) -> &ProcessExtra {
    proc.ext
        .call_once(|| {
            Box::new(ProcessExtra {
                files: Mutex::new(SparseVec::new()),
                sockets: Mutex::new(SparseVec::new()),
            })
        })
        .as_any()
        .downcast_ref()
        .unwrap()
}

fn map_vfs_err_to_syscall_err(err: IoError) -> Error {
    match err {
        IoError::NotFound => Error::NOT_FOUND,
        IoError::AlreadyExists => Error::ALREADY_EXISTS,
        IoError::NotADirectory => Error::NOT_A_DIRECTORY,
        IoError::IsADirectory => Error::NOT_A_FILE,
        IoError::FilesystemError => Error::FILESYSTEM_ERROR,
        IoError::PermissionDenied => Error::PERMISSION_DENIED,
        IoError::UnexpectedEOF => Error::UNEXPECTED_EOF,
        IoError::Interrupted => Error::INTERRUPTED,
        IoError::WriteZero => Error::WRITE_ZERO,
    }
}

fn read_slice_parts(ptr: u64, len: u64) -> Result<(VirtAddr, usize)> {
    if len == 0 {
        return Ok((VirtAddr::new_truncate(0), 0));
    }

    let Some(end) = ptr.checked_add(len) else {
        return Err(Error::INVALID_ADDRESS);
    };

    let (Ok(start), Ok(end)) = (VirtAddr::try_new(ptr), VirtAddr::try_new(end)) else {
        return Err(Error::INVALID_ADDRESS);
    };

    if !PageMap::current().is_mapped(start..end, PageTableFlags::USER_ACCESSIBLE) {
        // debug!("{:?} not mapped", start..end);
        return Err(Error::INVALID_ADDRESS);
    }

    Ok((start, len as _))
}

fn read_untrusted_bytes<'a>(ptr: u64, len: u64) -> Result<&'a [u8]> {
    read_slice_parts(ptr, len).map(|(start, len)| {
        // TODO:
        // SAFETY: this is most likely unsafe
        if len == 0 {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(start.as_ptr(), len as _) }
        }
    })
}

fn read_untrusted_bytes_mut<'a>(ptr: u64, len: u64) -> Result<&'a mut [u8]> {
    read_slice_parts(ptr, len).map(|(start, len)| {
        // TODO:
        // SAFETY: this is most likely unsafe
        if len == 0 {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(start.as_mut_ptr(), len as _) }
        }
    })
}

fn read_untrusted_str<'a>(ptr: u64, len: u64) -> Result<&'a str> {
    read_untrusted_bytes(ptr, len)
        .and_then(|bytes| core::str::from_utf8(bytes).map_err(|_| Error::INVALID_UTF8))
}
