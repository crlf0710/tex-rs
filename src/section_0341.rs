//! @ Now we're ready to take the plunge into |get_next| itself. Parts of
//! this routine are executed more often than any other instructions of \TeX.
//! @^mastication@>@^inner loop@>

// @d switch=25 {a label in |get_next|}
// @d start_cs=26 {another}

// @p procedure get_next; {sets |cur_cmd|, |cur_chr|, |cur_cs| to next token}
/// sets `cur_cmd`, `cur_chr`, `cur_cs` to next token
#[allow(unused_variables)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip(globals)))]
pub(crate) fn get_next(globals: &mut TeXGlobals) -> Result<(), JumpOutToEndOfTEX> {
    region_forward_label! {
    |'exit|
    {
    // label restart, {go here to get the next input token}
    //   switch, {go here to eat the next character from a file}
    //   reswitch, {go here to digest it again}
    //   start_cs, {go here to start looking for a control sequence}
    //   found, {go here when a control sequence has been found}
    //   exit; {go here when the next input token has been got}
    // var k:0..buf_size; {an index into |buffer|}
    // @!t:halfword; {a token}
    // @!cat:0..max_char_code; {|cat_code(cur_chr)|, usually}
    // @!c,@!cc:ASCII_code; {constituents of a possible expanded code}
    // @!d:2..3; {number of excess characters in an expanded code}
    const _: () = ();
    // begin restart: cur_cs:=0;
    region_backward_label! {
    'restart <-
    {
    globals.cur_cs = 0;
    trace_expr!("state = {:?}", state!(globals));
    // if state<>token_list then
    if state!(globals) != token_list {
        // @<Input from external file, |goto restart| if no input found@>
        Input_from_external_file__goto_restart_if_no_input_found!(globals);
    } else {
        // else @<Input from token list, |goto restart| if end of list or
        //   if a parameter needs to be expanded@>;
        Input_from_token_list__goto_restart_if_end_of_list_or_if_a_parameter_needs_to_be_expanded!(
            globals,
            'restart
        );
    }
    // @<If an alignment entry has just ended, take appropriate action@>;
    If_an_alignment_entry_has_just_ended_take_appropriate_action!(globals, 'restart);
    }
    |'restart|
    }
    }
    // exit:end;
    'exit <-
    }
    return_nojump!();
}

use crate::section_0004::TeXGlobals;
use crate::section_0307::token_list;
use crate::section_0081::JumpOutToEndOfTEX;
