//! ` `

// @d trie_link(#)==trie[#].rh {``downward'' link in a trie}
// @d trie_char(#)==trie[#].b1 {character matched at this trie location}
// @d trie_op(#)==trie[#].b0 {program for hyphenation at this trie location}
//
// @<Glob...@>=
// @!trie:array[trie_pointer] of two_halves; {|trie_link|, |trie_char|, |trie_op|}
// @!hyf_distance:array[1..trie_op_size] of small_number; {position |k-j| of $n_j$}
/// position `k-j` of `n_j`
#[globals_struct_field(TeXGlobals)]
pub(crate) static hyf_distance: trie_op_val_array<small_number> = trie_op_val_array::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0943::trie_op_val_array;

#[globals_struct_use(TeXGlobals)]
use crate::section_0101::small_number;

// @!hyf_num:array[1..trie_op_size] of small_number; {value of $n_j$}
/// value of `n_j`
#[globals_struct_field(TeXGlobals)]
pub(crate) static hyf_num: trie_op_val_array<small_number> = trie_op_val_array::default();

// @!hyf_next:array[1..trie_op_size] of quarterword; {continuation code}
/// continuation code
#[globals_struct_field(TeXGlobals)]
pub(crate) static hyf_next: trie_op_val_array<quarterword> = trie_op_val_array::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0113::quarterword;

// @!op_start:array[ASCII_code] of 0..trie_op_size; {offset for current language}

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
