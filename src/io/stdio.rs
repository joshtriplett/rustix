//! Functions returning the stdio file descriptors.
//!
//! # Safety
//!
//! These access the file descriptors by absolute index value, and nothing
//! prevents them from being closed and reused. They should only be used in
//! `main` or other situations where one is in control of the process'
//! stdio streams.
#![allow(unsafe_code)]

use crate::backend;
use crate::fd::OwnedFd;
use backend::fd::{BorrowedFd, FromRawFd, RawFd};

#[cfg(not(any(windows, target_os = "wasi")))]
use crate::io;
#[cfg(not(any(windows, target_os = "wasi")))]
use backend::fd::AsFd;

/// `STDIN_FILENO`—Standard input, borrowed.
///
/// In `std`-using configurations, this is a safe function, because the
/// standard library already assumes that the stdin file descriptor is always
/// valid. In `no_std` configurations, it is `unsafe`.
///
/// # Warning
///
/// This function allows reading directly from stdin without coordinating
/// with the buffering performed by [`std::io::Stdin`], so it could cause
/// corrupted input.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [FreeBSD]
///  - [NetBSD]
///  - [OpenBSD]
///  - [DragonFly BSD]
///  - [illumos]
///  - [glibc]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stdin.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stdin.3.html
/// [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=stdin&sektion=4
/// [NetBSD]: https://man.netbsd.org/stdin.4
/// [OpenBSD]: https://man.openbsd.org/stdin.4
/// [DragonFly BSD]: https://man.dragonflybsd.org/?command=stdin&section=4
/// [illumos]: https://illumos.org/man/4FS/stdin
/// [glibc]: https://www.gnu.org/software/libc/manual/html_node/Standard-Streams.html#index-stdin
#[cfg(feature = "std")]
#[doc(alias = "STDIN_FILENO")]
#[inline]
pub const fn stdin() -> BorrowedFd<'static> {
    // SAFETY: When "std" is enabled, the standard library assumes that the stdio
    // file descriptors are all valid.
    unsafe { BorrowedFd::borrow_raw(backend::io::types::STDIN_FILENO as RawFd) }
}

/// `STDIN_FILENO`—Standard input, borrowed.
///
/// In `std`-using configurations, this is a safe function, because the
/// standard library already assumes that the stdin file descriptor is always
/// valid. In `no_std` configurations, it is `unsafe`.
///
/// # Safety
///
/// In `no_std` configurations, the stdin file descriptor can be closed,
/// potentially on other threads, in which case the file descriptor index
/// value could be dynamically reused for other purposes, potentially on
/// different threads.
///
/// # Warning
///
/// This function allows reading directly from stdin without coordinating
/// with the buffering performed by [`std::io::Stdin`], so it could cause
/// corrupted input.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [FreeBSD]
///  - [NetBSD]
///  - [OpenBSD]
///  - [DragonFly BSD]
///  - [illumos]
///  - [glibc]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stdin.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stdin.3.html
/// [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=stdin&sektion=4
/// [NetBSD]: https://man.netbsd.org/stdin.4
/// [OpenBSD]: https://man.openbsd.org/stdin.4
/// [DragonFly BSD]: https://man.dragonflybsd.org/?command=stdin&section=4
/// [illumos]: https://illumos.org/man/4FS/stdin
/// [glibc]: https://www.gnu.org/software/libc/manual/html_node/Standard-Streams.html#index-stdin
#[cfg(not(feature = "std"))]
#[doc(alias = "STDIN_FILENO")]
#[inline]
pub const unsafe fn stdin() -> BorrowedFd<'static> {
    BorrowedFd::borrow_raw(backend::io::types::STDIN_FILENO as RawFd)
}

/// `STDIN_FILENO`—Standard input, owned.
///
/// This is similar to [`stdin`], however it returns an `OwnedFd` which closes
/// standard input when it is dropped.
///
/// # Safety
///
/// Safe `std`-using Rust code is permitted to assume that the stdin file
/// descriptor is always valid. This function returns an `OwnedFd` which will
/// close the stdin file descriptor when dropped.
///
/// # Warning
///
/// This has the same hazards as [`stdin`].
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [FreeBSD]
///  - [NetBSD]
///  - [OpenBSD]
///  - [DragonFly BSD]
///  - [illumos]
///  - [glibc]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stdin.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stdin.3.html
/// [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=stdin&sektion=4
/// [NetBSD]: https://man.netbsd.org/stdin.4
/// [OpenBSD]: https://man.openbsd.org/stdin.4
/// [DragonFly BSD]: https://man.dragonflybsd.org/?command=stdin&section=4
/// [illumos]: https://illumos.org/man/4FS/stdin
/// [glibc]: https://www.gnu.org/software/libc/manual/html_node/Standard-Streams.html#index-stdin
#[doc(alias = "STDIN_FILENO")]
#[inline]
pub unsafe fn take_stdin() -> OwnedFd {
    backend::fd::OwnedFd::from_raw_fd(backend::io::types::STDIN_FILENO as RawFd)
}

