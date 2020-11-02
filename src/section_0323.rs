//! @ Here is a procedure that starts a new level of token-list input, given
//! a token list |p| and its type |t|. If |t=macro|, the calling routine should
//! set |name| and |loc|.
//
// @d back_list(#)==begin_token_list(#,backed_up) {backs up a simple token list}
// @d ins_list(#)==begin_token_list(#,inserted) {inserts a simple token list}
//
// @p procedure begin_token_list(@!p:pointer;@!t:quarterword);
#[allow(unused_variables)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn begin_token_list(globals: &mut TeXGlobals, p: pointer, t: quarterword) {
    // begin push_input; state:=token_list; start:=p; token_type:=t;
    push_input!(globals);
    state!(globals) = token_list;
    token_type!(globals) = t;
    // if t>=macro then {the token list starts with a reference count}
    //   begin add_token_ref(p);
    //   if t=macro then param_start:=param_ptr
    //   else  begin loc:=link(p);
    //     if tracing_macros>1 then
    //       begin begin_diagnostic; print_nl("");
    //       case t of
    //       mark_text:print_esc("mark");
    //       write_text:print_esc("write");
    //       othercases print_cmd_chr(assign_toks,t-output_text+output_routine_loc)
    //       endcases;@/
    //       print("->"); token_show(p); end_diagnostic(false);
    //       end;
    //     end;
    //   end
    // else loc:=p;
    loc!(globals) = p;
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0113::quarterword;
use crate::section_0115::pointer;
use crate::section_0307::token_list;
