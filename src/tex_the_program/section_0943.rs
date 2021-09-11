//! @ Before we discuss trie building in detail, let's consider the simpler
//! problem of creating the |hyf_distance|, |hyf_num|, and |hyf_next| arrays.
//!
//! Suppose, for example, that \TeX\ reads the pattern `\.{ab2cde1}'. This is
//! a pattern of length 5, with $n_0\ldots n_5=0\,0\,2\,0\,0\,1$ in the
//! notation above. We want the corresponding |trie_op| code |v| to have
//! |hyf_distance[v]=3|, |hyf_num[v]=2|, and |hyf_next[v]=@t$v^\prime$@>|,
//! where the auxiliary |trie_op| code $v^\prime$ has
//! |hyf_distance[@t$v^\prime$@>]=0|, |hyf_num[@t$v^\prime$@>]=1|, and
//! |hyf_next[@t$v^\prime$@>]=min_quarterword|.
//!
//! \TeX\ computes an appropriate value |v| with the |new_trie_op| subroutine
//! below, by setting
//! $$\hbox{|@t$v^\prime$@>:=new_trie_op(0,1,min_quarterword)|,\qquad
//! |v:=new_trie_op(3,2,@t$v^\prime$@>)|.}$$
//! This subroutine looks up its three
//! parameters in a special hash table, assigning a new value only if these
//! three have not appeared before for the current language.
//!
//! The hash table is called |trie_op_hash|, and the number of entries it contains
//! is |trie_op_ptr|.
//
// @<Glob...@>=
// @!init @!trie_op_hash:array[-trie_op_size..trie_op_size] of 0..trie_op_size;
//   {trie op codes for quadruples}
/// trie op codes for quadruples
#[cfg(feature = "initex")]
#[globals_struct_field(TeXGlobals)]
pub(crate) static trie_op_hash: trie_op_hash_array<u16_from_0_to_n<trie_op_size_TYPENUM>> =
    trie_op_hash_array::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0943::trie_op_hash_array;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::u16_from_0_to_n;

#[globals_struct_use(TeXGlobals)]
use crate::section_0011::trie_op_size_TYPENUM;

pub type trie_op_hash_array_LENGTH_TYPENUM =
    typenum::op!(U1 + trie_op_size_TYPENUM + trie_op_size_TYPENUM);

define_array_keyed_with_ranged_signed_integer_with_fixed_start_and_length!(
    pub(crate) trie_op_hash_array[i16_from_m_to_n<trie_op_size_NEG_TYPENUM, trie_op_size_POS_TYPENUM>] =>
    i16; I16; U16; trie_op_size_NEG_TYPENUM; trie_op_hash_array_LENGTH_TYPENUM
);

// @!trie_used:array[ASCII_code] of quarterword;
//   {largest opcode used so far for this language}
#[cfg(feature = "initex")]
#[globals_struct_field(TeXGlobals)]
/// largest opcode used so far for this language
pub(crate) static trie_used: [quarterword; 256] = [quarterword::default(); 256];

// @!trie_op_lang:array[1..trie_op_size] of ASCII_code;
//   {language part of a hashed quadruple}
#[cfg(feature = "initex")]
#[globals_struct_field(TeXGlobals)]
/// language part of a hashed quadruple
pub(crate) static trie_op_lang: trie_op_val_array<ASCII_code> = trie_op_val_array::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0018::ASCII_code;

// @!trie_op_val:array[1..trie_op_size] of quarterword;
//   {opcode corresponding to a hashed quadruple}
#[cfg(feature = "initex")]
#[globals_struct_field(TeXGlobals)]
/// opcode corresponding to a hashed quadruple
pub(crate) static trie_op_val: trie_op_val_array<quarterword> = trie_op_val_array::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0943::trie_op_val_array;

#[globals_struct_use(TeXGlobals)]
use crate::section_0113::quarterword;

// @!trie_op_ptr:0..trie_op_size; {number of stored ops so far}
#[cfg(feature = "initex")]
#[globals_struct_field(TeXGlobals)]
/// number of stored ops so far
pub(crate) static trie_op_ptr: u16_from_0_to_n<trie_op_size_TYPENUM> = u16_from_0_to_n::default();
// tini

pub(crate) type trie_op_val_array_LENGTH_TYPENUM = typenum::op!(trie_op_size_TYPENUM - U1 + U1);

define_array_keyed_with_ranged_unsigned_integer_with_fixed_start_and_length!(
    pub(crate) trie_op_val_array[u16_from_m_to_n<U1, trie_op_size_TYPENUM>] =>
    u16; U16; U1; trie_op_val_array_LENGTH_TYPENUM
);

use crate::pascal::i16_from_m_to_n;
use crate::pascal::u16_from_m_to_n;
use crate::section_0011::trie_op_size_NEG_TYPENUM;
use crate::section_0011::trie_op_size_POS_TYPENUM;
use crate::section_0011::trie_op_size_TYPENUM;
use globals_struct::{globals_struct_field, globals_struct_use};
use typenum::U1;

pub(crate) macro trie_used($globals:expr, $sel:expr) {{
    let idx = $sel.numeric_value();
    if idx >= 256 {
        todo!(">= 256");
    }
    $globals.trie_used[idx as usize]
}}

pub(crate) macro trie_used_assign($globals:expr, $sel:expr, $val:expr) {{
    let idx = $sel.numeric_value();
    if idx >= 256 {
        todo!(">= 256");
    }
    $globals.trie_used[idx as usize] = $val;
}}