/// `STDOUT_FILENO`—Standard output, borrowed.
///
/// In `std`-using configurations, this is a safe function, because the
/// standard library already assumes that the stdout file descriptor is always
/// valid. In `no_std` configurations, it is `unsafe`.
///
/// # Warning
///
/// This function allows reading directly from stdout without coordinating
/// with the buffering performed by [`std::io::Stdout`], so it could cause
/// corrupted input.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [FreeBSD]
///  - [NetBSD]
///  - [OpenBSD]
///  - [DragonFly BSD]
///  - [illumos]
///  - [glibc]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stdout.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stdout.3.html
/// [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=stdout&sektion=4
/// [NetBSD]: https://man.netbsd.org/stdout.4
/// [OpenBSD]: https://man.openbsd.org/stdout.4
/// [DragonFly BSD]: https://man.dragonflybsd.org/?command=stdout&section=4
/// [illumos]: https://illumos.org/man/4FS/stdout
/// [glibc]: https://www.gnu.org/software/libc/manual/html_node/Standard-Streams.html#index-stdout
#[cfg(feature = "std")]
#[doc(alias = "STDOUT_FILENO")]
#[inline]
pub const fn stdout() -> BorrowedFd<'static> {
    // SAFETY: When "std" is enabled, the standard library assumes that the stdio
    // file descriptors are all valid.
    unsafe { BorrowedFd::borrow_raw(backend::io::types::STDOUT_FILENO as RawFd) }
}

/// `STDOUT_FILENO`—Standard output, borrowed.
///
/// In `std`-using configurations, this is a safe function, because the
/// standard library already assumes that the stdout file descriptor is always
/// valid. In `no_std` configurations, it is `unsafe`.
///
/// # Safety
///
/// In `no_std` configurations, the stdout file descriptor can be closed,
/// potentially on other threads, in which case the file descriptor index
/// value could be dynamically reused for other purposes, potentially on
/// different threads.
///
/// # Warning
///
/// This function allows reading directly from stdout without coordinating
/// with the buffering performed by [`std::io::Stdout`], so it could cause
/// corrupted input.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [FreeBSD]
///  - [NetBSD]
///  - [OpenBSD]
///  - [DragonFly BSD]
///  - [illumos]
///  - [glibc]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stdout.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stdout.3.html
/// [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=stdout&sektion=4
/// [NetBSD]: https://man.netbsd.org/stdout.4
/// [OpenBSD]: https://man.openbsd.org/stdout.4
/// [DragonFly BSD]: https://man.dragonflybsd.org/?command=stdout&section=4
/// [illumos]: https://illumos.org/man/4FS/stdout
/// [glibc]: https://www.gnu.org/software/libc/manual/html_node/Standard-Streams.html#index-stdout
#[cfg(not(feature = "std"))]
#[doc(alias = "STDOUT_FILENO")]
#[inline]
pub const unsafe fn stdout() -> BorrowedFd<'static> {
    BorrowedFd::borrow_raw(backend::io::types::STDOUT_FILENO as RawFd)
}

/// `STDOUT_FILENO`—Standard output, owned.
///
/// This is similar to [`stdout`], however it returns an `OwnedFd` which closes
/// standard output when it is dropped.
///
/// # Safety
///
/// Safe `std`-using Rust code is permitted to assume that the stdout file
/// descriptor is always valid. This function returns an `OwnedFd` which will
/// close the stdout file descriptor when dropped.
///
/// # Warning
///
/// This has the same hazards as [`stdout`].
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [FreeBSD]
///  - [NetBSD]
///  - [OpenBSD]
///  - [DragonFly BSD]
///  - [illumos]
///  - [glibc]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stdout.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stdout.3.html
/// [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=stdout&sektion=4
/// [NetBSD]: https://man.netbsd.org/stdout.4
/// [OpenBSD]: https://man.openbsd.org/stdout.4
/// [DragonFly BSD]: https://man.dragonflybsd.org/?command=stdout&section=4
/// [illumos]: https://illumos.org/man/4FS/stdout
/// [glibc]: https://www.gnu.org/software/libc/manual/html_node/Standard-Streams.html#index-stdout
#[doc(alias = "STDOUT_FILENO")]
#[inline]
pub unsafe fn take_stdout() -> OwnedFd {
    backend::fd::OwnedFd::from_raw_fd(backend::io::types::STDOUT_FILENO as RawFd)
}

