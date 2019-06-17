use std::fs::File;
use std::io::{Error, Result};
use std::os::unix::io::{FromRawFd, RawFd};

#[cfg(not(target_os="redox"))]
pub fn slave_stdio(tty_path: &str) -> Result<(File, File, File)> {
    use libc::{self, O_CLOEXEC, O_RDONLY, O_WRONLY};
    use std::ffi::CString;

    let cvt = |res: i32| -> Result<i32> {
        if res < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(res)
        }
    };

    let tty_c = CString::new(tty_path).unwrap();
    let stdin = unsafe { File::from_raw_fd(
        cvt(libc::open(tty_c.as_ptr(), O_CLOEXEC | O_RDONLY))?
    ) };
    let stdout = unsafe { File::from_raw_fd(
        cvt(libc::open(tty_c.as_ptr(), O_CLOEXEC | O_WRONLY))?
    ) };
    let stderr = unsafe { File::from_raw_fd(
        cvt(libc::open(tty_c.as_ptr(), O_CLOEXEC | O_WRONLY))?
    ) };

    Ok((stdin, stdout, stderr))
}

#[cfg(target_os="redox")]
pub fn slave_stdio(tty_path: &str) -> Result<(File, File, File)> {
    use syscall::flag::{O_CLOEXEC, O_RDONLY, O_WRONLY};

    let stdin = unsafe { File::from_raw_fd(
        syscall::open(tty_path, O_CLOEXEC | O_RDONLY).map_err(|err| Error::from_raw_os_error(err.errno))? as RawFd
    ) };
    let stdout = unsafe { File::from_raw_fd(
        syscall::open(tty_path, O_CLOEXEC | O_WRONLY).map_err(|err| Error::from_raw_os_error(err.errno))? as RawFd
    ) };
    let stderr = unsafe { File::from_raw_fd(
        syscall::open(tty_path, O_CLOEXEC | O_WRONLY).map_err(|err| Error::from_raw_os_error(err.errno))? as RawFd
    ) };

    Ok((stdin, stdout, stderr))
}
