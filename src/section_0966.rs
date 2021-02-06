//! @ Finally we put everything together: Here is how the trie gets to its
//! final, efficient form.
//! The following packing routine is rigged so that the root of the linked
//! tree gets mapped into location 1 of |trie|, as required by the hyphenation
//! algorithm. This happens because the first call of |first_fit| will
//! ``take'' location~1.
//
// @<Declare procedures for preprocessing hyphenation patterns@>=
// procedure init_trie;
pub(crate) fn init_trie(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var @!p:trie_pointer; {pointer for initialization}
    // @!j,@!k,@!t:integer; {all-purpose registers for initialization}
    // @!r,@!s:trie_pointer; {used to clean up the packed |trie|}
    // @!h:two_halves; {template used to zero out |trie|'s holes}
    // begin @<Get ready to compress the trie@>;
    // if trie_root<>0 then
    if trie_root!(globals) != 0 {
        // begin first_fit(trie_root); trie_pack(trie_root);
        first_fit(globals, trie_root!(globals))?;
        trie_pack(globals, trie_root!(globals));
        // end;
    }
    // @<Move the data into |trie|@>;
    // trie_not_ready:=false;
    globals.trie_not_ready = false;
    // end;
    todo!();
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0953::first_fit;
use crate::section_0957::trie_pack;