/// `STDERR_FILENO`—Standard error, borrowed.
///
/// In `std`-using configurations, this is a safe function, because the
/// standard library already assumes that the stderr file descriptor is always
/// valid. In `no_std` configurations, it is `unsafe`.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [FreeBSD]
///  - [NetBSD]
///  - [OpenBSD]
///  - [DragonFly BSD]
///  - [illumos]
///  - [glibc]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stderr.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stderr.3.html
/// [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=stderr&sektion=4
/// [NetBSD]: https://man.netbsd.org/stderr.4
/// [OpenBSD]: https://man.openbsd.org/stderr.4
/// [DragonFly BSD]: https://man.dragonflybsd.org/?command=stderr&section=4
/// [illumos]: https://illumos.org/man/4FS/stderr
/// [glibc]: https://www.gnu.org/software/libc/manual/html_node/Standard-Streams.html#index-stderr
#[cfg(feature = "std")]
#[doc(alias = "STDERR_FILENO")]
#[inline]
pub const fn stderr() -> BorrowedFd<'static> {
    // SAFETY: When "std" is enabled, the standard library assumes that the stdio
    // file descriptors are all valid.
    unsafe { BorrowedFd::borrow_raw(backend::io::types::STDERR_FILENO as RawFd) }
}

/// `STDERR_FILENO`—Standard error, borrowed.
///
/// In `std`-using configurations, this is a safe function, because the
/// standard library already assumes that the stderr file descriptor is always
/// valid. In `no_std` configurations, it is `unsafe`.
///
/// # Safety
///
/// In `no_std` configurations, the stderr file descriptor can be closed,
/// potentially on other threads, in which case the file descriptor index
/// value could be dynamically reused for other purposes, potentially on
/// different threads.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [FreeBSD]
///  - [NetBSD]
///  - [OpenBSD]
///  - [DragonFly BSD]
///  - [illumos]
///  - [glibc]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stderr.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stderr.3.html
/// [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=stderr&sektion=4
/// [NetBSD]: https://man.netbsd.org/stderr.4
/// [OpenBSD]: https://man.openbsd.org/stderr.4
/// [DragonFly BSD]: https://man.dragonflybsd.org/?command=stderr&section=4
/// [illumos]: https://illumos.org/man/4FS/stderr
/// [glibc]: https://www.gnu.org/software/libc/manual/html_node/Standard-Streams.html#index-stderr
#[cfg(not(feature = "std"))]
#[doc(alias = "STDERR_FILENO")]
#[inline]
pub const unsafe fn stderr() -> BorrowedFd<'static> {
    BorrowedFd::borrow_raw(backend::io::types::STDERR_FILENO as RawFd)
}

/// `STDERR_FILENO`—Standard error, owned.
///
/// This is similar to [`stderr`], however it returns an `OwnedFd` which closes
/// standard output when it is dropped.
///
/// # Safety
///
/// Safe std-using Rust code is permitted to assume that the stderr file
/// descriptor is always valid. This function returns an `OwnedFd` which will
/// close the stderr file descriptor when dropped.
///
/// # Other hazards
///
/// This has the same hazards as [`stderr`].
///
/// And, when the `OwnedFd` is dropped, subsequent newly created file
/// descriptors may unknowingly reuse the stderr file descriptor number, which
/// may break common assumptions, so it should typically only be dropped at the
/// end of a program when no more file descriptors will be created.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [FreeBSD]
///  - [NetBSD]
///  - [OpenBSD]
///  - [DragonFly BSD]
///  - [illumos]
///  - [glibc]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stderr.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stderr.3.html
/// [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=stderr&sektion=4
/// [NetBSD]: https://man.netbsd.org/stderr.4
/// [OpenBSD]: https://man.openbsd.org/stderr.4
/// [DragonFly BSD]: https://man.dragonflybsd.org/?command=stderr&section=4
/// [illumos]: https://illumos.org/man/4FS/stderr
/// [glibc]: https://www.gnu.org/software/libc/manual/html_node/Standard-Streams.html#index-stderr
#[doc(alias = "STDERR_FILENO")]
#[inline]
pub unsafe fn take_stderr() -> OwnedFd {
    backend::fd::OwnedFd::from_raw_fd(backend::io::types::STDERR_FILENO as RawFd)
}

