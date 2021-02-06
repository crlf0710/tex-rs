//! @ To pack the entire linked trie, we use the following recursive procedure.
//! @^recursion@>
//
// @<Declare procedures for preprocessing hyph...@>=
// procedure trie_pack(@!p:trie_pointer); {pack subtries of a family}
/// pack subtries of a family
#[allow(unused_variables)]
pub(crate) fn trie_pack(globals: &mut TeXGlobals, p: trie_pointer) {
    todo!();
    // var q:trie_pointer; {a local variable that need not be saved on recursive calls}
    // begin repeat q:=trie_l[p];
    // if (q>0)and(trie_ref[q]=0) then
    //   begin first_fit(q); trie_pack(q);
    //   end;
    // p:=trie_r[p];
    // until p=0;
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0920::trie_pointer;
