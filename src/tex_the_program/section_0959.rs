//! @ The fixing-up procedure is, of course, recursive. Since the linked trie
//! usually has overlapping subtries, the same data may be moved several
//! times; but that causes no harm, and at most as much work is done as it
//! took to build the uncompressed trie.
//! @^recursion@>
//
// @<Declare procedures for preprocessing hyph...@>=
// procedure trie_fix(@!p:trie_pointer); {moves |p| and its siblings into |trie|}
/// moves `p` and its siblings into `trie`
pub(crate) fn trie_fix(globals: &mut TeXGlobals, mut p: trie_pointer) {
    // var q:trie_pointer; {a local variable that need not be saved on recursive calls}
    /// a local variable that need not be saved on recursive calls
    let mut q: trie_pointer;
    // @!c:ASCII_code; {another one that need not be saved}
    /// another one that need not be saved
    let mut c: ASCII_code;
    // @!z:trie_pointer; {|trie| reference; this local variable must be saved}
    /// `trie` reference; this local variable must be saved
    let z: trie_pointer;
    // begin z:=trie_ref[p];
    z = trie_ref!(globals, p);
    // repeat q:=trie_l[p]; c:=so(trie_c[p]);
    loop {
        q = globals.trie_l[p];
        c = globals.trie_c[p];
        let c_u8 = if c.numeric_value() < 256 {
            c.numeric_value() as u8
        } else {
            todo!();
        };
        // trie_link(z+c):=trie_ref[q]; trie_char(z+c):=qi(c); trie_op(z+c):=trie_o[p];
        trie_link!(globals, z + c_u8 as trie_pointer_repr) = trie_ref!(globals, q).get();
        let trie_op = globals.trie_o[p];
        assign_trie_char_and_op!(globals, z + c_u8 as trie_pointer_repr, c, trie_op);
        // if q>0 then trie_fix(q);
        if q > 0 {
            trie_fix(globals, q);
        }
        // p:=trie_r[p];
        p = globals.trie_r[p];
        // until p=0;
        if p == 0 {
            break;
        }
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0018::ASCII_code;
use crate::section_0920::trie_pointer;
use crate::section_0920::trie_pointer_repr;
use crate::section_0921::assign_trie_char_and_op;
use crate::section_0921::trie_link;
use crate::section_0950::trie_ref;
