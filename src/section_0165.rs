//! @ If \TeX\ is extended improperly, the |mem| array might get screwed up.
//! For example, some pointers might be wrong, or some ``dead'' nodes might not
//! have been freed when the last reference to them disappeared. Procedures
//! |check_mem| and |search_mem| are available to help diagnose such
//! problems. These procedures make use of two arrays called |free| and
//! |was_free| that are present only if \TeX's debugging routines have
//! been included. (You may want to decrease the size of |mem| while you
//! @^debugging@>
//! are debugging.)
//
// @<Glob...@>=
// @!debug @!free: packed array [mem_min..mem_max] of boolean; {free cells}
/// free cells
#[cfg(feature = "debugging")]
#[globals_struct_field(TeXGlobals)]
pub(crate) static free: Box<mem_array<boolean>> = Box::new(mem_array::<boolean>::default_zeroed());
// @t\hskip10pt@>@!was_free: packed array [mem_min..mem_max] of boolean;
//   {previously free cells}

/// previously free cells
#[cfg(feature = "debugging")]
#[globals_struct_field(TeXGlobals)]
pub(crate) static was_free: Box<mem_array<boolean>> = Box::new(mem_array::<boolean>::default_zeroed());

#[globals_struct_use(TeXGlobals)]
use crate::section_0116::mem_array;
// @t\hskip10pt@>@!was_mem_end,@!was_lo_max,@!was_hi_min: pointer;
//   {previous |mem_end|, |lo_mem_max|, and |hi_mem_min|}

/// previous `mem_end`, `lo_mem_max`, and `hi_mem_min`
#[cfg(feature = "debugging")]
#[globals_struct_field(TeXGlobals)]
pub(crate) static was_mem_end: pointer = null;

#[cfg(feature = "debugging")]
#[globals_struct_field(TeXGlobals)]
pub(crate) static was_lo_max: pointer = null;

#[cfg(feature = "debugging")]
#[globals_struct_field(TeXGlobals)]
pub(crate) static was_hi_min: pointer = null;

// @t\hskip10pt@>@!panicking:boolean; {do we want to check memory constantly?}
/// do we want to check memory constantly?
#[cfg(feature = "debugging")]
#[globals_struct_field(TeXGlobals)]
pub(crate) static panicking: boolean = false;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::boolean;
// gubed

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
