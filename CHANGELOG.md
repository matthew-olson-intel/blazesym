Unreleased
----------
- Switched to using `gimli` for DWARF support
  - Added support for DWARF versions v2, v3, and v5 for symbol lookup and source
    code information retrieval
  - Introduced `dwarf` feature to make dependency optional
- Switched from `log` to using `tracing` as the logging backend
  - Added spans to a couple of relevant call sites
- Added support for using DWARF information for symbol lookup (instead of just
  ELF symbols; so far DWARF was only used for mapping names to symbol
  information)
- Added support for normalizing and symbolizing addresses in an ELF file
  contained in an APK
- Adjusted `symbolize::Source::Gsym` variant to support symbolizing Gsym from
  user provided "raw" data
- Renamed `normalize::UserAddrMeta::Binary` variant to `Elf`


0.2.0-alpha.2
-------------
- Added `extern "C"` guards in `blazesym.h` header for easy of use from C++ code
- Added unused member variable to `blaze_user_addr_meta_unknown` type for
  compliance with C standard, stating undefined behavior for empty structs
- Changed `blaze_inspect_elf_src::path` type to `*const _`
- Fixed incorrect `NULL` checks when working with `blaze_symbolize_src_kernel`
  objects
- Switched away from using Git LFS for large benchmark files towards
  on-demand downloading from a different repository, controlled by
  `generate-bench-files` feature


0.2.0-alpha.1
-------------
- Removed no longer necessary `base_address` member from various types
- Renamed `SymInfo::address` member to `addr`
- Fixed incorrect allocation size calculation in C API
- Fixed file offset lookup potentially reporting subtly wrong offsets on
  certain ELF file segment constellations


0.2.0-alpha.0
-------------
- Initial documented release
