use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::path::Path;
use std::rc::Rc;

use crate::elf::ElfResolver;
use crate::inspect::FindAddrOpts;
use crate::inspect::SymInfo;
use crate::ksym::KSymResolver;
use crate::symbolize::AddrLineInfo;
use crate::Addr;
use crate::SymResolver;


pub(crate) struct KernelResolver {
    pub ksym_resolver: Option<Rc<KSymResolver>>,
    pub elf_resolver: Option<ElfResolver>,
}

impl KernelResolver {
    pub fn new(
        ksym_resolver: Option<Rc<KSymResolver>>,
        elf_resolver: Option<ElfResolver>,
    ) -> Result<KernelResolver> {
        if ksym_resolver.is_none() && elf_resolver.is_none() {
            return Err(Error::new(
                ErrorKind::NotFound,
                    "failed to create kernel resolver: neither ksym resolver nor kernel image ELF resolver are present",
            ))
        }

        Ok(KernelResolver {
            ksym_resolver,
            elf_resolver,
        })
    }
}

impl SymResolver for KernelResolver {
    fn find_syms(&self, addr: Addr) -> Result<Vec<(&str, Addr)>> {
        if let Some(ksym_resolver) = self.ksym_resolver.as_ref() {
            ksym_resolver.find_syms(addr)
        } else {
            self.elf_resolver.as_ref().unwrap().find_syms(addr)
        }
    }

    fn find_addr(&self, _name: &str, _opts: &FindAddrOpts) -> Result<Vec<SymInfo>> {
        Ok(Vec::new())
    }

    fn find_line_info(&self, addr: Addr) -> Result<Option<AddrLineInfo>> {
        if let Some(resolver) = self.elf_resolver.as_ref() {
            resolver.find_line_info(addr)
        } else {
            Ok(None)
        }
    }

    fn addr_file_off(&self, _addr: Addr) -> Option<u64> {
        None
    }
}

impl Debug for KernelResolver {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "KernelResolver {} {}",
            self.ksym_resolver
                .as_ref()
                .map(|resolver| resolver.file_name())
                .unwrap_or_else(|| Path::new(""))
                .display(),
            self.elf_resolver
                .as_ref()
                .map(|resolver| resolver.file_name())
                .unwrap_or_else(|| Path::new(""))
                .display(),
        )
    }
}
