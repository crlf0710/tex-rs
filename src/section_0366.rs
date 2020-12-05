//! @* \[25] Expanding the next token.
//! Only a dozen or so command codes |>max_command| can possibly be returned by
//! |get_next|; in increasing order, they are |undefined_cs|, |expand_after|,
//! |no_expand|, |input|, |if_test|, |fi_or_else|, |cs_name|, |convert|, |the|,
//! |top_bot_mark|, |call|, |long_call|, |outer_call|, |long_outer_call|, and
//! |end_template|.{\emergencystretch=40pt\par}
//!
//! The |expand| subroutine is used when |cur_cmd>max_command|. It removes a
//! ``call'' or a conditional or one of the other special operations just
//! listed.  It follows that |expand| might invoke itself recursively. In all
//! cases, |expand| destroys the current token, but it sets things up so that
//! the next |get_next| will deliver the appropriate next token. The value of
//! |cur_tok| need not be known when |expand| is called.
//!
//! Since several of the basic scanning routines communicate via global variables,
//! their values are saved as local variables of |expand| so that
//! recursive calls don't invalidate them.
//! @^recursion@>
//
// @p@t\4@>@<Declare the procedure called |macro_call|@>@;@/
// @t\4@>@<Declare the procedure called |insert_relax|@>@;@/
// procedure@?pass_text; forward;@t\2@>
// procedure@?start_input; forward;@t\2@>
// procedure@?conditional; forward;@t\2@>
// procedure@?get_x_token; forward;@t\2@>
// procedure@?conv_toks; forward;@t\2@>
// procedure@?ins_the_toks; forward;@t\2@>
const _: () = ();

// procedure expand;
#[allow(unused_variables)]
pub(crate) fn expand(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var t:halfword; {token that is being ``expanded after''}
    // @!p,@!q,@!r:pointer; {for list manipulation}
    // @!j:0..buf_size; {index into |buffer|}
    // @!cv_backup:integer; {to save the global quantity |cur_val|}
    // @!cvl_backup,@!radix_backup,@!co_backup:small_number;
    //   {to save |cur_val_level|, etc.}
    // @!backup_backup:pointer; {to save |link(backup_head)|}
    // @!save_scanner_status:small_number; {temporary storage of |scanner_status|}
    // begin cv_backup:=cur_val; cvl_backup:=cur_val_level; radix_backup:=radix;
    // co_backup:=cur_order; backup_backup:=link(backup_head);
    // if cur_cmd<call then @<Expand a nonmacro@>
    if globals.cur_cmd < call {
        Expand_a_nonmacro!(globals);
    }
    // else if cur_cmd<end_template then macro_call
    else if globals.cur_cmd < end_template {
        macro_call(globals)?;
    }
    // else @<Insert a token containing |frozen_endv|@>;
    else {
        todo!();
    }
    // cur_val:=cv_backup; cur_val_level:=cvl_backup; radix:=radix_backup;
    // cur_order:=co_backup; link(backup_head):=backup_backup;
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0210::call;
use crate::section_0210::end_template;
use crate::section_0389::macro_call;
