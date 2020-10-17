//! @* \[18] The hash table.
//! Control sequences are stored and retrieved by means of a fairly standard hash
//! table algorithm called the method of ``coalescing lists'' (cf.\ Algorithm 6.4C
//! in {\sl The Art of Computer Programming\/}). Once a control sequence enters the
//! table, it is never removed, because there are complicated situations
//! involving \.{\\gdef} where the removal of a control sequence at the end of
//! a group would be a mistake preventable only by the introduction of a
//! complicated reference-count mechanism.
//!
//! The actual sequence of letters forming a control sequence identifier is
//! stored in the |str_pool| array together with all the other strings. An
//! auxiliary array |hash| consists of items with two halfword fields per
//! word. The first of these, called |next(p)|, points to the next identifier
//! belonging to the same coalesced list as the identifier corresponding to~|p|;
//! and the other, called |text(p)|, points to the |str_start| entry for
//! |p|'s identifier. If position~|p| of the hash table is empty, we have
//! |text(p)=0|; if position |p| is either empty or the end of a coalesced
//! hash list, we have |next(p)=0|. An auxiliary pointer variable called
//! |hash_used| is maintained in such a way that all locations |p>=hash_used|
//! are nonempty. The global variable |cs_count| tells how many multiletter
//! control sequences have been defined, if statistics are being kept.
//!
//! A global boolean variable called |no_new_control_sequence| is set to
//! |true| during the time that new hash table entries are forbidden.
//
// @d next(#) == hash[#].lh {link for coalesced lists}
// @d text(#) == hash[#].rh {string number for control sequence name}
// @d hash_is_full == (hash_used=hash_base) {test if all positions are occupied}
// @d font_id_text(#) == text(font_id_base+#) {a frozen font identifier's name}
//
// @<Glob...@>=
// @!hash: array[hash_base..undefined_control_sequence-1] of two_halves;
//   {the hash table}
// @!hash_used:pointer; {allocation pointer for |hash|}
// @!no_new_control_sequence:boolean; {are new identifiers legal?}
/// are new identifiers legal?
#[globals_struct_field(TeXGlobals)]
pub(crate) static no_new_control_sequence: boolean = true;
// @!cs_count:integer; {total number of known identifiers}

#[globals_struct_use(TeXGlobals)]
use crate::pascal::boolean;

use globals_struct::{globals_struct_field, globals_struct_use};
