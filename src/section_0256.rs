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
/// link for coalesced lists
macro_rules! next {
    ($globals:expr, $p:expr) => {
        $globals.hash[$p][crate::section_0113::TWO_HALVES_LH]
    };
}
// @d text(#) == hash[#].rh {string number for control sequence name}
/// string number for control sequence name
macro_rules! text {
    ($globals:expr, $p:expr) => {
        $globals.hash[$p][crate::section_0113::TWO_HALVES_RH]
    };
}
// @d hash_is_full == (hash_used=hash_base) {test if all positions are occupied}
/// test if all positions are occupied
macro_rules! hash_is_full {
    ($globals:expr) => {
        $globals.hash_used == hash_base as pointer
    };
}
// @d font_id_text(#) == text(font_id_base+#) {a frozen font identifier's name}
/// a frozen font identifier's name
macro_rules! font_id_text {
    ($globals:expr, $ptr:expr) => {
        text!(
            $globals,
            crate::section_0222::font_id_base as crate::section_0115::pointer
                + $ptr as crate::section_0115::pointer
        )
    };
}

//
// @<Glob...@>=
// @!hash: array[hash_base..undefined_control_sequence-1] of two_halves;
//   {the hash table}
/// the hash table
#[globals_struct_field(TeXGlobals)]
pub(crate) static hash: hash_array<two_halves> = hash_array::default();

type hash_array_last_index_TYPENUM = typenum::op!(undefined_control_sequence_TYPENUM - U1);
type hash_array_LENGTH_TYPENUM =
    typenum::op!(hash_array_last_index_TYPENUM - hash_base_TYPENUM + U1);

define_array_keyed_with_ranged_unsigned_integer_with_fixed_start_and_length!(
    pub(crate) hash_array[u16_from_m_to_n<hash_base_TYPENUM, hash_array_last_index_TYPENUM>] =>
    u16; U16; hash_base_TYPENUM; hash_array_LENGTH_TYPENUM
);

#[globals_struct_use(TeXGlobals)]
use crate::section_0256::hash_array;

#[globals_struct_use(TeXGlobals)]
use crate::section_0113::two_halves;

// @!hash_used:pointer; {allocation pointer for |hash|}
/// allocation pointer for |hash|
#[globals_struct_field(TeXGlobals)]
pub(crate) static hash_used: pointer = null;
// @!no_new_control_sequence:boolean; {are new identifiers legal?}
/// are new identifiers legal?
#[globals_struct_field(TeXGlobals)]
pub(crate) static no_new_control_sequence: boolean = true;
// @!cs_count:integer; {total number of known identifiers}
/// total number of known identifiers
#[globals_struct_field(TeXGlobals)]
pub(crate) static cs_count: integer = 0;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::boolean;

use crate::pascal::u16_from_m_to_n;
use crate::section_0222::hash_base_TYPENUM;
use crate::section_0222::undefined_control_sequence_TYPENUM;
use globals_struct::{globals_struct_field, globals_struct_use};
use typenum::U1;
