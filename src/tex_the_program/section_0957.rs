//! @ To pack the entire linked trie, we use the following recursive procedure.
//! @^recursion@>
//
// @<Declare procedures for preprocessing hyph...@>=
// procedure trie_pack(@!p:trie_pointer); {pack subtries of a family}
/// pack subtries of a family
#[allow(unused_variables)]
pub(crate) fn trie_pack(globals: &mut TeXGlobals, mut p: trie_pointer) -> TeXResult<()> {
    // var q:trie_pointer; {a local variable that need not be saved on recursive calls}
    /// a local variable that need not be saved on recursive calls
    let mut q: trie_pointer;
    // begin repeat q:=trie_l[p];
    loop {
        q = globals.trie_l[p];
        // if (q>0)and(trie_ref[q]=0) then
        if q > 0 && trie_ref!(globals, q) == 0 {
            // begin first_fit(q); trie_pack(q);
            first_fit(globals, q)?;
            trie_pack(globals, q)?;
            // end;
        }
        // p:=trie_r[p];
        p = globals.trie_r[p];
        // until p=0;
        if p == 0 {
            break;
        }
    }
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0920::trie_pointer;
use crate::section_0950::trie_ref;
use crate::section_0953::first_fit;
