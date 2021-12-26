//! @ The preamble is copied directly, except that \.{\\tabskip} causes a change
//! to the tabskip glue, thereby possibly expanding macros that immediately
//! follow it. An appearance of \.{\\span} also causes such an expansion.
//!
//! Note that if the preamble contains `\.{\\global\\tabskip}', the `\.{\\global}'
//! token survives in the preamble and the `\.{\\tabskip}' defines new
//! tabskip glue (locally).
//
// @<Declare the procedure called |get_preamble_token|@>=
// procedure get_preamble_token;
pub(crate) fn get_preamble_token(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label restart;
    // begin restart: get_token;
    crate::region_backward_label!(
        'restart <-
        {
            get_token(globals)?;
            // while (cur_chr=span_code)and(cur_cmd=tab_mark) do
            while globals.cur_chr.get() as integer == span_code as integer &&
                globals.cur_cmd == tab_mark {
                // begin get_token; {this token will be expanded once}
                get_token(globals)?;
                /// this token will be expanded once
                const _ : () = ();
                // if cur_cmd>max_command then
                if globals.cur_cmd > max_command {
                    // begin expand; get_token;
                    expand(globals)?;
                    get_token(globals)?;
                    // end;
                }
                // end;
            }
            // if cur_cmd=endv then
            if globals.cur_cmd == endv {
                todo!("fatal error");
                // fatal_error("(interwoven alignment preambles are not allowed)");
            }
            // @.interwoven alignment preambles...@>
            // if (cur_cmd=assign_glue)and(cur_chr=glue_base+tab_skip_code) then
            if globals.cur_cmd == assign_glue &&
                globals.cur_chr.get() as integer == glue_base as integer + tab_skip_code as integer {
                // begin scan_optional_equals; scan_glue(glue_val);
                scan_optional_equals(globals)?;
                scan_glue(globals, small_number::new(cur_val_level_kind::glue_val as _))?;
                // if global_defs>0 then geq_define(glue_base+tab_skip_code,glue_ref,cur_val)
                if global_defs!(globals) > 0 {
                    geq_define(globals, (glue_base + tab_skip_code as word) as pointer, glue_ref, globals.cur_val as _)?;
                }
                // else eq_define(glue_base+tab_skip_code,glue_ref,cur_val);
                else {
                    eq_define(globals, (glue_base + tab_skip_code as word) as pointer, glue_ref, globals.cur_val as _)?;
                }
                // goto restart;
                crate::goto_backward_label!('restart);
                // end;
            }
        }
        |'restart|
    );
    // end;
    crate::ok_nojump!()
}

use crate::pascal::integer;
use crate::pascal::word;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::small_number;
use crate::section_0115::pointer;
use crate::section_0207::endv;
use crate::section_0207::tab_mark;
use crate::section_0209::assign_glue;
use crate::section_0209::max_command;
use crate::section_0210::glue_ref;
use crate::section_0222::glue_base;
use crate::section_0224::tab_skip_code;
use crate::section_0236::global_defs;
use crate::section_0277::eq_define;
use crate::section_0279::geq_define;
use crate::section_0365::get_token;
use crate::section_0366::expand;
use crate::section_0405::scan_optional_equals;
use crate::section_0410::cur_val_level_kind;
use crate::section_0461::scan_glue;
use crate::section_0780::span_code;
