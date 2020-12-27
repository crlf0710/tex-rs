//! @ We have now completed the hyphenation routine, so the |line_break| procedure
//! is finished at last. Since the hyphenation exception table is fresh in our
//! minds, it's a good time to deal with the routine that adds new entries to it.
//!
//! When \TeX\ has scanned `\.{\\hyphenation}', it calls on a procedure named
//! |new_hyph_exceptions| to do the right thing.
//
// @d set_cur_lang==if language<=0 then cur_lang:=0
macro_rules! set_cur_lang {
    ($globals:expr) => {{
        if language!($globals) <= 0 {
            $globals.cur_lang = ASCII_code_literal!(0);
        }
        // else if language>255 then cur_lang:=0
        else if language!($globals) > 255 {
            $globals.cur_lang = ASCII_code_literal!(0);
        }
        // else cur_lang:=language
        else {
            $globals.cur_lang = ASCII_code::from(language!($globals));
        }
        use crate::section_0018::ASCII_code;
    }};
}

// @p procedure new_hyph_exceptions; {enters new exceptions}
/// enters new exceptions
pub(crate) fn new_hyph_exceptions(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label reswitch, exit, found, not_found;
    // var n:0..64; {length of current word; not always a |small_number|}
    // @!j:0..64; {an index into |hc|}
    // @!h:hyph_pointer; {an index into |hyph_word| and |hyph_list|}
    // @!k:str_number; {an index into |str_start|}
    // @!p:pointer; {head of a list of hyphen positions}
    // @!q:pointer; {used when creating a new node for list |p|}
    // @!s,@!t:str_number; {strings being compared or stored}
    // @!u,@!v:pool_pointer; {indices into |str_pool|}
    // begin scan_left_brace; {a left brace must follow \.{\\hyphenation}}
    /// a left brace must follow `\hyphenation`
    const _: () = ();

    scan_left_brace(globals)?;
    // set_cur_lang;
    set_cur_lang!(globals);
    // @<Enter as many hyphenation exceptions as are listed,
    // until coming to a right brace; then |return|@>;
    Enter_as_many_hyphenation_exceptions_as_are_listed__until_coming_to_a_right_brace__then_return!(
        globals
    );
    // exit:end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0403::scan_left_brace;
