//! @ Let us suppose that a linked trie has already been constructed.
//! Experience shows that we can often reduce its size by recognizing common
//! subtries; therefore another hash table is introduced for this purpose,
//! somewhat similar to |trie_op_hash|. The new hash table will be
//! initialized to zero.
//!
//! The function |trie_node(p)| returns |p| if |p| is distinct from other nodes
//! that it has seen, otherwise it returns the number of the first equivalent
//! node that it has seen.
//!
//! Notice that we might make subtries equivalent even if they correspond to
//! patterns for different languages, in which the trie ops might mean quite
//! different things. That's perfectly all right.
//
// @<Declare procedures for preprocessing hyph...@>=
// function trie_node(@!p:trie_pointer):trie_pointer; {converts
//   to a canonical form}
/// converts to a canonical form
pub(crate) fn trie_node(globals: &mut TeXGlobals, p: trie_pointer) -> trie_pointer {
    // label exit;
    // var h:trie_pointer; {trial hash location}
    /// trial hash location
    let mut h: trie_pointer;
    // @!q:trie_pointer; {trial trie node}
    /// trial trie node
    let mut q: trie_pointer;
    // begin h:=abs(trie_c[p]+1009*trie_o[p]+@|
    //     2718*trie_l[p]+3142*trie_r[p]) mod trie_size;
    h = trie_pointer::new(
        ((globals.trie_c[p].numeric_value() as integer
            + 1009 * globals.trie_o[p] as integer
            + 2718 * globals.trie_l[p].get() as integer
            + 3142 * globals.trie_r[p].get() as integer)
            % (trie_size as integer)) as _,
    );
    // loop@+  begin q:=trie_hash[h];
    loop {
        q = globals.trie_hash[h];
        // if q=0 then
        if q == 0 {
            // begin trie_hash[h]:=p; trie_node:=p; return;
            globals.trie_hash[h] = p;
            return p;
            // end;
        }
        // if (trie_c[q]=trie_c[p])and(trie_o[q]=trie_o[p])and@|
        //   (trie_l[q]=trie_l[p])and(trie_r[q]=trie_r[p]) then
        if globals.trie_c[q] == globals.trie_c[p]
            && globals.trie_o[q] == globals.trie_o[p]
            && globals.trie_l[q] == globals.trie_l[p]
            && globals.trie_r[q] == globals.trie_r[p]
        {
            // begin trie_node:=q; return;
            return q;
            // end;
        }
        // if h>0 then decr(h)@+else h:=trie_size;
        if h > 0 {
            decr!(h);
        } else {
            h = trie_size.into();
        }
        // end;
    }
    // exit:end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0011::trie_size;
use crate::section_0016::decr;
use crate::section_0920::trie_pointer;