/// `STDIN_FILENO`—Standard input, raw.
///
/// This is similar to [`stdin`], however it returns a `RawFd`.
///
/// # Other hazards
///
/// This has the same hazards as [`stdin`].
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [FreeBSD]
///  - [NetBSD]
///  - [OpenBSD]
///  - [DragonFly BSD]
///  - [illumos]
///  - [glibc]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stdin.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stdin.3.html
/// [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=stdin&sektion=4
/// [NetBSD]: https://man.netbsd.org/stdin.4
/// [OpenBSD]: https://man.openbsd.org/stdin.4
/// [DragonFly BSD]: https://man.dragonflybsd.org/?command=stdin&section=4
/// [illumos]: https://illumos.org/man/4FS/stdin
/// [glibc]: https://www.gnu.org/software/libc/manual/html_node/Standard-Streams.html#index-stdin
#[doc(alias = "STDIN_FILENO")]
#[inline]
pub const fn raw_stdin() -> RawFd {
    backend::io::types::STDIN_FILENO as RawFd
}

/// `STDOUT_FILENO`—Standard output, raw.
///
/// This is similar to [`stdout`], however it returns a `RawFd`.
///
/// # Other hazards
///
/// This has the same hazards as [`stdout`].
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [FreeBSD]
///  - [NetBSD]
///  - [OpenBSD]
///  - [DragonFly BSD]
///  - [illumos]
///  - [glibc]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stdout.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stdout.3.html
/// [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=stdout&sektion=4
/// [NetBSD]: https://man.netbsd.org/stdout.4
/// [OpenBSD]: https://man.openbsd.org/stdout.4
/// [DragonFly BSD]: https://man.dragonflybsd.org/?command=stdout&section=4
/// [illumos]: https://illumos.org/man/4FS/stdout
/// [glibc]: https://www.gnu.org/software/libc/manual/html_node/Standard-Streams.html#index-stdout
#[doc(alias = "STDOUT_FILENO")]
#[inline]
pub const fn raw_stdout() -> RawFd {
    backend::io::types::STDOUT_FILENO as RawFd
}

/// `STDERR_FILENO`—Standard error, raw.
///
/// This is similar to [`stderr`], however it returns a `RawFd`.
///
/// # Other hazards
///
/// This has the same hazards as [`stderr`].
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [FreeBSD]
///  - [NetBSD]
///  - [OpenBSD]
///  - [DragonFly BSD]
///  - [illumos]
///  - [glibc]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stderr.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stderr.3.html
/// [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=stderr&sektion=4
/// [NetBSD]: https://man.netbsd.org/stderr.4
/// [OpenBSD]: https://man.openbsd.org/stderr.4
/// [DragonFly BSD]: https://man.dragonflybsd.org/?command=stderr&section=4
/// [illumos]: https://illumos.org/man/4FS/stderr
/// [glibc]: https://www.gnu.org/software/libc/manual/html_node/Standard-Streams.html#index-stderr
#[doc(alias = "STDERR_FILENO")]
#[inline]
pub const fn raw_stderr() -> RawFd {
    backend::io::types::STDERR_FILENO as RawFd
}

/// Utility function to safely `dup2` over stdin (fd 0).
#[cfg(not(any(windows, target_os = "wasi")))]
#[inline]
pub fn dup2_stdin<Fd: AsFd>(fd: Fd) -> io::Result<()> {
    // SAFETY: We pass the returned `OwnedFd` to `forget` so that it isn't
    // dropped.
    let mut target = unsafe { io::take_stdin() };
    backend::io::syscalls::dup2(fd.as_fd(), &mut target)?;
    core::mem::forget(target);
    Ok(())
}

/// Utility function to safely `dup2` over stdout (fd 1).
#[cfg(not(any(windows, target_os = "wasi")))]
#[inline]
pub fn dup2_stdout<Fd: AsFd>(fd: Fd) -> io::Result<()> {
    // SAFETY: We pass the returned `OwnedFd` to `forget` so that it isn't
    // dropped.
    let mut target = unsafe { io::take_stdout() };
    backend::io::syscalls::dup2(fd.as_fd(), &mut target)?;
    core::mem::forget(target);
    Ok(())
}

/// Utility function to safely `dup2` over stderr (fd 2).
#[cfg(not(any(windows, target_os = "wasi")))]
#[inline]
pub fn dup2_stderr<Fd: AsFd>(fd: Fd) -> io::Result<()> {
    // SAFETY: We pass the returned `OwnedFd` to `forget` so that it isn't
    // dropped.
    let mut target = unsafe { io::take_stderr() };
    backend::io::syscalls::dup2(fd.as_fd(), &mut target)?;
    core::mem::forget(target);
    Ok(())
}
