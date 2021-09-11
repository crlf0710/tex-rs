//! @ In order to isolate the system-dependent aspects of file names, the
//! @^system dependencies@>
//! system-independent parts of \TeX\ are expressed in terms
//! of three system-dependent
//! procedures called |begin_name|, |more_name|, and |end_name|. In
//! essence, if the user-specified characters of the file name are $c_1\ldots c_n$,
//! the system-independent driver program does the operations
//! $$|begin_name|;\,|more_name|(c_1);\,\ldots\,;\,|more_name|(c_n);
//! \,|end_name|.$$
//! These three procedures communicate with each other via global variables.
//! Afterwards the file name will appear in the string pool as three strings
//! called |cur_name|\penalty10000\hskip-.05em,
//! |cur_area|, and |cur_ext|; the latter two are null (i.e.,
//! |""|), unless they were explicitly specified by the user.
//!
//! Actually the situation is slightly more complicated, because \TeX\ needs
//! to know when the file name ends. The |more_name| routine is a function
//! (with side effects) that returns |true| on the calls |more_name|$(c_1)$,
//! \dots, |more_name|$(c_{n-1})$. The final call |more_name|$(c_n)$
//! returns |false|; or, it returns |true| and the token following $c_n$ is
//! something like `\.{\\hbox}' (i.e., not a character). In other words,
//! |more_name| is supposed to return |true| unless it is sure that the
//! file name has been completely scanned; and |end_name| is supposed to be able
//! to finish the assembly of |cur_name|, |cur_area|, and |cur_ext| regardless of
//! whether $|more_name|(c_n)$ returned |true| or |false|.
//
// @<Glob...@>=
// @!cur_name:str_number; {name of file just scanned}
/// name of file just scanned
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_name: str_number = str_number::zero();
// @!cur_area:str_number; {file area just scanned, or \.{""}}
/// file area just scanned, or `""`
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_area: str_number = str_number::zero();
// @!cur_ext:str_number; {file extension just scanned, or \.{""}}
/// file extension just scanned, or `""`
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_ext: str_number = str_number::zero();

#[globals_struct_use(TeXGlobals)]
use crate::section_0038::str_number;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
