use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::path::PathBuf;

use crate::Pid;

#[cfg(doc)]
use super::Symbolizer;


/// A single ELF file.
#[derive(Clone, Debug)]
pub struct Elf {
    /// The name of ELF file.
    ///
    /// It can be an executable or shared object.
    /// For example, passing `"/bin/sh"` will load symbols and debug information from `sh`.
    /// Whereas passing `"/lib/libc.so.xxx"` will load symbols and debug information from the libc.
    pub path: PathBuf,
    /// The struct is non-exhaustive and open to extension.
    #[doc(hidden)]
    pub(crate) _non_exhaustive: (),
}

impl Elf {
    /// Create a new [`Elf`] object, referencing the provided path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            _non_exhaustive: (),
        }
    }
}

impl From<Elf> for Source<'static> {
    fn from(elf: Elf) -> Self {
        Source::Elf(elf)
    }
}


/// Linux Kernel's binary image and a copy of /proc/kallsyms
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Kernel {
    /// The path of a kallsyms copy.
    ///
    /// For the running kernel on the device, it can be
    /// "/proc/kallsyms".  However, you can make a copy for later.
    /// In that situation, you should give the path of the
    /// copy.  Passing `None`, by default, will be
    /// `"/proc/kallsyms"`.
    pub kallsyms: Option<PathBuf>,
    /// The path of a kernel image.
    ///
    /// This should be the path of a kernel image.  For example,
    /// `"/boot/vmlinux-xxxx"`.  A `None` value will find the
    /// kernel image of the running kernel in `"/boot/"` or
    /// `"/usr/lib/debug/boot/"`.
    pub kernel_image: Option<PathBuf>,
    /// The struct is non-exhaustive and open to extension.
    #[doc(hidden)]
    pub(crate) _non_exhaustive: (),
}

impl From<Kernel> for Source<'static> {
    fn from(kernel: Kernel) -> Self {
        Source::Kernel(kernel)
    }
}


/// Configuration for process based address symbolization.
///
/// The corresponding addresses supplied to [`Symbolizer::symbolize`] are
/// expected to be absolute addresses as valid within the process identified
/// by the [`pid`][Process::pid] member.
#[derive(Clone)]
pub struct Process {
    /// The referenced process' ID.
    pub pid: Pid,
    /// The struct is non-exhaustive and open to extension.
    #[doc(hidden)]
    pub(crate) _non_exhaustive: (),
}

impl Process {
    /// Create a new [`Process`] object using the provided `pid`.
    pub fn new(pid: Pid) -> Self {
        Self {
            pid,
            _non_exhaustive: (),
        }
    }
}

impl Debug for Process {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Process {
            pid,
            _non_exhaustive: (),
        } = self;

        f.debug_tuple(stringify!(Process))
            // We use the `Display` representation here.
            .field(&format_args!("{pid}"))
            .finish()
    }
}

impl From<Process> for Source<'static> {
    fn from(process: Process) -> Self {
        Source::Process(process)
    }
}


#[derive(Clone, Debug)]
pub enum Gsym<'dat> {
    /// "Raw" Gsym data.
    Data(GsymData<'dat>),
    /// A Gsym file.
    File(GsymFile),
}

/// Gsym data.
#[derive(Clone, Debug)]
pub struct GsymData<'dat> {
    /// The "raw" Gsym data.
    pub data: &'dat [u8],
    /// The struct is non-exhaustive and open to extension.
    #[doc(hidden)]
    pub(crate) _non_exhaustive: (),
}

impl<'dat> GsymData<'dat> {
    /// Create a new [`GsymData`] object, referencing the provided path.
    pub fn new(data: &'dat [u8]) -> Self {
        Self {
            data,
            _non_exhaustive: (),
        }
    }
}

impl<'dat> From<GsymData<'dat>> for Source<'dat> {
    fn from(gsym: GsymData<'dat>) -> Self {
        Source::Gsym(Gsym::Data(gsym))
    }
}


/// A Gsym file.
#[derive(Clone, Debug)]
pub struct GsymFile {
    /// The path to the Gsym file.
    pub path: PathBuf,
    /// The struct is non-exhaustive and open to extension.
    #[doc(hidden)]
    pub(crate) _non_exhaustive: (),
}

impl GsymFile {
    /// Create a new [`GsymFile`] object, referencing the provided path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            _non_exhaustive: (),
        }
    }
}

impl From<GsymFile> for Source<'static> {
    fn from(gsym: GsymFile) -> Self {
        Source::Gsym(Gsym::File(gsym))
    }
}


/// The description of a source of symbols and debug information.
///
/// The source of symbols and debug information can be an ELF file, kernel
/// image, or process.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Source<'dat> {
    /// A single ELF file
    Elf(Elf),
    /// Information about the Linux kernel.
    Kernel(Kernel),
    /// Information about a process.
    Process(Process),
    /// A Gsym file.
    Gsym(Gsym<'dat>),
}


#[cfg(test)]
mod tests {
    use super::*;


    /// Check that the `Debug` representation of [`Entry`] is as expected.
    #[test]
    fn process_debug() {
        let process = Process::new(Pid::Slf);
        assert_eq!(format!("{process:?}"), "Process(self)");

        let process = Process::new(Pid::from(1234));
        assert_eq!(format!("{process:?}"), "Process(1234)");
    }
}
