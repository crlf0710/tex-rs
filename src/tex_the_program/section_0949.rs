//! @ A neat recursive procedure is now able to compress a trie by
//! traversing it and applying |trie_node| to its nodes in ``bottom up''
//! fashion. We will compress the entire trie by clearing |trie_hash| to
//! zero and then saying `|trie_root:=compress_trie(trie_root)|'.
//! @^recursion@>
//
// @<Declare procedures for preprocessing hyph...@>=
// function compress_trie(@!p:trie_pointer):trie_pointer;
pub(crate) fn compress_trie(globals: &mut TeXGlobals, p: trie_pointer) -> trie_pointer {
    // begin if p=0 then compress_trie:=0
    if p == 0 {
        return 0.into();
    }
    // else  begin trie_l[p]:=compress_trie(trie_l[p]);
    else {
        globals.trie_l[p] = compress_trie(globals, globals.trie_l[p]);
        // trie_r[p]:=compress_trie(trie_r[p]);
        globals.trie_r[p] = compress_trie(globals, globals.trie_r[p]);
        // compress_trie:=trie_node(p);
        return trie_node(globals, p);
        // end;
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0920::trie_pointer;
use crate::section_0948::trie_node;
