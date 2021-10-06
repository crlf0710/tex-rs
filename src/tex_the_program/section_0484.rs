//! ` `
//!
//! Here we input on-line into the |buffer| array, prompting the user explicitly
//! if |n>=0|.  The value of |n| is set negative so that additional prompts
//! will not be given in the case of multi-line input.
//
// @<Input for \.{\\read} from the terminal@>=
pub(crate) macro Input_for_read_from_the_terminal($globals:expr, $n:expr, $r:expr) {{
    // if interaction>nonstop_mode then
    if $globals.interaction > nonstop_mode {
        // if n<0 then prompt_input("")
        if $n < 0 {
            prompt_input!($globals, crate::strpool_str!(""));
        }
        // else  begin wake_up_terminal;
        else {
            wake_up_terminal($globals);
            // print_ln; sprint_cs(r); prompt_input("="); n:=-1;
            print_ln(make_globals_io_string_log_view!($globals));
            sprint_cs($globals, $r);
            prompt_input!($globals, crate::strpool_str!("="));
            $n = -1;
            // end
        }
    }
    // else fatal_error("*** (cannot \read from terminal in nonstop modes)")
    else {
        fatal_error(
            $globals,
            crate::strpool_str!("*** (cannot \\read from terminal in nonstop modes)"),
        )?;
    }
    // @.cannot \\read@>
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0034::wake_up_terminal;
    use crate::section_0057::print_ln;
    use crate::section_0071::prompt_input;
    use crate::section_0073::nonstop_mode;
    use crate::section_0093::fatal_error;
    use crate::section_0263::sprint_cs;
}}
