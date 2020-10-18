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
// @t\hskip10pt@>@!was_free: packed array [mem_min..mem_max] of boolean;
//   {previously free cells}
// @t\hskip10pt@>@!was_mem_end,@!was_lo_max,@!was_hi_min: pointer;
//   {previous |mem_end|, |lo_mem_max|, and |hi_mem_min|}
// @t\hskip10pt@>@!panicking:boolean; {do we want to check memory constantly?}
/// do we want to check memory constantly?
#[cfg(feature = "debugging")]
#[globals_struct_field(TeXGlobals)]
pub(crate) static panicking: boolean = false;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::boolean;
// gubed
//

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
