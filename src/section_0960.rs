//! @ Now let's go back to the easier problem, of building the linked
//! trie.  When \.{INITEX} has scanned the `\.{\\patterns}' control
//! sequence, it calls on |new_patterns| to do the right thing.
//
// @<Declare procedures for preprocessing hyph...@>=
// procedure new_patterns; {initializes the hyphenation pattern data}
/// initializes the hyphenation pattern data
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
        todo!("error");
        // help1("All patterns must be given before typesetting begins.");
        // error; link(garbage):=scan_toks(false,false); flush_list(def_ref);
        // end;
    }
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0403::scan_left_brace;