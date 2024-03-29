use core::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Error(pub usize);

//

macro_rules! impl_error {
    (
        $(pub const $var:ident: $str:literal = $id:literal;)+
        pub const _: $unknown_str:literal = _;
    ) => {
        impl Error {
            $(pub const $var: Self = Self($id);)+

            #[must_use]
            pub const fn as_str(self) -> &'static str {
                match self.0 {
                    $($id => $str,)+
                    _ => $unknown_str,
                }
            }
        }

    };
}

impl_error! {
    pub const INVALID_ADDRESS: "invalid address" = 1;
    pub const NO_SUCH_PROCESS: "no such process" = 2;
    pub const OUT_OF_VIRTUAL_MEMORY: "out of virtual memory" = 3;
    pub const OUT_OF_MEMORY: "out of memory" = 4;
    pub const INVALID_ALLOC: "invalid alloc" = 5;
    pub const INVALID_UTF8: "invalid utf8" = 6;

    pub const NOT_FOUND: "not found" = 7;
    pub const ALREADY_EXISTS: "already exists" = 8;
    pub const NOT_A_DIRECTORY: "not a directory" = 9;
    pub const NOT_A_FILE: "not a file" = 10;
    pub const FILESYSTEM_ERROR: "internal filesystem error" = 11;
    pub const PERMISSION_DENIED: "permission denied" = 12;
    pub const UNEXPECTED_EOF: "unexpected end of file" = 13;
    pub const INTERRUPTED: "interrupted" = 14;
    pub const WRITE_ZERO: "wrote nothing" = 15;
    pub const BAD_FILE_DESCRIPTOR: "bad file descriptor" = 16;

    pub const INVALID_FLAGS: "invalid flags" = 17;

    pub const INVALID_DOMAIN: "invalid socket domain" = 18;
    pub const INVALID_TYPE: "invalid socket type" = 19;
    pub const UNKNOWN_PROTOCOL: "unknown protocol" = 20;

    pub const CONNECTION_REFUSED: "connection refused" = 21;
    pub const CLOSED: "stream closed" = 22;

    pub const INVALID_ARGUMENT: "invalid argument" = 23;

    pub const IS_A_PIPE: "file descriptor is a pipe/socket" = 24;
    pub const NOT_A_SOCKET: "file descriptor is not a socket" = 25;

    pub const _: "unknown error" = _;
}

impl Error {
    pub const fn decode(result: usize) -> Result<usize> {
        let error = -(result as isize); // errors in RAX are saved as negatives

        if error > 0 {
            Err(Error(error as usize))
        } else {
            Ok(result)
        }
    }

    #[must_use]
    pub const fn encode(v: Result<usize>) -> usize {
        match v {
            Ok(v) => v,
            Err(Error(err)) => (-(err as isize)) as usize,
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl core::error::Error for Error {}

// impl From<Error> for anyhow::Error {
//     fn from(value: Error) -> Self {
//         anyhow::Error::msg(value)
//     }
// }
