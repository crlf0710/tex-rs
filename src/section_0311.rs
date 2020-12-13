//! @ The status at each level is indicated by printing two lines, where the first
//! line indicates what was read so far and the second line shows what remains
//! to be read. The context is cropped, if necessary, so that the first line
//! contains at most |half_error_line| characters, and the second contains
//! at most |error_line|. Non-current input levels whose |token_type| is
//! `|backed_up|' are shown only if they have not been fully read.
//
// @p procedure show_context; {prints where the scanner is}
/// prints where the scanner is
pub(crate) fn show_context(globals: &mut TeXGlobals) {
    // label done;
    // var old_setting:0..max_selector; {saved |selector| setting}
    /// saved `selector` setting
    let mut old_setting;
    // @!nn:integer; {number of contexts shown so far, less one}
    /// number of contexts shown so far, less one
    let mut nn: integer;
    // @!bottom_line:boolean; {have we reached the final context to be shown?}
    /// have we reached the final context to be shown?
    let mut bottom_line: boolean;
    // @<Local variables for formatting calculations@>@/
    // begin base_ptr:=input_ptr; input_stack[base_ptr]:=cur_input;
    /// store current state
    const _ : () = ();
    globals.base_ptr = globals.input_ptr;
    globals.input_stack[globals.base_ptr] = globals.cur_input;
    //   {store current state}
    // nn:=-1; bottom_line:=false;
    nn = -1;
    bottom_line = false;
    region_forward_label!(
    |'done|
    {
        // loop@+begin cur_input:=input_stack[base_ptr]; {enter into the context}
        loop {
            /// enter into the context
            const _ : () = ();
            globals.cur_input = globals.input_stack[globals.base_ptr];

            // if (state<>token_list) then
            if state!(globals) != token_list {
                // if (name>17) or (base_ptr=0) then bottom_line:=true;
                if name!(globals) > 17 || globals.base_ptr == 0 {
                    bottom_line = true;
                }
            }
            // if (base_ptr=input_ptr)or bottom_line or(nn<error_context_lines) then
            if globals.base_ptr == globals.input_ptr || bottom_line ||
                nn < error_context_lines!(globals) {
                // @<Display the current context@>
                Display_the_current_context!(globals, nn, old_setting);
            }
            // else if nn=error_context_lines then
            else if nn == error_context_lines!(globals) {
                // begin print_nl("..."); incr(nn); {omitted if |error_context_lines<0|}
                /// omitted if `error_context_lines<0`
                const _ : () = ();
                print_nl(globals, strpool_str!("..."));
                incr!(nn);
                // end;
            }
            // if bottom_line then goto done;
            if bottom_line {
                goto_forward_label!('done);
            }
            // decr(base_ptr);
            decr!(globals.base_ptr);
            // end;
        }
    }
    // done: cur_input:=input_stack[input_ptr]; {restore original state}
    'done <-
    );
    /// restore original state
    const _ : () = ();
    globals.cur_input = globals.input_stack[globals.input_ptr];
    // end;
}

use crate::pascal::{boolean, integer};
use crate::section_0004::TeXGlobals;
use crate::section_0062::print_nl;
use crate::section_0307::token_list;
