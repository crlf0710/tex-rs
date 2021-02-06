//! @ The linked trie that is used to preprocess hyphenation patterns appears
//! in several global arrays. Each node represents an instruction of the form
//! ``if you see character |c|, then perform operation |o|, move to the
//! next character, and go to node |l|; otherwise go to node |r|.''
//! The four quantities |c|, |o|, |l|, and |r| are stored in four arrays
//! |trie_c|, |trie_o|, |trie_l|, and |trie_r|. The root of the trie
//! is |trie_l[0]|, and the number of nodes is |trie_ptr|. Null trie
//! pointers are represented by zero. To initialize the trie, we simply
//! set |trie_l[0]| and |trie_ptr| to zero. We also set |trie_c[0]| to some
//! arbitrary value, since the algorithm may access it.
//!
//! The algorithms maintain the condition
//! $$\hbox{|trie_c[trie_r[z]]>trie_c[z]|\qquad
//! whenever |z<>0| and |trie_r[z]<>0|};$$ in other words, sibling nodes are
//! ordered by their |c| fields.
//
// @d trie_root==trie_l[0] {root of the linked trie}
/// root of the linked trie
macro_rules! trie_root {
    ($globals:expr) => {
        $globals.trie_l[0]
    }
}
// @<Glob...@>=
// @!init @!trie_c:packed array[trie_pointer] of packed_ASCII_code;
//   {characters to match}
/// characters to match
#[cfg(feature = "initex")]
#[globals_struct_field(TeXGlobals)]
pub(crate) static trie_c: Box<trie_pointer_array<ASCII_code>> = Box::new(trie_pointer_array::default());
// @t\hskip10pt@>@!trie_o:packed array[trie_pointer] of quarterword;
//   {operations to perform}
/// operations to perform
#[cfg(feature = "initex")]
#[globals_struct_field(TeXGlobals)]
pub(crate) static trie_o: Box<trie_pointer_array<quarterword>> = Box::new(trie_pointer_array::default());
// @t\hskip10pt@>@!trie_l:packed array[trie_pointer] of trie_pointer;
//   {left subtrie links}
/// left subtrie links
#[cfg(feature = "initex")]
#[globals_struct_field(TeXGlobals)]
pub(crate) static trie_l: Box<trie_pointer_array<trie_pointer>> = Box::new(trie_pointer_array::default());
// @t\hskip10pt@>@!trie_r:packed array[trie_pointer] of trie_pointer;
//   {right subtrie links}
/// right subtrie links
#[cfg(feature = "initex")]
#[globals_struct_field(TeXGlobals)]
pub(crate) static trie_r: Box<trie_pointer_array<trie_pointer>> = Box::new(trie_pointer_array::default());
// @t\hskip10pt@>@!trie_ptr:trie_pointer; {the number of nodes in the trie}
/// the number of nodes in the trie
#[cfg(feature = "initex")]
#[globals_struct_field(TeXGlobals)]
pub(crate) static trie_ptr: trie_pointer = trie_pointer::default();
// @t\hskip10pt@>@!trie_hash:packed array[trie_pointer] of trie_pointer;
//   {used to identify equivalent subtries}
// tini

#[globals_struct_use(TeXGlobals)]
use crate::section_0018::ASCII_code;

#[globals_struct_use(TeXGlobals)]
use crate::section_0113::quarterword;

#[globals_struct_use(TeXGlobals)]
use crate::section_0920::trie_pointer_array;

#[globals_struct_use(TeXGlobals)]
use crate::section_0920::trie_pointer;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};

macro_rules! trie_c {
    ($globals:expr, $p:expr) => {
        $globals.trie_c[$p]
    };
}

macro_rules! trie_c_assign {
    ($globals:expr, $p:expr, $c:expr) => {
        $globals.trie_c[$p] = $c
    };
}
