//! @* \[30] Font metric data.
//! \TeX\ gets its knowledge about fonts from font metric files, also called
//! \.{TFM} files; the `\.T' in `\.{TFM}' stands for \TeX,
//! but other programs know about them too.
//! @:TFM files}{\.{TFM} files@>
//! @^font metric files@>
//!
//! The information in a \.{TFM} file appears in a sequence of 8-bit bytes.
//! Since the number of bytes is always a multiple of 4, we could
//! also regard the file as a sequence of 32-bit words, but \TeX\ uses the
//! byte interpretation. The format of \.{TFM} files was designed by
//! Lyle Ramshaw in 1980. The intent is to convey a lot of different kinds
//! @^Ramshaw, Lyle Harold@>
//! of information in a compact but useful form.
//
// @<Glob...@>=
// @!tfm_file:byte_file;
#[globals_struct_field(TeXGlobals)]
pub(crate) static tfm_file: byte_file = byte_file::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0025::byte_file;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
