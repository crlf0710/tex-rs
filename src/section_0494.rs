//! @ Here is a procedure that ignores text until coming to an \.{\\or},
//! \.{\\else}, or \.{\\fi} at level zero of $\.{\\if}\ldots\.{\\fi}$
//! nesting. After it has acted, |cur_chr| will indicate the token that
//! was found, but |cur_tok| will not be set (because this makes the
//! procedure run faster).
//
// @p procedure pass_text;
#[allow(unused_variables)]
pub(crate) fn pass_text(globals: &mut TeXGlobals) -> Result<(), JumpOutToEndOfTEX> {
    // label done;
    // var l:integer; {level of $\.{\\if}\ldots\.{\\fi}$ nesting}
    /// level of `\if`...`\fi` nesting
    let mut l: integer;
    // @!save_scanner_status:small_number; {|scanner_status| upon entry}
    /// `scanner_status` upon entry
    let save_scanner_status : scanner_status_kind; 
    // begin save_scanner_status:=scanner_status; scanner_status:=skipping; l:=0;
    save_scanner_status = globals.scanner_status;
    globals.scanner_status = scanner_status_kind::skipping;
    l = 0;
    // skip_line:=line;
    globals.skip_line = globals.line;
    region_forward_label!(
    |'done|
    {
        // loop@+  begin get_next;
        loop {
            get_next(globals)?;
            // if cur_cmd=fi_or_else then
            if globals.cur_cmd == fi_or_else {
                // begin if l=0 then goto done;
                if l == 0 {
                    goto_forward_label!('done);
                }
                // if cur_chr=fi_code then decr(l);
                if globals.cur_chr.get() == fi_code as _ {
                    decr!(l);
                }
                // end
            }
            // else if cur_cmd=if_test then incr(l);
            else if globals.cur_cmd == if_test {
                incr!(l);
            }
            // end;
        }
    }
    // done: scanner_status:=save_scanner_status;
    'done <-
    );
    globals.scanner_status = save_scanner_status;
    // end;
    ok_nojump!()
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0081::JumpOutToEndOfTEX;
use crate::section_0101::small_number;
use crate::section_0210::if_test;
use crate::section_0210::fi_or_else;
use crate::section_0341::get_next;
use crate::section_0305::scanner_status_kind;
use crate::section_0489::fi_code;
