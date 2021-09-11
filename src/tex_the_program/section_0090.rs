//! ` `

// @<Put help message on the transcript file@>=
pub(crate) macro Put_help_message_on_the_transcript_file($globals:expr) {{
    // if interaction>batch_mode then decr(selector); {avoid terminal output}
    if $globals.interaction > batch_mode {
        /// avoid terminal output
        const _: () = ();

        decr!($globals.selector);
    }
    // if use_err_help then
    if $globals.use_err_help {
        // begin print_ln; give_err_help;
        print_ln(make_globals_io_string_log_view!($globals));
        give_err_help($globals);
        // end
    }
    // else while help_ptr>0 do
    else {
        while $globals.help_ptr > 0 {
            // begin decr(help_ptr); print_nl(help_line[help_ptr]);
            decr!($globals.help_ptr);
            print_nl($globals, $globals.help_line[$globals.help_ptr.get()]);
            // end;
        }
    }
    // print_ln;
    print_ln(make_globals_io_string_log_view!($globals));
    // if interaction>batch_mode then incr(selector); {re-enable terminal output}
    if $globals.interaction > batch_mode {
        /// re-enable terminal output
        const _: () = ();
        incr!($globals.selector);
    }
    // print_ln
    print_ln(make_globals_io_string_log_view!($globals));

    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0016::decr;
    use crate::section_0016::incr;
    use crate::section_0057::print_ln;
    use crate::section_0062::print_nl;
    use crate::section_0073::batch_mode;
    use crate::section_1284::give_err_help;
}}
