//! @ The |mem| array is divided into two regions that are allocated separately,
//! but the dividing line between these two regions is not fixed; they grow
//! together until finding their ``natural'' size in a particular job.
//! Locations less than or equal to |lo_mem_max| are used for storing
//! variable-length records consisting of two or more words each. This region
//! is maintained using an algorithm similar to the one described in exercise
//! 2.5--19 of {\sl The Art of Computer Programming}. However, no size field
//! appears in the allocated nodes; the program is responsible for knowing the
//! relevant size when a node is freed. Locations greater than or equal to
//! |hi_mem_min| are used for storing one-word records; a conventional
//! \.{AVAIL} stack is used for allocation in this region.
//!
//! Locations of |mem| between |mem_bot| and |mem_top| may be dumped as part
//! of preloaded format files, by the \.{INITEX} preprocessor.
//! @.INITEX@>
//! Production versions of \TeX\ may extend the memory at both ends in order to
//! provide more space; locations between |mem_min| and |mem_bot| are always
//! used for variable-size nodes, and locations between |mem_top| and |mem_max|
//! are always used for single-word nodes.
//!
//! The key pointers that govern |mem| allocation have a prescribed order:
//! $$\advance\thickmuskip-2mu
//! \hbox{|null<=mem_min<=mem_bot<lo_mem_max<
//!   hi_mem_min<mem_top<=mem_end<=mem_max|.}$$
//!
//! Empirical tests show that the present implementation of \TeX\ tends to
//! spend about 9\pct! of its running time allocating nodes, and about 6\pct!
//! deallocating them after their use.
//
// @<Glob...@>=
// @!mem : array[mem_min..mem_max] of memory_word; {the big dynamic storage area}
/// the big dynamic storage area
#[globals_struct_field(TeXGlobals)]
pub(crate) static mem: mem_array<memory_word> = mem_array::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0116::mem_array;

#[globals_struct_use(TeXGlobals)]
use crate::section_0113::memory_word;

type mem_array_LENGTH_TYPENUM = typenum::op!(mem_max_TYPENUM - mem_min_TYPENUM + U1);

define_array_keyed_with_ranged_unsigned_integer_with_fixed_start_and_length!(
    pub(crate) mem_array[u32_from_m_to_n<mem_min_TYPENUM, mem_max_TYPENUM>] =>
    u32; U32; mem_min_TYPENUM; mem_array_LENGTH_TYPENUM
);

// @!lo_mem_max : pointer; {the largest location of variable-size memory in use}
// @!hi_mem_min : pointer; {the smallest location of one-word memory in use}
//

use typenum::U1;
use crate::section_0011::mem_min_TYPENUM;
use crate::section_0011::mem_max_TYPENUM;
use crate::pascal::u32_from_m_to_n;
use globals_struct::{globals_struct_field, globals_struct_use};
