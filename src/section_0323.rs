//! @ Here is a procedure that starts a new level of token-list input, given
//! a token list |p| and its type |t|. If |t=macro|, the calling routine should
//! set |name| and |loc|.
//
// @d back_list(#)==begin_token_list(#,backed_up) {backs up a simple token list}
/// backs up a simple token list
macro_rules! back_list {
    ($globals:expr, $val:expr) => {
        crate::section_0323::begin_token_list($globals, $val, crate::section_0307::backed_up)
    };
}
// @d ins_list(#)==begin_token_list(#,inserted) {inserts a simple token list}
/// inserts a simple token list
macro_rules! ins_list {
    ($globals:expr, $val:expr) => {
        crate::section_0323::begin_token_list($globals, $val, crate::section_0307::inserted)
    };
}

// @p procedure begin_token_list(@!p:pointer;@!t:quarterword);
#[allow(unused_variables)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn begin_token_list(globals: &mut TeXGlobals, p: pointer, t: quarterword) {
    // begin push_input; state:=token_list; start:=p; token_type:=t;
    push_input!(globals);
    state!(globals) = token_list;
    token_type!(globals) = t;
    // if t>=macro then {the token list starts with a reference count}
    if t >= r#macro {
        /// the token list starts with a reference count
        const _: () = ();
        // begin add_token_ref(p);
        add_token_ref!(globals, p);
        // if t=macro then param_start:=param_ptr
        if t == r#macro {
            param_start!(globals) = globals.param_ptr.get() as _;
        }
        // else  begin loc:=link(p);
        else {
            loc!(globals) = link!(globals, p);
            // if tracing_macros>1 then
            if tracing_macros!(globals) > 1 {
                // begin begin_diagnostic; print_nl("");
                begin_diagnostic(globals);
                // case t of
                // mark_text:print_esc("mark");
                if t == mark_text {
                    print_esc(globals, strpool_str!("mark"));
                }
                // write_text:print_esc("write");
                else if t == write_text {
                    print_esc(globals, strpool_str!("write"));
                }
                // othercases print_cmd_chr(assign_toks,t-output_text+output_routine_loc)
                else {
                    print_cmd_chr(
                        globals,
                        assign_toks,
                        chr_code_type::new(
                            t as chr_code_repr - output_text as chr_code_repr
                                + output_routine_loc as chr_code_repr,
                        ),
                    );
                }
                // endcases;@/
                // print("->"); token_show(p); end_diagnostic(false);
                print(globals, strpool_str!("->").get() as _);
                token_show(globals, p);
                end_diagnostic(globals, false);
                // end;
            }
            // end;
        }
    // end
    }
    // else loc:=p;
    else {
        loc!(globals) = p;
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0059::print;
use crate::section_0063::print_esc;
use crate::section_0113::quarterword;
use crate::section_0115::pointer;
use crate::section_0209::assign_toks;
use crate::section_0230::output_routine_loc;
use crate::section_0245::begin_diagnostic;
use crate::section_0245::end_diagnostic;
use crate::section_0295::token_show;
use crate::section_0297::chr_code_type;
use crate::section_0297::chr_code_repr;
use crate::section_0298::print_cmd_chr;
use crate::section_0307::mark_text;
use crate::section_0307::output_text;
use crate::section_0307::r#macro;
use crate::section_0307::token_list;
use crate::section_0307::write_text;
