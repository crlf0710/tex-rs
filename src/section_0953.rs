//! @ The |first_fit| procedure finds the smallest hole |z| in |trie| such that
//! a trie family starting at a given node |p| will fit into vacant positions
//! starting at |z|. If |c=trie_c[p]|, this means that location |z-c| must
//! not already be taken by some other family, and that |z-c+@t$c^\prime$@>|
//! must be vacant for all characters $c^\prime$ in the family. The procedure
//! sets |trie_ref[p]| to |z-c| when the first fit has been found.
//
// @<Declare procedures for preprocessing hyph...@>=
// procedure first_fit(@!p:trie_pointer); {packs a family into |trie|}
/// packs a family into `trie`
#[allow(unused_variables, unused_assignments)]
pub(crate) fn first_fit(globals: &mut TeXGlobals, p: trie_pointer) -> TeXResult<()> {
    // label not_found,found;
    // var h:trie_pointer; {candidate for |trie_ref[p]|}
    /// candidate for `trie_ref[p]`
    let h: trie_pointer;
    // @!z:trie_pointer; {runs through holes}
    /// runs through holes
    let z: trie_pointer;
    // @!q:trie_pointer; {runs through the family starting at |p|}
    // @!c:ASCII_code; {smallest character in the family}
    /// smallest character in the family
    let c: ASCII_code;
    // @!l,@!r:trie_pointer; {left and right neighbors}
    // @!ll:1..256; {upper limit of |trie_min| updating}
    // begin c:=so(trie_c[p]);
    c = globals.trie_c[p];
    // z:=trie_min[c]; {get the first conceivably good hole}
    /// get the first conceivably good hole
    const _ : () = ();
    let c_u8 = if c.numeric_value() < 256 {
        c.numeric_value() as u8
    } else {
        todo!();
    };
    z = globals.trie_min[c_u8 as usize];
    // loop@+  begin h:=z-c;@/
    loop {
        h = trie_pointer::new((z.get() as integer - c_u8 as integer) as _);
        // @<Ensure that |trie_max>=h+256|@>;
        Ensure_that_trie_max_ge_h_plus_256!(globals, h);
        todo!();
        // if trie_taken[h] then goto not_found;
        // @<If all characters of the family fit relative to |h|, then
        //   |goto found|,\30\ otherwise |goto not_found|@>;
        // not_found: z:=trie_link(z); {move to the next hole}
        // end;
    }
    // found: @<Pack the family into |trie| relative to |h|@>;
    // end;
    todo!();
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0018::ASCII_code;
use crate::section_0081::TeXResult;
use crate::section_0920::trie_pointer;
