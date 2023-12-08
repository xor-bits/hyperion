#![no_std]
#![no_main]

//

use core::str::from_utf8;

use libstd::{
    alloc::format,
    fs::{File, OpenOptions, Stdin},
    println,
    sys::{
        err::Result,
        fs::FileDesc,
        net::{Protocol, SocketDesc, SocketDomain, SocketType},
        *,
    },
    thread::spawn,
    CliArgs,
};

//

fn run_server() -> Result<()> {
    let server = socket(SocketDomain::LOCAL, SocketType::STREAM, Protocol::LOCAL)?;
    bind(server, "/dev/server.sock")?;

    close(Stdin::FD).unwrap();

    rename("local server")?;

    let mut i = 0usize;
    loop {
        let conn = accept(server)?;
        i += 1;

        spawn(move || _ = handle_client(i, conn));
    }
}

fn handle_client(i: usize, conn: SocketDesc) -> Result<()> {
    let msg = format!("Hello {i}");

    send(conn, msg.as_bytes(), 0)?;

    let mut buf = [0u8; 64];
    let len = recv(conn, &mut buf, 0)?;
    assert_eq!(&buf[..len], b"ack");

    let file = OpenOptions::new()
        .read(true)
        .create(false)
        .open(format!("/tmp/{msg}"))?;

    let mut buf = [0u8; 64];
    let len = file.read(&mut buf)?;
    assert_eq!(&buf[..len], b"testing data");

    Ok(())
}

fn run_client() -> Result<()> {
    let client = socket(SocketDomain::LOCAL, SocketType::STREAM, Protocol::LOCAL)?;
    connect(client, "/dev/server.sock")?;

    rename("local client")?;

    println!("connected");

    loop {
        let mut buf = [0u8; 64];
        let len = recv(client, &mut buf, 0)?;

        let utf8 = &buf[..len];
        let msg = from_utf8(utf8).unwrap();
        println!("got `{msg:?}`");

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(format!("/tmp/{msg}"))?;

        file.write(b"testing data")?;

        drop(file); // drop = flush + close

        // if buf[..len].ends_with(b"2") {
        //     println!("infinite loop");
        //     loop {
        //         spin_loop();
        //     }
        // }

        send(client, b"ack", 0)?;
    }
}

fn _test_duplicate_stdin() -> File {
    let dup = dup(Stdin::FD, FileDesc(10)).unwrap();
    let stdin_dupe = unsafe { File::new(dup) };
    close(Stdin::FD).unwrap();
    stdin_dupe
}

fn _test_userspace_mutex() {
    let counter = libstd::alloc::sync::Arc::new(libstd::sync::Mutex::new(0usize));
    for _ in 0..100 {
        let counter = counter.clone();
        spawn(move || {
            *counter.lock() += 1;
        });
    }

    loop {
        if *counter.lock() == 100 {
            break;
        }

        yield_now();
    }

    println!("complete");
}

#[no_mangle]
pub fn main(_args: CliArgs) {
    // _test_userspace_mutex();

    // let mut stdin = _test_duplicate_stdin();

    // let mut reader = BufReader::new(&mut stdin);
    // // let mut reader = BufReader::new(&STDIN);
    // let mut buf = String::new();
    // loop {
    //     buf.clear();
    //     let len = reader.read_line(&mut buf).unwrap();

    //     if len == 0 {
    //         break;
    //     }

    //     let stdout = &STDOUT;
    //     stdout.write(buf.as_bytes()).unwrap();
    // }

    println!("PID:{} TID:{}", get_pid(), get_tid());

    if run_server().is_err() {
        if let Err(err) = run_client() {
            println!("error: {err}")
        };
    }
}