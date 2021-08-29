//! @ Now let's go back to the easier problem, of building the linked
//! trie.  When \.{INITEX} has scanned the `\.{\\patterns}' control
//! sequence, it calls on |new_patterns| to do the right thing.
//
// @<Declare procedures for preprocessing hyph...@>=
// procedure new_patterns; {initializes the hyphenation pattern data}
/// initializes the hyphenation pattern data
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn new_patterns(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label done, done1;
    // var k,@!l:0..64; {indices into |hc| and |hyf|;
    //                   not always in |small_number| range}
    // @!digit_sensed:boolean; {should the next digit be treated as a letter?}
    // @!v:quarterword; {trie op code}
    // @!p,@!q:trie_pointer; {nodes of trie traversed during insertion}
    // @!first_child:boolean; {is |p=trie_l[q]|?}
    // @!c:ASCII_code; {character being inserted}
    // begin if trie_not_ready then
    if globals.trie_not_ready {
        // begin set_cur_lang; scan_left_brace; {a left brace must follow \.{\\patterns}}
        set_cur_lang!(globals);
        /// a left brace must follow `\patterns`
        const _ : () = ();
        scan_left_brace(globals)?;
        // @<Enter all of the patterns into a linked trie, until coming to a right
        // brace@>;
        Enter_all_of_the_patterns_into_a_linked_trie__until_coming_to_a_right_brace!(globals);
        // end
    }
    // else begin print_err("Too late for "); print_esc("patterns");
    else {
        print_err!(globals, strpool_str!("Too late for "));
        print_esc(globals, strpool_str!("patterns"));
        // help1("All patterns must be given before typesetting begins.");
        help1!(globals, strpool_str!("All patterns must be given before typesetting begins."));
        // error; link(garbage):=scan_toks(false,false); flush_list(def_ref);
        error(globals)?;
        link!(globals, garbage) = scan_toks(globals, false, false)?;
        flush_list(globals, globals.def_ref);
        // end;
    }
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0063::print_esc;
use crate::section_0081::TeXResult;
use crate::section_0082::error;
use crate::section_0123::flush_list;
use crate::section_0162::garbage;
use crate::section_0403::scan_left_brace;
use crate::section_0473::scan_toks;
