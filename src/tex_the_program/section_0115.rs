//! @* \[9] Dynamic memory allocation.
//! The \TeX\ system does nearly all of its own memory allocation, so that it
//! can readily be transported into environments that do not have automatic
//! facilities for strings, garbage collection, etc., and so that it can be in
//! control of what error messages the user receives. The dynamic storage
//! requirements of \TeX\ are handled by providing a large array |mem| in
//! which consecutive blocks of words are used as nodes by the \TeX\ routines.
//!
//! Pointer variables are indices into this array, or into another array
//! called |eqtb| that will be explained later. A pointer variable might
//! also be a special flag that lies outside the bounds of |mem|, so we
//! allow pointers to assume any |halfword| value. The minimum halfword
//! value represents a null pointer. \TeX\ does not assume that |mem[null]| exists.

// @d pointer==halfword {a flag or a location in |mem| or |eqtb|}
/// a flag or a location in `mem` or `eqtb`
pub(crate) type pointer = halfword;
// @d null==min_halfword {the null pointer}
pub(crate) const null: pointer = min_halfword;

// @<Glob...@>=

// @!temp_ptr:pointer; {a pointer variable for occasional emergency use}
#[globals_struct_field(TeXGlobals)]
/// a pointer variable for occasional emergency use
pub(crate) static temp_ptr: pointer = null;

#[globals_struct_use(TeXGlobals)]
use crate::section_0115::pointer;

use crate::section_0110::min_halfword;
use crate::section_0113::halfword;
use globals_struct::{globals_struct_field, globals_struct_use};
