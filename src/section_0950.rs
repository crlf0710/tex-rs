//! @ The compressed trie will be packed into the |trie| array using a
//! ``top-down first-fit'' procedure. This is a little tricky, so the reader
//! should pay close attention: The |trie_hash| array is cleared to zero
//! again and renamed |trie_ref| for this phase of the operation; later on,
//! |trie_ref[p]| will be nonzero only if the linked trie node |p| is the
//! smallest character
//! in a family and if the characters |c| of that family have been allocated to
//! locations |trie_ref[p]+c| in the |trie| array. Locations of |trie| that
//! are in use will have |trie_link=0|, while the unused holes in |trie|
//! will be doubly linked with |trie_link| pointing to the next larger vacant
//! location and |trie_back| pointing to the next smaller one. This double
//! linking will have been carried out only as far as |trie_max|, where
//! |trie_max| is the largest index of |trie| that will be needed.
//! To save time at the low end of the trie, we maintain array entries
//! |trie_min[c]| pointing to the smallest hole that is greater than~|c|.
//! Another array |trie_taken| tells whether or not a given location is
//! equal to |trie_ref[p]| for some |p|; this array is used to ensure that
//! distinct nodes in the compressed trie will have distinct |trie_ref|
//! entries.
//
// @d trie_ref==trie_hash {where linked trie families go into |trie|}
// @d trie_back(#)==trie[#].lh {backward links in |trie| holes}
macro_rules! trie_back {
    ($globals:expr, $v:expr) => {
        $globals.trie[$v][crate::section_0113::TWO_HALVES_LH]
    }
}
// @<Glob...@>=
// @!init @!trie_taken:packed array[1..trie_size] of boolean;
//   {does a family start here?}
#[cfg(feature = "initex")]
/// does a family start here?
#[globals_struct_field(TeXGlobals)]
pub(crate) static trie_taken: trie_taken_array<boolean> = Default::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0950::trie_taken_array;

define_array_keyed_with_ranged_unsigned_integer_with_fixed_start_and_length!(
    pub(crate) trie_taken_array[u16_from_m_to_n<U1, trie_size_TYPENUM>] =>
    u16; U16; U1; trie_size_TYPENUM
);

// @t\hskip10pt@>@!trie_min:array[ASCII_code] of trie_pointer;
//   {the first possible slot for each character}
/// the first possible slot for each character
#[globals_struct_field(TeXGlobals)]
pub(crate) static trie_min: [trie_pointer; 256] = [Default::default(); 256];
// @t\hskip10pt@>@!trie_max:trie_pointer; {largest location used in |trie|}
#[cfg(feature = "initex")]
/// largest location used in `trie`
#[globals_struct_field(TeXGlobals)]
pub(crate) static trie_max: trie_pointer = trie_pointer::default();
// @t\hskip10pt@>@!trie_not_ready:boolean; {is the trie still in linked form?}
#[cfg(feature = "initex")]
/// is the trie still in linked form?
#[globals_struct_field(TeXGlobals)]
pub(crate) static trie_not_ready: boolean = true;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::boolean;

// tini
const _ : () = ();

use crate::pascal::u16_from_m_to_n;
use crate::section_0004::TeXGlobals;
use crate::section_0011::trie_size_TYPENUM;
use globals_struct::{globals_struct_field, globals_struct_use};
use typenum::U1;

