//! ` `

// @<Declare act...@>=
// procedure show_whatever;
pub(crate) fn show_whatever(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label common_ending;
    // var p:pointer; {tail of a token list to show}
    region_forward_label!(
    |'common_ending|
    {
    // begin case cur_chr of
    // show_lists: begin begin_diagnostic; show_activities;
    if globals.cur_chr.get() == show_lists as chr_code_repr {
        begin_diagnostic(globals);
        show_activities(globals);
        // end;
    }
    // show_box_code: @<Show the current contents of a box@>;
    else if globals.cur_chr.get() == show_box_code as chr_code_repr {
        todo!();
    }
    // show_code: @<Show the current meaning of a token, then |goto common_ending|@>;
    else if globals.cur_chr.get() == show_code as chr_code_repr {
        Show_the_current_meaning_of_a_token__then_goto_common_ending!(globals, 'common_ending);
    }
    // othercases @<Show the current value of some parameter or register,
    //   then |goto common_ending|@>
    else {
        Show_the_current_value_of_some_parameter_or_register__then_goto_common_ending!(globals, 'common_ending);
    }
    // endcases;@/
    // @<Complete a potentially long \.{\\show} command@>;
    Complete_a_potentially_long_show_command!(globals);
    }
    // common_ending: if interaction<error_stop_mode then
    'common_ending <-
    );
    if globals.interaction < error_stop_mode {
        // begin help0; decr(error_count);
        help0!(globals);
        decr!(globals.error_count);
    // end
    }
    // else if tracing_online>0 then
    else if tracing_online!(globals) > 0 {
        // begin@t@>@;@/
        // help3("This isn't an error message; I'm just \showing something.")@/
        // ("Type `I\show...' to show more (e.g., \show\cs,")@/
        // ("\showthe\count10, \showbox255, \showlists).");
        // end
    }
    // else  begin@t@>@;@/
    else {
        // help5("This isn't an error message; I'm just \showing something.")@/
        // ("Type `I\show...' to show more (e.g., \show\cs,")@/
        // ("\showthe\count10, \showbox255, \showlists).")@/
        // ("And type `I\tracingonline=1\show...' to show boxes and")@/
        // ("lists on your terminal as well as in the transcript file.");
        // end;
    }
    // error;
    error(globals)?;
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0073::error_stop_mode;
use crate::section_0081::TeXResult;
use crate::section_0082::error;
use crate::section_0218::show_activities;
use crate::section_0245::begin_diagnostic;
use crate::section_0297::chr_code_repr;
use crate::section_1291::show_kind::*;
